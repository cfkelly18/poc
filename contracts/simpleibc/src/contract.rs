use crate::state::{Config, State, CONFIG, STATE};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, SubMsg,
    SubMsgResponse, SubMsgResult,
};
use cw2::set_contract_version;
use osmosis_std::types::cosmos::base::v1beta1::Coin;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgBurn, MsgCreateDenom, MsgMint};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:simpleibc";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const CREATE_TOKEN_REPLY_ID: u64 = 1;

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let contract_addr = env.contract.address.to_string();
    let state = State { tx_count: 0 };
    let config = Config {
        denom: msg.denom,
        channel: msg.channel,
        min_amount: msg.min_amount,
        dest_address: info.sender.to_string(),
    };

    // init new TF denom
    let subdenom = "simple".to_string();
    let sender = env.contract.address.clone().into();
    let msg_create_denom: CosmosMsg = MsgCreateDenom { sender, subdenom }.into();
    let mut msgs = vec![SubMsg::reply_always(
        msg_create_denom,
        CREATE_TOKEN_REPLY_ID,
    )];

    //TODO: Need to update initial address
    let mint_msg = MsgMint {
        sender: env.contract.address.into(),
        amount: Some(Coin {
            denom: format!("factory/{contract_addr}/simple"), //TODO: dynamic
            amount: "1000000000".to_string(),
        }),
        mint_to_address: contract_addr,
    };
    msgs.push(SubMsg::new(mint_msg));

    CONFIG.save(deps.storage, &config)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_submessages(msgs)
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

/// Handling contract migration
/// To make a contract migratable, you need
/// - this entry_point implemented
/// - only contract admin can migrate, so admin has to be set at contract initiation time
/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    match msg {
        // Find matched incoming message variant and execute them with your custom logic.
        //
        // With `Response` type, it is possible to dispatch message to invoke external logic.
        // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    }
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // Find matched incoming message variant and execute them with your custom logic.
        //
        // With `Response` type, it is possible to dispatch message to invoke external logic.
        // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // Find matched incoming message variant and query them your custom logic
        // and then construct your query response with the type usually defined
        // `msg.rs` alongside with the query message itself.
        //
        // use `cosmwasm_std::to_binary` to serialize query response to json binary.
    }
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    if msg.id == CREATE_TOKEN_REPLY_ID {
        if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), .. }) = msg.result {
            todo!()
        }
    };

    Ok(Response::new())
}
