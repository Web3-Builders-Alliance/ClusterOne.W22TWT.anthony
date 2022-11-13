#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, CosmosMsg, BankMsg, Coin, Uint128, SubMsg, coins,   };
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, MigrateMsg};
use crate::state::{State, STATE};

use crate::execute::{execute_decrement, execute_increment, execute_reset, execute_send_fund};
use crate::query::{count, has_reset};

//  version info for migration info
const CONTRACT_NAME: &str = "crates.io:{{project-name}}";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => execute_increment(deps, None),
        ExecuteMsg::Decrement {} => execute_decrement(deps, None),
        ExecuteMsg::DecrementBy { amount} => execute_decrement(deps, Some(amount)),
        ExecuteMsg::IncrementBy {amount} => execute_increment(deps, Some(amount)),
        ExecuteMsg::Reset { count } => execute_reset(deps, info, count),
        ExecuteMsg::AddCosmosMsg {  } => {
            return Ok(Response::new().add_message(CosmosMsg::Bank(BankMsg::Send {
                to_address: "cosmos1ghekyjucln7y67ntx7cf27m9dpuxxemn4c8gza".to_string(),
                amount: vec![Coin {
                    denom: "uatom".to_string(),
                    amount: Uint128::from(1000000u128),
                }],
            })))
        },
        ExecuteMsg::AddSubMsg {  } => {
            let msg = BankMsg::Send { to_address: String::from("you"), amount: coins(1015, "earth") };
            Ok(Response::new().add_submessage(SubMsg::reply_on_success(msg,  0)))     
        },


        ExecuteMsg::SendFund { recipient, coin }=> execute_send_fund(deps, info, recipient, coin),
    }
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&count(deps)?),
        QueryMsg::HasReset {} => to_binary(&has_reset(deps)?),
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}