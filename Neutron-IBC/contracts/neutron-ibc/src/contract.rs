#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo, Response};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, IbcExecuteMsg };
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:neutron-ibc";
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
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::VerifyProof { channel, proof } => Ok(Response::new()
            .add_attribute("method", "execute_query")
            .add_attribute("channel", channel.clone())
            .add_message(IbcMsg::SendPacket {
                channel_id: channel,
                data: to_json_binary(&IbcExecuteMsg::SendProof { proof })?, 
                timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(120)),
            })
        ),
    }
}

#[cfg(test)]
mod tests {}
