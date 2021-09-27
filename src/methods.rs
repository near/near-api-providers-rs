use std::io;

use serde_json::json;

mod chk {
    // this lets us make the RpcMethod trait public but non-implementable by users outside this crate
    pub trait ValidRpcMarkerTrait {}
}

pub trait RpcMethod: chk::ValidRpcMarkerTrait
where
    Self::Result: RpcHandlerResult,
    Self::Error: RpcHandlerError,
{
    type Result;
    type Error;

    const METHOD_NAME: &'static str;

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        Ok(json!(null))
    }
}

impl<T> chk::ValidRpcMarkerTrait for &T where T: chk::ValidRpcMarkerTrait {}
impl<T> RpcMethod for &T
where
    T: RpcMethod,
{
    type Result = T::Result;
    type Error = T::Error;

    const METHOD_NAME: &'static str = T::METHOD_NAME;

    fn params(&self) -> Result<serde_json::Value, io::Error> {
        T::params(self)
    }
}

pub trait RpcHandlerResult: serde::de::DeserializeOwned + chk::ValidRpcMarkerTrait {
    fn parse_result(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(value)
    }
}

pub trait RpcHandlerError: serde::de::DeserializeOwned + chk::ValidRpcMarkerTrait {
    /// parser for the `.data` field in RpcError, not `.error_struct`
    /// this would only ever be used if `.error_struct` can't be deserialized
    fn parse_raw_error(_value: serde_json::Value) -> Option<Result<Self, serde_json::Error>> {
        None
    }
}

macro_rules! impl_method {
    (
        $(#[$meta:meta])*
        pub mod $method_name:ident {
            $($body:tt)+
        }
    ) => {
        #[allow(non_snake_case)]
        pub mod $method_name {
            $(#![$meta])*

            use super::*;

            const METHOD_NAME: &'static str = stringify!($method_name);

            $($body)+
        }
    }
}

macro_rules! impl_ {
    (RpcMethod for $for_type:ty { $($body:tt)+ }) => {
        impl chk::ValidRpcMarkerTrait for $for_type {}
        impl_!(@final RpcMethod for $for_type {
            const METHOD_NAME: &'static str = METHOD_NAME;
            $($body)+
        });
    };
    ($valid_trait:ident for $for_type:ty { $($body:tt)* }) => {
        impl chk::ValidRpcMarkerTrait for $for_type {}
        impl_!(@final $valid_trait for $for_type { $($body)* });
    };
    (@final $valid_trait:ident for $for_type:ty { $($body:tt)* }) => {
        impl $valid_trait for $for_type { $($body)* }
    };
}

mod shared_structs {
    use super::{chk, RpcHandlerError, RpcHandlerResult};

    impl chk::ValidRpcMarkerTrait for () {}

    // broadcast_tx_async, EXPERIMENTAL_genesis_config, adv_*
    impl_!(@final RpcHandlerError for () {});

    // adv_*
    impl_!(@final RpcHandlerResult for () {
        fn parse_result(_value: serde_json::Value) -> Result<Self, serde_json::Error> {
            Ok(())
        }
    });

    // broadcast_tx_commit, tx
    impl_!(RpcHandlerResult for near_primitives::views::FinalExecutionOutcomeView {});

    // broadcast_tx_commit, tx, EXPERIMENTAL_check_tx, EXPERIMENTAL_tx_status
    impl_!(RpcHandlerError for near_jsonrpc_primitives::types::transactions::RpcTransactionError {
        fn parse_raw_error(value: serde_json::Value) -> Option<Result<Self, serde_json::Error>> {
            match serde_json::from_value::<near_jsonrpc_primitives::errors::ServerError>(value) {
                Ok(near_jsonrpc_primitives::errors::ServerError::TxExecutionError(
                    near_primitives::errors::TxExecutionError::InvalidTxError(context),
                )) => Some(Ok(Self::InvalidTransaction { context })),
                Err(err) => Some(Err(err)),
                _ => None,
            }
        }
    });

    // health, status
    impl_!(RpcHandlerError for near_jsonrpc_primitives::types::status::RpcStatusError {});

    // EXPERIMENTAL_changes, EXPERIMENTAL_changes_in_block
    impl_!(RpcHandlerError for near_jsonrpc_primitives::types::changes::RpcStateChangesError {});

    // EXPERIMENTAL_broadcast_tx_sync, EXPERIMENTAL_check_tx
    impl_!(RpcHandlerResult for near_jsonrpc_primitives::types::transactions::RpcBroadcastTxSyncResponse {});

    // validators, EXPERIMENTAL_validators_ordered
    impl_!(RpcHandlerError for near_jsonrpc_primitives::types::validator::RpcValidatorError {});
}

impl_method! {
    pub mod block {
        pub use near_jsonrpc_primitives::types::blocks::RpcBlockError;
        pub use near_jsonrpc_primitives::types::blocks::RpcBlockRequest;
        pub use near_primitives::views::BlockView;

        impl_!(RpcHandlerResult for BlockView {});

        impl_!(RpcHandlerError for RpcBlockError {});

        impl_!(RpcMethod for RpcBlockRequest {
            type Result = BlockView;
            type Error = RpcBlockError;

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!([self]))
            }
        });
    }
}

