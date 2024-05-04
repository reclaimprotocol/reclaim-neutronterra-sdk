#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_json, Coin, DepsMut, Env, IbcBasicResponse, IbcChannel, IbcChannelCloseMsg, IbcChannelConnectMsg, 
    IbcChannelOpenMsg, IbcChannelOpenResponse, IbcOrder, IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse
};
use crate::error::{ContractError, Never};
use crate::msg::{ExecuteMsg, IbcExecuteMsg, ProofMsg};
use crate::ack::{make_ack_fail, make_ack_success};
use crate::state::{RESULT_MAP, VERIFICATION_CONTRACT};

pub const IBC_VERSION: &str = "ibc-cosmwasm";

pub fn validate_order_and_version(
    channel: &IbcChannel,
    counterparty_version: Option<&str>,
) -> Result<(), ContractError> {
    if channel.order != IbcOrder::Unordered {
        return Err(ContractError::OrderedChannel {});
    }

    if channel.version != IBC_VERSION {
        return Err(ContractError::InvalidVersion {
            actual: channel.version.to_string(),
            expected: IBC_VERSION.to_string(),
        });
    }

    if let Some(counterparty_version) = counterparty_version {
        if counterparty_version != IBC_VERSION {
            return Err(ContractError::InvalidVersion {
                actual: counterparty_version.to_string(),
                expected: IBC_VERSION.to_string(),
            });
        }
    }
    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_open(
    _deps: DepsMut,
    _env: Env,
    msg: IbcChannelOpenMsg,
) -> Result<IbcChannelOpenResponse, ContractError> {
    validate_order_and_version(msg.channel(), msg.counterparty_version())?;
    Ok(None)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_connect(
    _deps: DepsMut,
    _env: Env,
    msg: IbcChannelConnectMsg,
) -> Result<IbcBasicResponse, ContractError> {
    validate_order_and_version(msg.channel(), msg.counterparty_version())?;

    Ok(IbcBasicResponse::new()
        .add_attribute("method", "ibc_channel_connect"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_close(
    _deps: DepsMut,
    _env: Env,
    msg: IbcChannelCloseMsg,
) -> Result<IbcBasicResponse, ContractError> {
    let channel = msg.channel().endpoint.channel_id.clone();

    Ok(IbcBasicResponse::new()
        .add_attribute("method", "ibc_channel_close")
        .add_attribute("channel", channel))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_receive(
    deps: DepsMut,
    env: Env,
    msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, Never> {
    match do_ibc_packet_receive(deps, env, msg) {
        Ok(response) => Ok(response),
        Err(error) => Ok(IbcReceiveResponse::new()
            .add_attribute("method", "ibc_packet_receive")
            .add_attribute("error", error.to_string())
            .set_ack(make_ack_fail(error.to_string()))),
    }
}

pub fn do_ibc_packet_receive(
    deps: DepsMut,
    env: Env,
    msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, ContractError> {
    let channel_id = &msg.packet.dest.channel_id;
    let msg: IbcExecuteMsg = from_json(&msg.packet.data)?;

    match msg {
        IbcExecuteMsg::SendProof { proof } => execute_ibc_method(deps, env, proof, channel_id)
    }
}

fn execute_ibc_method(deps:DepsMut, _env: Env, proof: ProofMsg, _channel_id: &String) -> Result<IbcReceiveResponse, ContractError> {

    let exec_msg = ExecuteMsg::VerifyProof(proof.clone());

    //Fail by default, will become Success when cosmos_msg turns out Ok
    RESULT_MAP.save(deps.storage, proof.proof.signedClaim.claim.identifier.clone(), &String::from("Fail"))?;
    let cosmos_msg = cosmwasm_std::wasm_execute(
        &String::from(VERIFICATION_CONTRACT),
        &exec_msg,
        vec![Coin::new(0, "untrn")]);

    match cosmos_msg {
        Ok(wasm_msg) => {
            RESULT_MAP.save(deps.storage, proof.proof.signedClaim.claim.identifier.clone(), &String::from("Success"))?;
            Ok(IbcReceiveResponse::new()
            .add_message(wasm_msg)
            .add_attribute("method", "execute_ibc_method")
            .set_ack(make_ack_success(String::from("Success"))))
        },
        Err(error) => {
            Ok(IbcReceiveResponse::new()
            .add_attribute("method", "execute_ibc_method")
            .set_ack(make_ack_success(String::from(error.to_string()))))
        }
    }

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_ack(
    _deps: DepsMut,
    _env: Env,
    _ack: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
    Ok(IbcBasicResponse::new().add_attribute("method", "ibc_packet_ack"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_timeout(
    _deps: DepsMut,
    _env: Env,
    _msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
    Ok(IbcBasicResponse::new().add_attribute("method", "ibc_packet_timeout"))
}