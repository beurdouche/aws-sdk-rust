//! <p>Amazon Polly is a web service that makes it easy to synthesize speech from
//! text.</p>
//! <p>The Amazon Polly service provides API operations for synthesizing high-quality speech
//! from plain text and Speech Synthesis Markup Language (SSML), along with managing
//! pronunciations lexicons that enable you to get the best results for your application
//! domain.</p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_json_errors;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
mod http_serde;
pub mod input;
mod instant_epoch;
pub mod model;
pub mod operation;
pub mod output;
mod serde_util;
mod serializer;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("polly", PKG_VERSION);
pub use aws_auth::Credentials;
pub use aws_types::region::Region;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;