/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{Layer, Service};
type BoxError = Box<dyn Error + Send + Sync>;
use pin_project::pin_project;
use smithy_http::body::SdkBody;
use smithy_http::operation;

pub trait RequestStage {
    type Error: Into<BoxError>;
    fn apply(&self, request: operation::Request) -> Result<operation::Request, Self::Error>;
}

#[derive(Clone)]
pub struct OperationRequestMiddlewareService<S, M> {
    inner: S,
    middleware: M,
}

pub struct OperationPipelineService<M> {
    layer: M,
}

impl<M: Clone> OperationPipelineService<M> {
    pub fn for_stage(layer: M) -> Self {
        OperationPipelineService { layer }
    }
}

impl<S, M> Layer<S> for OperationPipelineService<M>
where
    M: Clone,
{
    type Service = OperationRequestMiddlewareService<S, M>;

    fn layer(&self, inner: S) -> Self::Service {
        OperationRequestMiddlewareService {
            inner,
            middleware: self.layer.clone(),
        }
    }
}

pub trait RequestConstructionErr {
    fn request_error(err: BoxError) -> Self;
}

#[pin_project(project = EnumProj)]
pub enum OperationMiddlewareFuture<F, E> {
    Inner(#[pin] F),
    Ready(Option<E>),
}

impl<O, F, E> Future for OperationMiddlewareFuture<F, E>
where
    F: Future<Output = Result<O, E>>,
{
    type Output = Result<O, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project() {
            EnumProj::Ready(e) => Poll::Ready(Err(e.take().unwrap())),
            EnumProj::Inner(f) => f.poll(cx),
        }
    }
}

impl<S, M> Service<operation::Request> for OperationRequestMiddlewareService<S, M>
where
    S: Service<operation::Request>,
    M: RequestStage,
    S::Error: RequestConstructionErr,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = OperationMiddlewareFuture<S::Future, S::Error>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: operation::Request) -> Self::Future {
        match self
            .middleware
            .apply(req)
            .map_err(|e| S::Error::request_error(e.into()))
        {
            Err(e) => OperationMiddlewareFuture::Ready(Some(e)),
            Ok(req) => OperationMiddlewareFuture::Inner(self.inner.call(req)),
        }
    }
}

/// Connects Operation driven middleware to an HTTP implementation.
///
/// It will also wrap the error type in OperationError to enable operation middleware
/// reporting specific errors
#[derive(Clone)]
pub struct DispatchMiddleware<S> {
    inner: S,
}

pub fn to_request<E>(
    request: operation::Request,
) -> Result<http::Request<SdkBody>, OperationError<E>> {
    Ok(request.into_parts().0)
}

#[derive(Clone, Copy)]
pub struct DispatchLayer;

impl<S> Layer<S> for DispatchLayer
where
    S: Service<http::Request<SdkBody>>,
{
    type Service = DispatchMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        DispatchMiddleware { inner }
    }
}

#[derive(Debug)]
pub enum OperationError<E> {
    DispatchError(E),
    ConstructionError(Box<dyn Error + Send + Sync + 'static>),
}

impl<E> RequestConstructionErr for OperationError<E> {
    fn request_error(err: Box<dyn Error + Send + Sync + 'static>) -> Self {
        OperationError::ConstructionError(err)
    }
}

#[pin_project]
pub struct OperationFuture<F> {
    #[pin]
    f: F,
}

impl<F, T, E> Future for OperationFuture<F>
where
    F: Future<Output = Result<T, E>>,
{
    type Output = Result<T, OperationError<E>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        this.f.poll(cx).map_err(OperationError::DispatchError)
    }
}

impl<S> Service<operation::Request> for DispatchMiddleware<S>
where
    S: Service<http::Request<SdkBody>>,
{
    type Response = S::Response;
    type Error = OperationError<S::Error>;
    type Future = OperationFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner
            .poll_ready(cx)
            .map_err(OperationError::DispatchError)
    }

    fn call(&mut self, req: operation::Request) -> Self::Future {
        let (req, _propery_bag) = req.into_parts();
        OperationFuture {
            f: self.inner.call(req),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::middleware::{DispatchLayer, OperationPipelineService, RequestStage};
    use bytes::Bytes;
    use http::header::HeaderName;
    use http::{HeaderValue, Request, Response};
    use std::str::FromStr;

    use smithy_http::body::SdkBody;
    use smithy_http::operation;
    use smithy_http::response::ParseHttpResponse;
    use std::convert::Infallible;
    use tower::service_fn;
    use tower::{Layer, Service};

    struct TestOperationParser;

    impl<B> ParseHttpResponse<B> for TestOperationParser
    where
        B: http_body::Body,
    {
        type Output = String;

        fn parse_unloaded(&self, _response: &mut Response<B>) -> Option<Self::Output> {
            Some("Hello!".to_string())
        }

        fn parse_loaded(&self, _response: &Response<Bytes>) -> Self::Output {
            "Hello!".to_string()
        }
    }

    #[tokio::test]
    async fn middleware_test() {
        #[derive(Clone)]
        struct AddHeader(String, String);
        impl RequestStage for AddHeader {
            type Error = Infallible;
            fn apply(
                &self,
                request: operation::Request,
            ) -> Result<operation::Request, Self::Error> {
                request.augment(|mut request, _| {
                    request.headers_mut().append(
                        HeaderName::from_str(&self.0).unwrap(),
                        HeaderValue::from_str(&self.0).unwrap(),
                    );
                    Ok(request)
                })
            }
        }

        let add_header = OperationPipelineService::for_stage(AddHeader(
            "X-Key".to_string(),
            "X-Value".to_string(),
        ));
        let http_service = service_fn(|request: Request<SdkBody>| async move {
            if request.uri().to_string().as_str() == "123" {
                Err("invalid url")
            } else {
                Ok(http::Response::new(
                    request
                        .headers()
                        .iter()
                        .map(|(k, v)| format!("{}:{:?}", k, v))
                        .collect::<String>(),
                ))
            }
        });
        let mut service = add_header.layer(DispatchLayer.layer(http_service));
        let operation = operation::Request::new(
            Request::builder()
                .uri("/some_url")
                .body(SdkBody::from("Hello"))
                .unwrap(),
        ); /*,
               signing_config: SigningConfig::Http(HttpSigningConfig {
                   algorithm: SigningAlgorithm::SigV4,
                   signature_type: HttpSignatureType::HttpRequestHeaders,
                   service_config: ServiceConfig {
                       service: "svc".to_string(),
                       region: "region".to_string(),
                   },
                   request_config: RequestConfig {
                       request_ts: || SystemTime::now(),
                   },
                   double_uri_encode: false,
                   normalize_uri_path: false,
                   omit_session_token: false,
               }),
               credentials_provider: Box::new(Credentials::from_static("key", "secret", None)),
               endpoint_config: Box::new(StaticEndpoint::from_service_region("dynamodb", "us-east-1")),
           };*/
        let response = service.call(operation).await;
        assert_eq!(response.unwrap().body(), "x-key:\"X-Key\"");
    }
}
