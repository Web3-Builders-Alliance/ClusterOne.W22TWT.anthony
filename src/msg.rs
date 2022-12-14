use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Decrement{},
    DecrementBy {amount:i32},
    IncrementBy {amount:i32},
    SendFund {recipient: String, coin: Coin},
    
    // more examples
    AddCosmosMsg{},
    AddSubMsg{},
    // UpdateState {new_value:i32}, does the same thing as Reset ?!
    Reset { count: i32 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
    
    #[returns(bool)]
    HasReset {},
}

#[cw_serde]
pub struct MigrateMsg {}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}
