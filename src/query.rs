use cosmwasm_std::{ Deps, StdResult};

use crate::state::{STATE};
use crate::msg::GetCountResponse;

// Returns current State.count value wrapped in GetCountResponse type
pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetCountResponse { count: state.count })
}

// Returns true if State.count is 0
pub fn has_reset(deps: Deps) -> StdResult<bool> {
    let state = STATE.load(deps.storage)?;
    Ok( state.count == 0)
}
    
