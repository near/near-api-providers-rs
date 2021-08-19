#![deprecated(note = "This crate is unstable and hence, unfit for use.")]
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;

use near_jsonrpc_primitives::errors::RpcError;
use near_jsonrpc_primitives::message::{from_slice, Message};
use near_primitives::hash::CryptoHash;
use near_primitives::types::{AccountId, BlockId, BlockReference, MaybeBlockId, ShardId};
use near_primitives::views;

#[derive(Debug, Serialize)]
pub enum ChunkId {
    BlockShardId(BlockId, ShardId),
    Hash(CryptoHash),
}

pub enum ExperimentalRpcMethod {
    CheckTx(views::SignedTransactionView),
    GenesisConfig,
    BroadcastTxSync(views::SignedTransactionView),
    TxStatus(String),
    Changes(near_jsonrpc_primitives::types::changes::RpcStateChangesRequest),
    ValidatorsOrdered(near_jsonrpc_primitives::types::validator::RpcValidatorsOrderedRequest),
    Receipt(near_jsonrpc_primitives::types::receipts::RpcReceiptRequest),
    ProtocolConfig(near_jsonrpc_primitives::types::config::RpcProtocolConfigRequest),
}

pub enum RpcMethod {
    BroadcastTxAsync(views::SignedTransactionView),
    BroadcastTxCommit(views::SignedTransactionView),
    Status,
    Health,
    Tx(CryptoHash, AccountId),
    Chunk(ChunkId),
    Validators(MaybeBlockId),
    GasPrice(MaybeBlockId),
    Query(near_jsonrpc_primitives::types::query::RpcQueryRequest),
    Block(BlockReference),
    Experimental(ExperimentalRpcMethod),
}

impl RpcMethod {
    fn method_and_params(&self) -> (&str, serde_json::Value) {
        use ExperimentalRpcMethod::*;
        use RpcMethod::*;
        match self {
            BroadcastTxAsync(tx) => ("broadcast_tx_async", json!([tx])),
            BroadcastTxCommit(tx) => ("broadcast_tx_commit", json!([tx])),
            Status => ("status", json!([])),
            Health => ("health", json!([])),
            Tx(hash, id) => ("tx", json!([hash, id])),
            Chunk(id) => ("chunk", json!([id])),
            Validators(block_id) => ("validators", json!([block_id])),
            GasPrice(block_id) => ("gas_price", json!([block_id])),
            Query(request) => ("query", json!(request)),
            Block(request) => ("block", json!(request)),
            Experimental(method) => match method {
                CheckTx(tx) => ("EXPERIMENTAL_check_tx", json!([tx])),
                GenesisConfig => ("EXPERIMENTAL_genesis_config", json!([])),
                BroadcastTxSync(tx) => ("EXPERIMENTAL_broadcast_tx_sync", json!([tx])),
                TxStatus(tx) => ("EXPERIMENTAL_tx_status", json!([tx])),
                Changes(request) => ("EXPERIMENTAL_changes", json!(request)),
                ValidatorsOrdered(request) => ("EXPERIMENTAL_validators_ordered", json!(request)),
                Receipt(request) => ("EXPERIMENTAL_receipt", json!(request)),
                ProtocolConfig(request) => ("EXPERIMENTAL_protocol_config", json!(request)),
            },
        }
    }

    pub async fn call_on<T: DeserializeOwned>(
        &self,
        rpc_client: &JsonRpcClient,
    ) -> Result<T, RpcError> {
        let (method_name, params) = self.method_and_params();
        let request_payload = Message::request(method_name.to_string(), Some(params));
        let request = rpc_client
            .client
            .post(&rpc_client.server_addr)
            .header("Content-Type", "application/json")
            .json(&request_payload);
        let response = request
            .send()
            .await
            .map_err(|err| RpcError::new_internal_error(None, format!("{:?}", err)))?;
        let response_payload = response.bytes().await.map_err(|err| {
            RpcError::parse_error(format!("Failed to retrieve response payload: {:?}", err))
        })?;
        if let Message::Response(response) = from_slice(&response_payload).map_err(|err| {
            RpcError::parse_error(format!("Failed parsing response payload: {:?}", err))
        })? {
            return serde_json::from_value(response.result?)
                .map_err(|err| RpcError::parse_error(format!("Failed to parse: {:?}", err)));
        }
        Err(RpcError::parse_error(format!(
            "Failed to parse JSON RPC response"
        )))
    }
}

use ExperimentalRpcMethod::*;
use RpcMethod::*;

