
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakerInfo {
    pub address: Addr,
    pub amount: Uint128,
    pub reward: Uint128,
    pub last_time: u64
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    /// Owner If None set, contract is frozen.
    pub owner: Option<Addr>,
    pub reward_token_address: Addr,
    pub stake_token_address: Addr,
    pub reward_amount: Uint128,
    pub stake_amount: Uint128,
    pub daily_reward_amount: Uint128,
    pub apy_prefix: Uint128,
    pub reward_interval: u64,
    pub lock_days: u64,
    pub enabled: bool
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct StakerListResponse {
    pub stakers: Vec<StakerInfo>,
}

/// Returns the vote (opinion as well as weight counted) as well as
/// the address of the voter who submitted it

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct StakerResponse {
    pub address: Addr,
    pub amount: Uint128,
    pub reward: Uint128,
    pub last_time: u64
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CountInfo {
    pub count: u128
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const STAKERS_KEY: &str = "stakers";
pub const STAKERS: Map<Addr, (Uint128, Uint128, u64)> = Map::new(STAKERS_KEY);

pub const UNSTAKING_KEY: &str = "unstaking";
pub const UNSTAKING: Map<Addr, Vec<(Uint128, u64)>> = Map::new(UNSTAKING_KEY);