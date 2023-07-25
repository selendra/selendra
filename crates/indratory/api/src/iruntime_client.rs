use anyhow::Result;
use log::info;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::boxed::Box;

use crate::irpc::{
    client::{Error as ClientError, RequestClient},
    indratory_api_client::IndratoryApiClient,
    server::ProtoError as ServerError,
    Message,
};

pub type IruntimeClient = IndratoryApiClient<RpcRequest>;

pub fn new_iruntime_client(base_url: String) -> IndratoryApiClient<RpcRequest> {
    IndratoryApiClient::new(RpcRequest::new(base_url))
}

pub struct RpcRequest {
    base_url: String,
}

impl RpcRequest {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
}

#[async_trait::async_trait]
impl RequestClient for RpcRequest {
    async fn request(&self, path: &str, body: Vec<u8>) -> Result<Vec<u8>, ClientError> {
        fn from_display(err: impl core::fmt::Display) -> ClientError {
            ClientError::RpcError(err.to_string())
        }

        let url = alloc::format!("{}/irpc/{path}", self.base_url);
        let res = reqwest::Client::new()
            .post(url)
            .header("Connection", "close")
            .body(body)
            .send()
            .await
            .map_err(from_display)?;

        info!("{path}: {}", res.status());
        let status = res.status();
        let body = res.bytes().await.map_err(from_display)?;
        if status.is_success() {
            Ok(body.as_ref().to_vec())
        } else {
            let err: ServerError = Message::decode(body.as_ref())?;
            Err(ClientError::ServerError(err))
        }
    }
}
