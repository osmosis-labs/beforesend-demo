#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:beforesend";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[entry_point]
pub fn sudo(_deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::BlockBeforeSend { from, to, amount } => {
            // return error if any coin amount is 100
            if amount.amount == Uint128::from(100u64) {
                return Err(ContractError::CustomError {
                    val: String::from("Invalid Send Amount"),
                });
            }

            Ok(Response::new())
        }
        SudoMsg::TrackBeforeSend { from, to, amount } => {
            // pass on track before send for now
            Ok(Response::new())
        }
        
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {}
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins, Coin};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn beforesend() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Make sure a non 100 token send passes
        let msg = SudoMsg::BlockBeforeSend {
            from: String::from("addr0"),
            to: String::from("addr1"),
            amount: coin(1, "uosmo"),
        };
        sudo(deps.as_mut(), mock_env(), msg).unwrap();

        // Make sure a non 100 token send fails
        let msg = SudoMsg::BlockBeforeSend {
            from: String::from("addr0"),
            to: String::from("addr1"),
            amount: coin(100, "uosmo"),
        };
        sudo(deps.as_mut(), mock_env(), msg).unwrap_err();

        // Make sure a non 100 token send fails
        let msg = SudoMsg::BlockBeforeSend {
            from: String::from("addr0"),
            to: String::from("addr1"),
            amount: coin(100, "uosmo"),
        };
        sudo(deps.as_mut(), mock_env(), msg).unwrap_err();

        // Track before send should return ok value
        let msg = SudoMsg::TrackBeforeSend {
            from: String::from("addr0"),
            to: String::from("addr1"),
            amount: coin(100, "uosmo"),
        };
        sudo(deps.as_mut(), mock_env(), msg).unwrap();
    }
}
