mod config;
mod handler;
mod service;

pub use config::setup;
pub use service::Service;

const LOG_TARGET: &str = "selendra-base-protocol";
