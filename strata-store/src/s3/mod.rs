mod config;
mod error;
mod signing;

pub use config::S3Config;
pub use error::SigningError;
pub use signing::{s3_authorization, s3_signing_key};
