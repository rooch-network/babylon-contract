use crate::msg::btc_header::{BtcHeader, BtcHeaderResponse, BtcHeadersResponse};
use crate::msg::cz_header::CzHeaderResponse;
use crate::msg::epoch::EpochResponse;
use crate::state::config::Config;
use babylon_apis::Bytes;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, StdError, StdResult};

const BABYLON_TAG_BYTES: usize = 4;

// common functions for contract msgs
pub trait ContractMsg {
    fn validate(&self) -> StdResult<()>;
    fn babylon_tag_to_bytes(&self) -> StdResult<Vec<u8>>;
}

#[cw_serde]
pub struct InstantiateMsg {
    pub network: babylon_bitcoin::chain_params::Network,
    /// babylon_tag is a string encoding four bytes used for identification / tagging of the Babylon zone.
    /// NOTE: this is a hex string, not raw bytes
    pub babylon_tag: String,
    pub btc_confirmation_depth: u64,
    pub checkpoint_finalization_timeout: u64,
    // notify_cosmos_zone indicates whether to send Cosmos zone messages notifying BTC-finalised headers
    // NOTE: if set true, then the Cosmos zone needs to integrate the corresponding message handler as well
    pub notify_cosmos_zone: bool,
    /// If set, this will instantiate a BTC staking contract for BTC re-staking
    pub btc_staking_code_id: Option<u64>,
    /// If set, this will define the instantiate message for the BTC staking contract.
    /// This message is opaque to the Babylon contract, and depends on the specific staking contract
    /// being instantiated
    pub btc_staking_msg: Option<Binary>,
    /// If set, this will be the Wasm migration / upgrade admin of the BTC staking contract
    pub admin: Option<String>,
}

impl ContractMsg for InstantiateMsg {
    fn validate(&self) -> StdResult<()> {
        if self.babylon_tag.len() != BABYLON_TAG_BYTES * 2 {
            return Err(StdError::invalid_data_size(
                BABYLON_TAG_BYTES * 2,
                self.babylon_tag.len(),
            ));
        }
        let _ = self.babylon_tag_to_bytes()?;
        Ok(())
    }

    fn babylon_tag_to_bytes(&self) -> StdResult<Vec<u8>> {
        hex::decode(&self.babylon_tag).map_err(|_| {
            StdError::generic_err(format!(
                "babylon_tag is not a valid hex string: {}",
                self.babylon_tag
            ))
        })
    }
}

#[cw_serde]
pub enum ExecuteMsg {
    BtcHeaders {
        /// `headers` is a list of BTC headers. Typically:
        /// - A given delta of headers a user wants to add to the tip of, or fork the BTC chain.
        headers: Vec<BtcHeader>,
    },
    /// `slashing` is a slashing event from the BTC staking contract.
    ///
    /// This will be forwarded over IBC to the Babylon side for propagation to other Consumers, and
    /// Babylon itself
    Slashing {
        /// `fp_btc_pk` is the BTC PK of the slashed finality provider
        fp_btc_pk: Bytes,
        /// `block_height` is the Consumer blockchain slashing height
        block_height: u64,
        /// `secret_key` is the secret key extracted from the slashing evidence
        secret_key: Bytes,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Config returns the current configuration of the babylon-contract
    #[returns(Config)]
    Config {},
    /// BtcBaseHeader returns the base BTC header stored in the contract
    #[returns(BtcHeaderResponse)]
    BtcBaseHeader {},
    /// BtcTipHeader returns the tip BTC header stored in the contract
    #[returns(BtcHeaderResponse)]
    BtcTipHeader {},
    /// BtcHeader returns the BTC header information stored in the contract, by BTC height.
    #[returns(BtcHeaderResponse)]
    BtcHeader { height: u64 },
    /// BtcHeaderByHash returns the BTC header information stored in the contract, by BTC hash.
    ///
    /// `hash` is the (byte-reversed) hex-encoded hash of the BTC header
    #[returns(BtcHeaderResponse)]
    BtcHeaderByHash { hash: String },
    /// BtcHeaders returns the canonical BTC chain stored in the contract.
    ///
    /// `start_after` is the height of the header to start after, or `None` to start from the base
    #[returns(BtcHeadersResponse)]
    BtcHeaders {
        start_after: Option<u64>,
        limit: Option<u32>,
        reverse: Option<bool>,
    },
    /// BabylonBaseEpoch returns the base Babylon epoch stored in the contract
    #[returns(EpochResponse)]
    BabylonBaseEpoch {},
    /// BabylonLastEpoch returns the last babylon finalized epoch stored in the contract
    #[returns(EpochResponse)]
    BabylonLastEpoch {},
    /// BabylonEpoch returns the Babylon epoch stored in the contract, by epoch number.
    #[returns(EpochResponse)]
    BabylonEpoch { epoch_number: u64 },
    /// BabylonCheckpoint returns the Babylon checkpoint stored in the contract, by epoch number.
    #[returns(EpochResponse)]
    BabylonCheckpoint { epoch_number: u64 },
    /// CzLastHeader returns the last CZ epoch stored in the contract
    #[returns(CzHeaderResponse)]
    CzLastHeader {},
    /// CzHeader returns the CZ header stored in the contract, by CZ height.
    #[returns(CzHeaderResponse)]
    CzHeader { height: u64 },
}
