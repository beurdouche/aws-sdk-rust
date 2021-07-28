#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
//! <p>Amazon Connect is a cloud-based contact center solution that you use to set up and manage a customer
//! contact center and provide reliable customer engagement at any scale.</p>
//! <p>Amazon Connect provides metrics and real-time reporting that enable you to optimize contact routing.
//! You can also resolve customer issues more efficiently by getting customers in touch with the
//! appropriate agents.</p>
//! <p>There are limits to the number of Amazon Connect resources that you can create. There are also limits
//! to the number of requests that you can make per second. For more information, see <a href="https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-service-limits.html">Amazon Connect
//! Service Quotas</a> in the <i>Amazon Connect Administrator Guide</i>.</p>
//! <p>You can
//! connect
//! programmatically to an AWS service by using an endpoint. For a list of Amazon Connect endpoints, see
//! <a href="https://docs.aws.amazon.com/general/latest/gr/connect_region.html">Amazon Connect
//! Endpoints</a>.</p>
//! <note>
//! <p>Working with contact flows? Check out the <a href="https://docs.aws.amazon.com/connect/latest/adminguide/flow-language.html">Amazon Connect Flow language</a>.</p>
//! </note>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
mod idempotency_token;
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
pub mod model;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("connect", PKG_VERSION);
pub use aws_auth::Credentials;
pub use aws_types::region::Region;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;