fn serialize_signed_transaction(
    tx: &near_primitives::transaction::SignedTransaction,
) -> Result<String, io::Error> {
    Ok(near_primitives::serialize::to_base64(
        &borsh::BorshSerialize::try_to_vec(&tx)?,
    ))
}

impl_method! {
    pub mod broadcast_tx_async {
        pub use near_primitives::hash::CryptoHash;
        pub use near_primitives::transaction::SignedTransaction;

        #[derive(Debug)]
        pub struct RpcBroadcastTxAsyncRequest {
            pub signed_transaction: SignedTransaction,
        }

        impl From<RpcBroadcastTxAsyncRequest>
            for near_jsonrpc_primitives::types::transactions::RpcBroadcastTransactionRequest
        {
            fn from(this: RpcBroadcastTxAsyncRequest) -> Self {
                Self {
                    signed_transaction: this.signed_transaction,
                }
            }
        }

        impl_!(RpcHandlerResult for CryptoHash {});

        impl_!(RpcMethod for RpcBroadcastTxAsyncRequest {
            type Result = CryptoHash;
            type Error = ();

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!([serialize_signed_transaction(&self.signed_transaction)?]))
            }
        });
    }
}

impl_method! {
    pub mod broadcast_tx_commit {
        pub use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
        pub use near_primitives::transaction::SignedTransaction;
        pub use near_primitives::views::FinalExecutionOutcomeView;

        #[derive(Debug)]
        pub struct RpcBroadcastTxCommitRequest {
            pub signed_transaction: SignedTransaction,
        }

        impl From<RpcBroadcastTxCommitRequest>
            for near_jsonrpc_primitives::types::transactions::RpcBroadcastTransactionRequest
        {
            fn from(this: RpcBroadcastTxCommitRequest) -> Self {
                Self {
                    signed_transaction: this.signed_transaction,
                }
            }
        }

        impl_!(RpcMethod for RpcBroadcastTxCommitRequest {
            type Result = FinalExecutionOutcomeView;
            type Error = RpcTransactionError;

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!([serialize_signed_transaction(&self.signed_transaction)?]))
            }
        });
    }
}

impl_method! {
    pub mod chunk {
        pub use near_jsonrpc_primitives::types::chunks::{RpcChunkError, RpcChunkRequest};
        pub use near_primitives::views::ChunkView;

        impl_!(RpcHandlerResult for ChunkView {});

        impl_!(RpcHandlerError for RpcChunkError {});

        impl_!(RpcMethod for RpcChunkRequest {
            type Result = ChunkView;
            type Error = RpcChunkError;

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!([self]))
            }
        });
    }
}

impl_method! {
    pub mod gas_price {
        pub use near_jsonrpc_primitives::types::gas_price::{
            RpcGasPriceError, RpcGasPriceRequest,
        };
        pub use near_primitives::views::GasPriceView;

        impl_!(RpcHandlerResult for GasPriceView {});

        impl_!(RpcHandlerError for RpcGasPriceError {});

        impl_!(RpcMethod for RpcGasPriceRequest {
            type Result = GasPriceView;
            type Error = RpcGasPriceError;

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!([self]))
            }
        });
    }
}

impl_method! {
    pub mod health {
        pub use near_jsonrpc_primitives::types::status::{
            RpcHealthResponse, RpcStatusError,
        };

        #[derive(Debug)]
        pub struct RpcHealthRequest;

        impl_!(RpcHandlerResult for RpcHealthResponse {});

        impl_!(RpcMethod for RpcHealthRequest {
            type Result = RpcHealthResponse;
            type Error = RpcStatusError;
        });
    }
}

impl_method! {
    pub mod light_client_proof {
        pub use near_jsonrpc_primitives::types::light_client::{
            RpcLightClientExecutionProofRequest, RpcLightClientExecutionProofResponse,
            RpcLightClientProofError,
        };

        impl_!(RpcHandlerResult for RpcLightClientExecutionProofResponse {});

        impl_!(RpcHandlerError for RpcLightClientProofError {});

        impl_!(RpcMethod for RpcLightClientExecutionProofRequest {
            type Result = RpcLightClientExecutionProofResponse;
            type Error = RpcLightClientProofError;

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!(self))
            }
        });
    }
}

