use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use cw2::set_contract_version;

use crate::msg::{ExecuteMsg, InstantiateMsg};

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::SendData { binary, array } => {
            let bin = binary.unwrap_or_default();
            let bin_arr = bin.as_slice();
            let arr = array.unwrap_or_default();
            if bin_arr != arr.as_slice() {
                return Err(StdError::generic_err("Binary and array are not equal"));
            }
            Ok(Response::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::Binary;

    const CREATOR: &str = "creator";

    #[test]
    fn instantiate_works() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info(CREATOR, &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn execute_send_data_works() {
        let mut deps = mock_dependencies();
        let msg = ExecuteMsg::SendData {
            binary: Some(Binary(b"my binary data".to_vec())),
            array: Some(b"my binary data".to_vec()),
        };
        let res = execute(deps.as_mut(), mock_env(), msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn execute_send_data_empty() {
        let mut deps = mock_dependencies();
        let msg = ExecuteMsg::SendData {
            binary: None,
            array: None,
        };
        let res = execute(deps.as_mut(), mock_env(), msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn execute_send_data_no_binary() {
        let mut deps = mock_dependencies();
        let msg = ExecuteMsg::SendData {
            binary: None,
            array: Some(vec![]),
        };
        let res = execute(deps.as_mut(), mock_env(), msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn execute_send_data_no_array() {
        let mut deps = mock_dependencies();
        let msg = ExecuteMsg::SendData {
            binary: Some(vec![].into()),
            array: None,
        };
        let res = execute(deps.as_mut(), mock_env(), msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
    #[test]
    fn execute_send_data_errors() {
        let mut deps = mock_dependencies();
        let msg = ExecuteMsg::SendData {
            binary: Some(Binary(b"my binary data".to_vec())),
            array: Some(b"other binary data".to_vec()),
        };
        let err = execute(deps.as_mut(), mock_env(), msg).unwrap_err();
        assert_eq!(err, StdError::generic_err("Binary and array are not equal"));
    }
}
