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
pub fn execute(_deps: DepsMut, _env: Env, msg: ExecuteMsg) -> StdResult<Response> {
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

    const CREATOR: &str = "creator";

    #[test]
    fn instantiate_works() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info(CREATOR, &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
}