impl_method! {
    pub mod next_light_client_block {
        pub use near_jsonrpc_primitives::types::light_client::{
            RpcLightClientNextBlockError, RpcLightClientNextBlockRequest,
        };
        pub use near_primitives::views::LightClientBlockView;
        pub type RpcLightClientNextBlockResponse = Option<LightClientBlockView>;

        impl_!(RpcHandlerResult for RpcLightClientNextBlockResponse {});

        impl_!(RpcHandlerError for RpcLightClientNextBlockError {});

        impl_!(RpcMethod for RpcLightClientNextBlockRequest {
            type Result = RpcLightClientNextBlockResponse;
            type Error = RpcLightClientNextBlockError;

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!(self))
            }
        });
    }
}

impl_method! {
    pub mod network_info {
        pub use near_client_primitives::types::NetworkInfoResponse;
        pub use near_jsonrpc_primitives::types::network_info::RpcNetworkInfoError;

        #[derive(Debug)]
        pub struct RpcNetworkInfoRequest;

        impl_!(RpcHandlerResult for NetworkInfoResponse {});

        impl_!(RpcHandlerError for RpcNetworkInfoError {});

        impl_!(RpcMethod for RpcNetworkInfoRequest {
            type Result = NetworkInfoResponse;
            type Error = RpcNetworkInfoError;
        });
    }
}

impl_method! {
    pub mod query {
        pub use near_jsonrpc_primitives::types::query::{
            RpcQueryError, RpcQueryRequest, RpcQueryResponse,
        };

        impl_!(RpcHandlerResult for RpcQueryResponse {});

        impl_!(RpcHandlerError for RpcQueryError {});

        impl_!(RpcMethod for RpcQueryRequest {
            type Result = RpcQueryResponse;
            type Error = RpcQueryError;

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!(self))
            }
        });
    }
}

impl_method! {
    pub mod status {
        pub use near_jsonrpc_primitives::types::status::RpcStatusError;
        pub use near_primitives::views::StatusResponse;

        #[derive(Debug)]
        pub struct RpcStatusRequest;

        impl_!(RpcHandlerResult for StatusResponse {});

        impl_!(RpcMethod for RpcStatusRequest {
            type Result = StatusResponse;
            type Error = RpcStatusError;
        });
    }
}

impl_method! {
    pub mod tx {
        pub use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
        pub use near_jsonrpc_primitives::types::transactions::TransactionInfo;
        pub use near_primitives::views::FinalExecutionOutcomeView;

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

        impl_!(RpcMethod for RpcTransactionStatusRequest {
            type Result = FinalExecutionOutcomeView;
            type Error = RpcTransactionError;

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(
                    match &self.transaction_info {
                        TransactionInfo::Transaction(signed_transaction) => {
                            json!([serialize_signed_transaction(&signed_transaction)?])
                        }
                        TransactionInfo::TransactionId { hash, account_id } => {
                            json!([hash, account_id])
                        }
                    }
                )
            }
        });
    }
}

impl_method! {
    pub mod validators {
        pub use near_jsonrpc_primitives::types::validator::{
            RpcValidatorError, RpcValidatorRequest,
        };
        pub use near_primitives::views::EpochValidatorInfo;

        impl_!(RpcHandlerResult for EpochValidatorInfo {});

        impl_!(RpcMethod for RpcValidatorRequest {
            type Result = EpochValidatorInfo;
            type Error = RpcValidatorError;

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!(self))
            }
        });
    }
}

impl_method! {
    pub mod EXPERIMENTAL_broadcast_tx_sync {
        pub use near_jsonrpc_primitives::types::transactions::{
            RpcBroadcastTxSyncResponse, RpcTransactionError,
        };
        pub use near_primitives::transaction::SignedTransaction;

        #[derive(Debug)]
        pub struct RpcBroadcastTxSyncRequest {
            pub signed_transaction: SignedTransaction,
        }

        impl From<RpcBroadcastTxSyncRequest>
            for near_jsonrpc_primitives::types::transactions::RpcBroadcastTransactionRequest
        {
            fn from(this: RpcBroadcastTxSyncRequest) -> Self {
                Self { signed_transaction: this.signed_transaction }
            }
        }

        impl_!(RpcMethod for RpcBroadcastTxSyncRequest {
            type Result = RpcBroadcastTxSyncResponse;
            type Error = RpcTransactionError;

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!([serialize_signed_transaction(&self.signed_transaction)?]))
            }
        });
    }
}

