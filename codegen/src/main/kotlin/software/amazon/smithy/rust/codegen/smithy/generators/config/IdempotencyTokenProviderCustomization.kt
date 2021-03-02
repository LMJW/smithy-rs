/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

package software.amazon.smithy.rust.codegen.smithy.generators.config

import software.amazon.smithy.rust.codegen.rustlang.Writable
import software.amazon.smithy.rust.codegen.rustlang.rust
import software.amazon.smithy.rust.codegen.rustlang.writable
import software.amazon.smithy.rust.codegen.smithy.RuntimeType
import software.amazon.smithy.rust.codegen.smithy.customize.NamedSectionGenerator

/**
 * Add a `make_token` field to Service config. See below for the resulting generated code.
 */
class IdempotencyTokenProviderCustomization : NamedSectionGenerator<ServiceConfig>() {
    override fun section(section: ServiceConfig): Writable {
        return when (section) {
            is ServiceConfig.ConfigStruct -> writable {
                rust("pub (crate) make_token: Box<dyn #T::MakeIdempotencyToken>,", RuntimeType.IdempotencyToken)
            }
            ServiceConfig.ConfigImpl -> emptySection
            ServiceConfig.BuilderStruct -> writable {
                rust("make_token: Option<Box<dyn #T::MakeIdempotencyToken>>,", RuntimeType.IdempotencyToken)
            }
            ServiceConfig.BuilderImpl -> writable {
                rust(
                    """
            pub fn make_token(mut self, make_token: impl #T::MakeIdempotencyToken + 'static) -> Self {
                self.make_token = Some(Box::new(make_token));
                self
            }
            """,
                    RuntimeType.IdempotencyToken
                )
            }
            ServiceConfig.BuilderBuild -> writable {
                rust("make_token: self.make_token.unwrap_or_else(|| Box::new(#T::default_provider())),", RuntimeType.IdempotencyToken)
            }
        }
    }
}

/* Generated Code
pub struct Config {
    pub(crate) make_token: Box<dyn crate::idempotency_token::MakeIdempotencyToken>,
}
impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}
#[derive(Default)]
pub struct ConfigBuilder {
    #[allow(dead_code)]
    make_token: Option<Box<dyn crate::idempotency_token::MakeIdempotencyToken>>,
}
impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn make_token(
        mut self,
        make_token: impl crate::idempotency_token::MakeIdempotencyToken + 'static,
    ) -> Self {
        self.make_token = Some(Box::new(make_token));
        self
    }

    pub fn build(self) -> Config {
        Config {
            make_token: self
            .make_token
            .unwrap_or_else(|| Box::new(crate::idempotency_token::default_provider())),
        }
    }
}
 */