#[derive(Clone)]
pub struct JsonRpcClientBuilder {
    client: reqwest::Client,
}

impl JsonRpcClientBuilder {
    pub fn connect(&self, server_addr: &str) -> JsonRpcClient {
        JsonRpcClient {
            server_addr: server_addr.to_string(),
            client: self.client.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JsonRpcClient {
    server_addr: String,
    client: reqwest::Client,
}

impl JsonRpcClient {
    pub fn new_client() -> JsonRpcClientBuilder {
        JsonRpcClientBuilder {
            client: reqwest::Client::new(),
        }
    }

    pub async fn broadcast_tx_async(
        &self,
        tx: views::SignedTransactionView,
    ) -> Result<String, RpcError> {
        BroadcastTxAsync(tx).call_on(self).await
    }

    pub async fn broadcast_tx_commit(
        &self,
        tx: views::SignedTransactionView,
    ) -> Result<views::FinalExecutionOutcomeView, RpcError> {
        BroadcastTxCommit(tx).call_on(self).await
    }

    pub async fn status(&self) -> Result<views::StatusResponse, RpcError> {
        Status.call_on(self).await
    }

    pub async fn health(&self) -> Result<(), RpcError> {
        Health.call_on(self).await
    }

    pub async fn tx(
        &self,
        hash: CryptoHash,
        id: AccountId,
    ) -> Result<views::FinalExecutionOutcomeView, RpcError> {
        Tx(hash, id).call_on(self).await
    }

    pub async fn chunk(&self, id: ChunkId) -> Result<views::ChunkView, RpcError> {
        Chunk(id).call_on(self).await
    }

    pub async fn validators(
        &self,
        block_id: MaybeBlockId,
    ) -> Result<views::EpochValidatorInfo, RpcError> {
        Validators(block_id).call_on(self).await
    }

    pub async fn gas_price(&self, block_id: MaybeBlockId) -> Result<views::GasPriceView, RpcError> {
        GasPrice(block_id).call_on(self).await
    }

    pub async fn query(
        &self,
        request: near_jsonrpc_primitives::types::query::RpcQueryRequest,
    ) -> Result<near_jsonrpc_primitives::types::query::RpcQueryResponse, RpcError> {
        Query(request).call_on(self).await
    }

    pub async fn block(&self, request: BlockReference) -> Result<views::BlockView, RpcError> {
        Block(request).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_check_tx(
        &self,
        tx: views::SignedTransactionView,
    ) -> Result<serde_json::Value, RpcError> {
        Experimental(CheckTx(tx)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_genesis_config(&self) -> Result<serde_json::Value, RpcError> {
        Experimental(GenesisConfig).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_broadcast_tx_sync(
        &self,
        tx: views::SignedTransactionView,
    ) -> Result<serde_json::Value, RpcError> {
        Experimental(BroadcastTxSync(tx)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_tx_status(&self, tx: String) -> Result<serde_json::Value, RpcError> {
        Experimental(TxStatus(tx)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_changes(
        &self,
        request: near_jsonrpc_primitives::types::changes::RpcStateChangesRequest,
    ) -> Result<near_jsonrpc_primitives::types::changes::RpcStateChangesResponse, RpcError> {
        Experimental(Changes(request)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_validators_ordered(
        &self,
        request: near_jsonrpc_primitives::types::validator::RpcValidatorsOrderedRequest,
    ) -> Result<Vec<views::validator_stake_view::ValidatorStakeView>, RpcError> {
        Experimental(ValidatorsOrdered(request)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_receipt(
        &self,
        request: near_jsonrpc_primitives::types::receipts::RpcReceiptRequest,
    ) -> Result<near_jsonrpc_primitives::types::receipts::RpcReceiptResponse, RpcError> {
        Experimental(Receipt(request)).call_on(self).await
    }

    #[allow(non_snake_case)]
    pub async fn EXPERIMENTAL_protocol_config(
        &self,
        request: near_jsonrpc_primitives::types::config::RpcProtocolConfigRequest,
    ) -> Result<near_jsonrpc_primitives::types::config::RpcProtocolConfigResponse, RpcError> {
        Experimental(ProtocolConfig(request)).call_on(self).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{JsonRpcClient, RpcMethod};

    #[tokio::test]
    async fn it_works() {
        let rpc_client = JsonRpcClient::new_client().connect("http://localhost:3030");
        let status1 = rpc_client.status().await;
        let status2 = RpcMethod::Status
            .call_on::<near_primitives::views::StatusResponse>(&rpc_client)
            .await;

        println!("{:?}", status1);
        println!("{:?}", status2);
    }
}