impl_method! {
    pub mod EXPERIMENTAL_changes {
        pub use near_jsonrpc_primitives::types::changes::{
            RpcStateChangesError, RpcStateChangesInBlockByTypeRequest,
            RpcStateChangesInBlockResponse,
        };

        impl_!(RpcHandlerResult for RpcStateChangesInBlockResponse {});

        impl_!(RpcMethod for RpcStateChangesInBlockByTypeRequest {
            type Result = RpcStateChangesInBlockResponse;
            type Error = RpcStateChangesError;

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!(self))
            }
        });
    }
}

impl_method! {
    pub mod EXPERIMENTAL_changes_in_block {
        pub use near_jsonrpc_primitives::types::changes::{
            RpcStateChangesError, RpcStateChangesInBlockRequest,
            RpcStateChangesInBlockByTypeResponse,
        };

        impl_!(RpcHandlerResult for RpcStateChangesInBlockByTypeResponse {});

        impl_!(RpcMethod for RpcStateChangesInBlockRequest {
            type Result = RpcStateChangesInBlockByTypeResponse;
            type Error = RpcStateChangesError;

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!(self))
            }
        });
    }
}

impl_method! {
    pub mod EXPERIMENTAL_check_tx {
        pub use near_jsonrpc_primitives::types::transactions::{
            RpcBroadcastTxSyncResponse, RpcTransactionError,
        };
        pub use near_primitives::transaction::SignedTransaction;

        #[derive(Debug)]
        pub struct RpcCheckTxRequest {
            pub signed_transaction: SignedTransaction,
        }

        impl From<RpcCheckTxRequest>
            for near_jsonrpc_primitives::types::transactions::RpcBroadcastTransactionRequest
        {
            fn from(this: RpcCheckTxRequest) -> Self {
                Self { signed_transaction: this.signed_transaction }
            }
        }

        impl_!(RpcMethod for RpcCheckTxRequest {
            type Result = RpcBroadcastTxSyncResponse;
            type Error = RpcTransactionError;

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!([serialize_signed_transaction(&self.signed_transaction)?]))
            }
        });
    }
}

impl_method! {
    pub mod EXPERIMENTAL_genesis_config {
        pub use near_chain_configs::GenesisConfig;

        #[derive(Debug)]
        pub struct RpcGenesisConfigRequest;

        impl_!(RpcHandlerResult for GenesisConfig {});

        impl_!(RpcMethod for RpcGenesisConfigRequest {
            type Result = GenesisConfig;
            type Error = ();
        });
    }
}

impl_method! {
    pub mod EXPERIMENTAL_protocol_config {
        pub use near_chain_configs::ProtocolConfigView;
        pub use near_jsonrpc_primitives::types::config::{
            RpcProtocolConfigError, RpcProtocolConfigRequest,
        };

        impl_!(RpcHandlerResult for ProtocolConfigView {});

        impl_!(RpcHandlerError for RpcProtocolConfigError {});

        impl_!(RpcMethod for RpcProtocolConfigRequest {
            type Result = ProtocolConfigView;
            type Error = RpcProtocolConfigError;

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!(self))
            }
        });
    }
}

impl_method! {
    pub mod EXPERIMENTAL_receipt {
        pub use near_jsonrpc_primitives::types::receipts::{
            RpcReceiptError, RpcReceiptRequest,
        };
        pub use near_primitives::views::ReceiptView;

        impl_!(RpcHandlerResult for ReceiptView {});

        impl_!(RpcHandlerError for RpcReceiptError {});

        impl_!(RpcMethod for RpcReceiptRequest {
            type Result = ReceiptView;
            type Error = RpcReceiptError;

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!(self))
            }
        });
    }
}

impl_method! {
    pub mod EXPERIMENTAL_tx_status {
        pub use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
        pub use near_jsonrpc_primitives::types::transactions::TransactionInfo;
        pub use near_primitives::views::FinalExecutionOutcomeWithReceiptView;

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

        impl_!(RpcHandlerResult for FinalExecutionOutcomeWithReceiptView {});

        impl_!(RpcMethod for RpcTransactionStatusRequest {
            type Result = FinalExecutionOutcomeWithReceiptView;
            type Error = RpcTransactionError;

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(
                    match &self.transaction_info {
                        TransactionInfo::Transaction(signed_transaction) => {
                            json!([serialize_signed_transaction(&signed_transaction)?])
                        }
                        TransactionInfo::TransactionId { hash, account_id } => {
                            json!([hash, account_id])
                        }
                    }
                )
            }
        });
    }
}

