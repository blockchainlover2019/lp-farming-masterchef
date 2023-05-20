use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw20::{Cw20ReceiveMsg};
use cosmwasm_std::{Uint128, Addr};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    /// Owner if none set to info.sender.
    pub owner: Option<String>,
    pub reward_token_address: Addr,
    pub stake_token_address: Addr,
    pub daily_reward_amount: Uint128,
    pub apy_prefix: Uint128,
    pub reward_interval: u64,
    pub lock_days: u64,
    pub enabled: bool
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ConfigResponse {
    pub owner: Option<String>,
    pub reward_token_address: String,
    pub stake_token_address: String,
    pub reward_amount: Uint128,
    pub stake_amount: Uint128,
    pub daily_reward_amount: Uint128,
    pub apy_prefix: Uint128,
    pub reward_interval: u64,
    pub lock_days: u64

}