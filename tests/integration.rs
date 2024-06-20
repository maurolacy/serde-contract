use cosmwasm_std::testing::message_info;
use cosmwasm_std::{Addr, Binary, ContractResult, Empty, Response};
use cosmwasm_vm::testing::{
    execute, instantiate, mock_env, mock_instance, mock_instance_with_gas_limit, MockApi,
    MockQuerier, MockStorage,
};
use cosmwasm_vm::Instance;
use ser_de::msg::{ExecuteMsg, InstantiateMsg};

static WASM: &[u8] = include_bytes!("../target/wasm32-unknown-unknown/release/ser_de.wasm");

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

#[test]
fn execute_send_data_works() {
    let mut deps = mock_instance(WASM, &[]);

    let msg = ExecuteMsg::SendData {
        binary: Some(Binary::new(b"my binary data".to_vec())),
        array: Some(b"my binary data".to_vec()),
    };
    let mock_info = message_info(&Addr::unchecked(CREATOR), &[]);
    let res: Response =
        execute(&mut deps, cosmwasm_std::testing::mock_env(), mock_info, msg).unwrap();
    assert_eq!(0, res.messages.len());
}

#[test]
fn execute_send_data_empty() {
    let mut deps = mock_instance(WASM, &[]);

    let msg = ExecuteMsg::SendData {
        binary: None,
        array: None,
    };
    let mock_info = message_info(&Addr::unchecked(CREATOR), &[]);
    let res: Response =
        execute(&mut deps, cosmwasm_std::testing::mock_env(), mock_info, msg).unwrap();
    assert_eq!(0, res.messages.len());
}

#[test]
fn execute_send_data_no_binary() {
    let mut deps = mock_instance(WASM, &[]);

    let msg = ExecuteMsg::SendData {
        binary: None,
        array: Some(vec![]),
    };
    let mock_info = message_info(&Addr::unchecked(CREATOR), &[]);
    let res: Response =
        execute(&mut deps, cosmwasm_std::testing::mock_env(), mock_info, msg).unwrap();
    assert_eq!(0, res.messages.len());
}

#[test]
fn execute_send_data_no_array() {
    let mut deps = mock_instance(WASM, &[]);

    let msg = ExecuteMsg::SendData {
        binary: Some(vec![].into()),
        array: None,
    };
    let mock_info = message_info(&Addr::unchecked(CREATOR), &[]);
    let res: Response =
        execute(&mut deps, cosmwasm_std::testing::mock_env(), mock_info, msg).unwrap();
    assert_eq!(0, res.messages.len());
}

#[test]
fn execute_send_data_errors() {
    let mut deps = mock_instance(WASM, &[]);

    let msg = ExecuteMsg::SendData {
        binary: Some(Binary::new(b"my binary data".to_vec())),
        array: Some(b"other binary data".to_vec()),
    };
    let mock_info = message_info(&Addr::unchecked(CREATOR), &[]);
    let err =
        execute::<_, _, _, _, Empty>(&mut deps, cosmwasm_std::testing::mock_env(), mock_info, msg)
            .unwrap_err();
    assert_eq!(err, "Generic error: Binary and array are not equal");
}
