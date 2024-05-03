#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_json, DepsMut, Env, IbcBasicResponse, IbcChannel, IbcChannelCloseMsg, 
    IbcChannelConnectMsg, IbcChannelOpenMsg, IbcChannelOpenResponse, IbcOrder, IbcPacketAckMsg, 
    IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse
};
use crate::error::{ContractError, Never};
use crate::msg::{IbcExecuteMsg, ProofMsg};

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
    _deps: DepsMut,
    _env: Env,
    msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, Never> {
    let msg = &msg.packet.data.to_string();
    Ok(IbcReceiveResponse::new()
        .add_attribute("method", "ibc_packet_receive")
        .add_attribute("result", msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_ack(  
    _deps: DepsMut,
    _env: Env,
    msg: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
    let original_packet: IbcExecuteMsg = from_json(&msg.original_packet.data)?;
    
    match original_packet {
        IbcExecuteMsg::SendProof { proof } => after_ibc_acknowledgement(proof)
    }
}

fn after_ibc_acknowledgement(proof: ProofMsg) -> Result<IbcBasicResponse, ContractError> {
    Ok(IbcBasicResponse::new()
        .add_attribute("proof", proof.proof.signedClaim.claim.identifier)
        .add_attribute("method", "after_ibc_acknowledgement"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_timeout(
    _deps: DepsMut,
    _env: Env,
    _msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
    Ok(IbcBasicResponse::new().add_attribute("method", "ibc_packet_timeout"))
}
