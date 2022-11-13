use crate::state::{STATE};
use crate::error::ContractError;
use cosmwasm_std::{DepsMut, MessageInfo, Response, SubMsg, Coin, BankMsg};



pub fn execute_increment(deps: DepsMut, amount:Option<i32>) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        match amount {
            Some(amount) => state.count += amount,
            None => state.count += 1,
        }        
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "increment").add_attribute("amount", match amount {
        Some(amount) => amount.to_string(),
        None => "1".to_string(),
    }))

}


pub fn execute_decrement(deps: DepsMut, amount:Option<i32>) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        match amount {
            Some(amount) => state.count -= amount,
            None => state.count -= 1,
        }        
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "decrement").add_attribute("amount", match amount {
        Some(amount) => amount.to_string(),
        None => "1".to_string(),
    }))
}

pub fn execute_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("action", "reset"))
}

pub fn execute_send_fund(deps: DepsMut,_info:MessageInfo, recipient:String, coin:Coin) -> Result<Response, ContractError> {
    // validate addr
    let valid_addr = deps.api.addr_validate(&recipient)?;

    // we could check if recipient is the same as info.sender, no point in sending yourself some Coin

    // create a message to send some Coin
    let send_fund_msg: SubMsg = SubMsg::new(BankMsg::Send{amount:vec![coin], to_address:valid_addr.to_string()});
       
    Ok(Response::new().add_submessage(send_fund_msg))
}