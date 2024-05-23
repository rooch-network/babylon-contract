use cosmwasm_schema::{cw_serde, QueryResponses};

use babylon_apis::btc_staking_api::{ActiveBtcDelegation, FinalityProvider};

use crate::state::{Config, Params};

#[cw_serde]
#[derive(Default)]
pub struct InstantiateMsg {
    pub params: Option<Params>,
}

pub type ExecuteMsg = babylon_apis::btc_staking_api::ExecuteMsg;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// `Config` returns the current configuration of the btc-staking contract
    #[returns(Config)]
    Config {},
    /// `Params` returns the current Consumer-specific parameters of the btc-staking contract
    #[returns(Params)]
    Params {},
    /// `FinalityProvider` returns the finality provider by its BTC public key, in hex format
    #[returns(FinalityProvider)]
    FinalityProvider { btc_pk_hex: String },
    /// `FinalityProviders` returns the list of registered finality providers
    ///
    /// `start_after` is the BTC public key of the FP to start after, or `None` to start from the beginning
    #[returns(FinalityProvidersResponse)]
    FinalityProviders {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// `Delegation` returns delegation information by its staking tx hash, in hex format
    #[returns(ActiveBtcDelegation)]
    Delegation { staking_tx_hash_hex: String },
    /// `Delegations` return the list of delegations
    ///
    /// `start_after` is the staking tx hash (in hex format) of the delegation to start after,
    /// or `None` to start from the beginning.
    /// `limit` is the maximum number of delegations to return.
    /// `active` is an optional filter to return only active delegations
    #[returns(BtcDelegationsResponse)]
    Delegations {
        start_after: Option<String>,
        limit: Option<u32>,
        active: Option<bool>,
    },
    /// `DelegationsByFP` returns the list of staking tx hashes (in hex format) corresponding to
    /// delegations, for a given finality provider.
    ///
    /// `btc_pk_hex` is the BTC public key of the finality provider, in hex format.
    /// The hashes are returned in hex format
    //TODO?: Support pagination
    #[returns(DelegationsByFPResponse)]
    DelegationsByFP { btc_pk_hex: String },
    /// `FinalityProviderPower` returns the finality provider aggregated power by its BTC public key,
    /// in hex format
    #[returns(FinalityProviderInfo)]
    FinalityProviderInfo { btc_pk_hex: String },
    /// `FinalityProvidersByPower` returns the list of finality providers sorted by their aggregated
    /// power, in descending order
    ///
    /// `start_after` is the BTC public key of the FP to start after, or `None` to start from the top
    #[returns(FinalityProvidersByPowerResponse)]
    FinalityProvidersByPower {
        start_after: Option<FinalityProviderInfo>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct FinalityProvidersResponse {
    pub fps: Vec<FinalityProvider>,
}

#[cw_serde]
pub struct BtcDelegationsResponse {
    pub delegations: Vec<ActiveBtcDelegation>,
}

#[cw_serde]
pub struct DelegationsByFPResponse {
    pub hashes: Vec<String>,
}

#[cw_serde]
pub struct FinalityProvidersByPowerResponse {
    pub fps: Vec<FinalityProviderInfo>,
}

#[cw_serde]
pub struct FinalityProviderInfo {
    /// btc_pk_hex is the Bitcoin secp256k1 PK of this finality provider
    /// the PK follows encoding in BIP-340 spec in hex format
    pub btc_pk_hex: String,
    /// `power` is the aggregated power of this finality provider.
    /// The power is calculated based on the amount of BTC delegated to this finality provider
    pub power: u64,
}
