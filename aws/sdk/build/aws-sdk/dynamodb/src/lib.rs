//! <fullname>Amazon DynamoDB</fullname>
//! <p>Amazon DynamoDB is a fully managed NoSQL database service that provides fast and
//! predictable performance with seamless scalability. DynamoDB lets you offload the
//! administrative burdens of operating and scaling a distributed database, so that you don't have
//! to worry about hardware provisioning, setup and configuration, replication, software patching,
//! or cluster scaling.</p>
//! <p>With DynamoDB, you can create database tables that can store and retrieve any amount of
//! data, and serve any level of request traffic. You can scale up or scale down your tables'
//! throughput capacity without downtime or performance degradation, and use the AWS Management
//! Console to monitor resource utilization and performance metrics.</p>
//! <p>DynamoDB automatically spreads the data and traffic for your tables over a sufficient
//! number of servers to handle your throughput and storage requirements, while maintaining
//! consistent and fast performance. All of your data is stored on solid state disks (SSDs) and
//! automatically replicated across multiple Availability Zones in an AWS region, providing
//! built-in high availability and data durability. </p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

pub use config::Config;

mod aws_json_errors;
mod blob_serde;
pub mod config;
pub mod error;
mod idempotency_token;
pub mod input;
mod instant_epoch;
pub mod model;
pub mod operation;
pub mod output;
mod serde_util;
mod serializer;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use ::smithy_types::Blob;
static API_METADATA: ::aws_http::user_agent::ApiMetadata =
    ::aws_http::user_agent::ApiMetadata::new("dynamodb", PKG_VERSION);
pub use ::aws_auth::Credentials;
pub use ::aws_types::region::Region;
pub use ::smithy_http::endpoint::Endpoint;
