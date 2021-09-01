//! HTTP API Client for the NEAR Protocol

use thiserror::Error;

use serde::de::DeserializeOwned;

use super::NearClient;

#[derive(Debug, Error)]
pub enum HttpTransportSendError {
    #[error("error while sending payload: [{0}]")]
    PayloadSendError(reqwest::Error),
    #[error("error while serializing payload: [{0}]")]
    PayloadSerializeError(serde_json::Error),
}

#[derive(Debug, Error)]
pub enum HttpTransportRecvError {
    #[error("error while reading response: [{0}]")]
    PayloadRecvError(reqwest::Error),
    #[error("error while parsing response: [{0}]")]
    PayloadParseError(serde_json::Error),
}

#[derive(Debug, Error)]
pub enum HttpMethodCallError {
    #[error(transparent)]
    Send(HttpTransportSendError),
    #[error(transparent)]
    Recv(HttpTransportRecvError),
}

type HttpMethodCallResult<T> = Result<T, HttpMethodCallError>;

struct HttpMethodCaller(&'static str, Option<serde_json::Value>);

impl HttpMethodCaller {
    fn _params(mut self, params: serde_json::Value) -> Self {
        self.1.replace(params);
        self
    }

    async fn call_on<T: DeserializeOwned>(
        &self,
        http_client: &NearHttpClient,
    ) -> HttpMethodCallResult<T> {
        let near_client = &http_client.near_client;
        let mut request = near_client
            .client
            .get(format!("{}/{}", near_client.server_addr, self.0));
        if let Some(params) = &self.1 {
            request = request.body(serde_json::to_vec(params).map_err(|err| {
                HttpMethodCallError::Send(HttpTransportSendError::PayloadSerializeError(err))
            })?);
        }
        let response = request
            .send()
            .await
            .map_err(|err| {
                HttpMethodCallError::Send(HttpTransportSendError::PayloadSendError(err))
            })?
            .bytes()
            .await
            .map_err(|err| {
                HttpMethodCallError::Recv(HttpTransportRecvError::PayloadRecvError(err))
            })?;
        serde_json::from_slice(&response).map_err(|err| {
            HttpMethodCallError::Recv(HttpTransportRecvError::PayloadParseError(err))
        })
    }
}

fn http_method(method_name: &'static str) -> HttpMethodCaller {
    HttpMethodCaller(method_name, None)
}

#[derive(Clone)]
pub struct NearHttpClient {
    pub(crate) near_client: NearClient,
}

impl NearHttpClient {
    pub async fn status(&self) -> HttpMethodCallResult<near_primitives::views::StatusResponse> {
        http_method("status").call_on(self).await
    }

    pub async fn health(
        &self,
    ) -> HttpMethodCallResult<near_jsonrpc_primitives::types::status::RpcHealthResponse> {
        http_method("health").call_on(self).await?;
        Ok(near_jsonrpc_primitives::types::status::RpcHealthResponse)
    }

    pub async fn network_info(
        &self,
    ) -> HttpMethodCallResult<near_client_primitives::types::NetworkInfoResponse> {
        http_method("network_info").call_on(self).await
    }

    pub async fn metrics(&self) -> HttpMethodCallResult<String> {
        http_method("metrics").call_on(self).await
    }
}