impl_method! {
    pub mod EXPERIMENTAL_validators_ordered {
        pub use near_jsonrpc_primitives::types::validator::{
            RpcValidatorError, RpcValidatorsOrderedRequest, RpcValidatorsOrderedResponse,
        };

        impl_!(RpcHandlerResult for RpcValidatorsOrderedResponse {});

        impl_!(RpcMethod for RpcValidatorsOrderedRequest {
            type Result = RpcValidatorsOrderedResponse;
            type Error = RpcValidatorError;

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!(self))
            }
        });
    }
}

#[cfg(feature = "sandbox")]
impl_method! {
    pub mod sandbox_patch_state {
        pub use near_jsonrpc_primitives::types::sandbox::{
            RpcSandboxPatchStateError, RpcSandboxPatchStateRequest,
            RpcSandboxPatchStateResponse,
        };

        impl_!(RpcHandlerResult for RpcSandboxPatchStateResponse {});

        impl_!(RpcHandlerError for RpcSandboxPatchStateError {});

        impl_!(RpcMethod for RpcSandboxPatchStateRequest {
            type Result = RpcSandboxPatchStateResponse;
            type Error = RpcSandboxPatchStateError;

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!(self))
            }
        });
    }
}

#[cfg(feature = "adversarial")]
impl_method! {
    pub mod adv_set_weight {
        #[derive(Debug)]
        pub struct RpcAdversarialSetWeightRequest { pub height: u64 }

        impl_!(RpcMethod for RpcAdversarialSetWeightRequest {
            type Result = ();
            type Error = ();

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!(self.height))
            }
        });
    }
}

#[cfg(feature = "adversarial")]
impl_method! {
    pub mod adv_disable_header_sync {
        #[derive(Debug)]
        pub struct RpcAdversarialDisableHeaderSyncRequest;

        impl_!(RpcMethod for RpcAdversarialDisableHeaderSyncRequest {
            type Result = ();
            type Error = ();
        });
    }
}

#[cfg(feature = "adversarial")]
impl_method! {
    pub mod adv_disable_doomslug {
        #[derive(Debug)]
        pub struct RpcAdversarialDisableDoomslugRequest;

        impl_!(RpcMethod for RpcAdversarialDisableDoomslugRequest {
            type Result = ();
            type Error = ();
        });
    }
}

#[cfg(feature = "adversarial")]
impl_method! {
    pub mod adv_produce_blocks {
        #[derive(Debug)]
        pub struct RpcAdversarialProduceBlocksRequest {
            pub num_blocks: u64,
            pub only_valid: bool,
        }

        impl_!(RpcMethod for RpcAdversarialProduceBlocksRequest {
            type Result = ();
            type Error = ();

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!([self.num_blocks, self.only_valid]))
            }
        });
    }
}

#[cfg(feature = "adversarial")]
impl_method! {
    pub mod adv_switch_to_height {
        #[derive(Debug)]
        pub struct RpcAdversarialSwitchToHeightRequest { pub height: u64 }

        impl_!(RpcMethod for RpcAdversarialSwitchToHeightRequest {
            type Result = ();
            type Error = ();

            fn params(&self) -> Result<serde_json::Value, io::Error> {
                Ok(json!([self.height]))
            }
        });
    }
}

#[cfg(feature = "adversarial")]
impl_method! {
    pub mod adv_get_saved_blocks {
        use serde::Deserialize;

        #[derive(Debug)]
        pub struct RpcAdversarialGetSavedBlocksRequest;

        #[derive(Debug, Deserialize)]
        pub struct RpcAdversarialGetSavedBlocksResponse(pub u64);

        impl_!(RpcHandlerResult for RpcAdversarialGetSavedBlocksResponse {});

        impl_!(RpcMethod for RpcAdversarialGetSavedBlocksRequest {
            type Result = RpcAdversarialGetSavedBlocksResponse;
            type Error = ();
        });
    }
}

#[cfg(feature = "adversarial")]
impl_method! {
    pub mod adv_check_store {
        use serde::Deserialize;

        #[derive(Debug)]
        pub struct RpcAdversarialCheckStoreRequest;

        #[derive(Debug, Deserialize)]
        pub struct RpcAdversarialCheckStoreResponse(pub u64);

        impl_!(RpcHandlerResult for RpcAdversarialCheckStoreResponse {});

        impl_!(RpcMethod for RpcAdversarialCheckStoreRequest {
            type Result = RpcAdversarialCheckStoreResponse;
            type Error = ();
        });
    }
}
