#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, QueryResultResponse};
use crate::state::RESULT_MAP;

const CONTRACT_NAME: &str = "crates.io:terra_host";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // Checks if the input address has made any IBC increment request to Host Contract
        QueryMsg::QueryResult { identifier } => query_result(deps, identifier)
    }
}

fn query_result(deps: Deps, identifier: String) -> StdResult<Binary> {
    let proof_result: String = RESULT_MAP.may_load(deps.storage, identifier)?.unwrap_or_default();
    to_json_binary(&QueryResultResponse { proof_result })
}

#[cfg(test)]
mod tests {}
