use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub xcall_address: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    SendPacket { message: Vec<u8> },
    ReceivePacket { message: Vec<u8> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(u64)]
    SequenceSend { port_id: String, channel_id: String },
}
