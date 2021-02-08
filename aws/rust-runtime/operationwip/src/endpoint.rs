/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use core::convert::AsRef;
use std::str::FromStr;

use http::uri::Uri;

use http::header::HOST;
use smithy_http::operation;
use smithy_http::property_bag::PropertyBag;
use std::sync::Arc;
use smithy_http::middleware::MapRequest;

pub struct StaticEndpoint(http::Uri);

impl StaticEndpoint {
    pub fn uri(&self) -> &Uri {
        &self.0
    }
    pub fn from_service_region(svc: impl AsRef<str>, region: impl AsRef<str>) -> Self {
        StaticEndpoint(
            Uri::from_str(&format!(
                "https://{}.{}.amazonaws.com",
                svc.as_ref(),
                region.as_ref()
            ))
            .unwrap(),
        )
    }

    pub fn from_uri(uri: Uri) -> Self {
        StaticEndpoint(uri)
    }

    pub fn apply(&self, base_uri: &Uri) -> Uri {
        let parts = self.0.clone().into_parts();

        Uri::builder()
            .authority(parts.authority.expect("base uri must have an authority"))
            .scheme(parts.scheme.expect("base uri must have scheme"))
            .path_and_query(base_uri.path_and_query().unwrap().clone())
            .build()
            .expect("valid uri")
    }
}

pub trait ProvideEndpoint: Send + Sync {
    fn set_endpoint(&self, request_uri: &mut Uri);
}

impl ProvideEndpoint for StaticEndpoint {
    fn set_endpoint(&self, request_uri: &mut Uri) {
        let new_uri = self.apply(request_uri);
        *request_uri = new_uri;
    }
}

/*
impl<T> OperationMiddleware for T
where
    T: ProvideEndpoint,
{
    fn apply(&self, mut request: operation::Request) -> Result<operation::Request, BoxError> {
        request.augment(|request, config| {
            self.set_endpoint(&mut request.);
        })
    }
}*/

pub trait EndpointProviderExt {
    fn endpoint_provider(&self) -> Option<&Arc<dyn ProvideEndpoint>>;
    fn insert_endpoint_provider(
        &mut self,
        provider: Arc<dyn ProvideEndpoint>,
    ) -> Option<Arc<dyn ProvideEndpoint>>;
}

impl EndpointProviderExt for PropertyBag {
    fn endpoint_provider(&self) -> Option<&Arc<dyn ProvideEndpoint>> {
        self.get()
    }

    fn insert_endpoint_provider(
        &mut self,
        provider: Arc<dyn ProvideEndpoint>,
    ) -> Option<Arc<dyn ProvideEndpoint>> {
        self.insert(provider)
    }
}

// TODO: this should probably move to a collection of middlewares
#[derive(Clone, Copy)]
/// Set the endpoint for a request based on the endpoint config
pub struct AddEndpointStage;
impl MapRequest for AddEndpointStage {
    type Error = String;
    fn apply(&self, request: operation::Request) -> Result<operation::Request, Self::Error> {
        request.augment(|mut request, extensions| {
            let endpoint_provider: &Arc<dyn ProvideEndpoint> = extensions
                .endpoint_provider()
                .ok_or("missing endpoint provider")?;
            endpoint_provider.set_endpoint(&mut request.uri_mut());
            let uri = request.uri().host().unwrap().to_string();
            request.headers_mut().append(
                HOST,
                uri.parse().expect("host should be valid header value"),
            );
            Ok(request)
        })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use http::Uri;

    use crate::endpoint::StaticEndpoint;

    #[test]
    fn endpoint_from_svc() {
        let endpoint = StaticEndpoint::from_service_region("dynamodb", "us-west-2");
        assert_eq!(
            endpoint.uri().to_string(),
            "https://dynamodb.us-west-2.amazonaws.com/"
        );
    }

    #[test]
    fn properly_update_uri() {
        let uri = Uri::builder()
            .path_and_query("/get?k=123&v=456")
            .build()
            .unwrap();
        let endpoint = StaticEndpoint::from_uri(Uri::from_str("http://localhost:8080/").unwrap());
        assert_eq!(
            endpoint.apply(&uri).to_string(),
            "http://localhost:8080/get?k=123&v=456"
        );
    }
}
