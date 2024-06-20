use cosmwasm_std::testing::message_info;
use cosmwasm_std::{Addr, ContractResult, Response};
use cosmwasm_vm::testing::{
    instantiate, mock_env, mock_instance, mock_instance_with_gas_limit, MockApi, MockQuerier,
    MockStorage,
};
use cosmwasm_vm::Instance;
use serde::msg::InstantiateMsg;

static WASM: &[u8] = include_bytes!("../target/wasm32-unknown-unknown/release/serde.wasm");

const CREATOR: &str = "creator";

#[track_caller]
#[allow(dead_code)]
fn setup() -> Instance<MockApi, MockStorage, MockQuerier> {
    let mut deps = mock_instance_with_gas_limit(WASM, 1_000_000_000);
    let msg = InstantiateMsg {};
    let info = message_info(&Addr::unchecked(CREATOR), &[]);
    let res: Response = instantiate(&mut deps, mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    deps
}

#[test]
fn instantiate_works() {
    let mut deps = mock_instance(WASM, &[]);

    let msg = InstantiateMsg {};
    let info = message_info(&Addr::unchecked(CREATOR), &[]);
    let res: ContractResult<Response> = instantiate(&mut deps, mock_env(), info, msg);
    let msgs = res.unwrap().messages;
    assert_eq!(0, msgs.len());
}
