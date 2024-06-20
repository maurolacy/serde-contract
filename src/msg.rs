use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    SendData {
        binary: Option<Binary>,
        array: Option<Vec<u8>>,
    },
}
