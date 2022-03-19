//! Queries the status of a transaction.
//!
//! ## Example
//!
//! Returns the final transaction result for
//! <https://explorer.near.org/transactions/B9aypWiMuiWR5kqzewL9eC96uZWA3qCMhLe67eBMWacq>
//!
//! ```
//! use near_jsonrpc_client::{methods, JsonRpcClient};
//! use near_primitives::views;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = JsonRpcClient::connect("https://archival-rpc.mainnet.near.org");
//! let tx_hash = "B9aypWiMuiWR5kqzewL9eC96uZWA3qCMhLe67eBMWacq".parse()?;
//!
//! let request = methods::EXPERIMENTAL_tx_status::RpcTransactionStatusRequest {
//!     transaction_info: methods::EXPERIMENTAL_tx_status::TransactionInfo::TransactionId {
//!         hash: tx_hash,
//!         account_id: "itranscend.near".parse()?,
//!    }
//! };
//!
//! let response = client.call(request).await?;
//!
//! assert!(matches!(
//!     response,
//!     views::FinalExecutionOutcomeWithReceiptView { .. }
//! ));
//! # Ok(())
//! # }
//! ```
use super::*;

pub use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
pub use near_jsonrpc_primitives::types::transactions::TransactionInfo;

pub type RpcTransactionStatusResponse =
    near_primitives::views::FinalExecutionOutcomeWithReceiptView;

#[derive(Debug)]
pub struct RpcTransactionStatusRequest {
    pub transaction_info: TransactionInfo,
}

impl From<RpcTransactionStatusRequest>
    for near_jsonrpc_primitives::types::transactions::RpcTransactionStatusCommonRequest
{
    fn from(this: RpcTransactionStatusRequest) -> Self {
        Self {
            transaction_info: this.transaction_info,
        }
    }
}

impl RpcHandlerResponse for RpcTransactionStatusResponse {}

impl RpcMethod for RpcTransactionStatusRequest {
    type Response = RpcTransactionStatusResponse;
    type Error = RpcTransactionError;

    fn method_name(&self) -> &str {
        "EXPERIMENTAL_tx_status"
    }

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(match &self.transaction_info {
            TransactionInfo::Transaction(signed_transaction) => {
                json!([common::serialize_signed_transaction(signed_transaction)?])
            }
            TransactionInfo::TransactionId { hash, account_id } => {
                json!([hash, account_id])
            }
        })
    }
}

impl private::Sealed for RpcTransactionStatusRequest {}
