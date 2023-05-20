#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
    WasmMsg, WasmQuery, QueryRequest,Order, Addr, Storage
};
use cw2::{get_contract_version, set_contract_version};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg, Cw20QueryMsg};
use cw20::{TokenInfoResponse};
use cw_utils::{maybe_addr};
use cw_storage_plus::Bound;

use crate::error::CustomError;
use crate::state::{
    Config, CONFIG
};
use crate::msg::{ InstantiateMsg, ExecuteMsg, QueryMsg, ConfigResponse };

const CONTRACT_NAME: &str = "lp-staking";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  let owner = msg.owner.map_or(Ok(info.sender), |o| deps.api.addr_validate(&o))?;
  let config = Config {
    owner: Some(owner),
    reward_token_address: msg.reward_token_address,
    stake_token_address: msg.stake_token_address,
    reward_amount: Uint128::zero(),
    stake_amount: Uint128::zero(),
    daily_reward_amount: msg.daily_reward_amount,
    apy_prefix: msg.apy_prefix,
    reward_interval: msg.reward_interval,
    lock_days: msg.lock_days,
    enabled: true
  };
  CONFIG.save(deps.storage, &config)?;
  Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, CustomError> {
  Err(CustomError::Unauthorized {})
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
  to_binary(&query_config(deps)?)
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
  let cfg = CONFIG.load(deps.storage)?;
  Ok(ConfigResponse {
      owner: cfg.owner.map(|o| o.into()),
      reward_token_address: cfg.reward_token_address.into(),
      stake_token_address: cfg.stake_token_address.into(),
      reward_amount: cfg.reward_amount,
      stake_amount: cfg.stake_amount,
      daily_reward_amount: cfg.daily_reward_amount,
      apy_prefix: cfg.apy_prefix,
      reward_interval: cfg.reward_interval,
      lock_days: cfg.lock_days
  })
}
