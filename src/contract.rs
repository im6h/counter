#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{CounterResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

const CONTRACT_NAME: &str = "crates.io:counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ZERO_CODE: i32 = 0;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let state = State { counter: 0 };

    STATE.save(deps.storage, &state)?;
    Ok(Response::new().add_attribute("counter", ZERO_CODE.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Update {} => try_update_counter(deps),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Counter {} => query_counter(deps, env),
    }
}

pub fn try_update_counter(deps: DepsMut) -> Result<Response, ContractError> {
    let current_state = STATE.load(deps.storage)?;
    let mut current_counter = current_state.counter;

    current_counter += 1;

    let new_state = State {
        counter: current_counter,
    };
    STATE.save(deps.storage, &new_state)?;
    Ok(Response::new().add_attribute("counter", current_counter.to_string()))
}

pub fn query_counter(deps: Deps, _env: Env) -> StdResult<Binary> {
    let current_state = STATE.load(deps.storage)?;
    let counter = current_state.counter;

    let resp = to_binary(&CounterResponse { counter }).unwrap();
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::attr;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use super::{
        execute, instantiate, query, to_binary, CounterResponse, ExecuteMsg, InstantiateMsg,
        QueryMsg, STATE, ZERO_CODE,
    };

    const ADDR1: &str = "addr1";

    // instantiate with provide admin address
    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);

        let msg = InstantiateMsg {};
        let resp = instantiate(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(
            resp.attributes,
            vec![attr("counter", ZERO_CODE.to_string())]
        )
    }

    #[test]
    fn test_execute() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        let expect_number = 1;

        // instantiate msg
        let msg = InstantiateMsg {};
        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // execute first time
        let msg = ExecuteMsg::Update {};
        let _resp = execute(deps.as_mut(), env, info, msg);
        let current_state = STATE.load(deps.as_mut().storage).unwrap();
        println!("Execute twice!");
        assert_eq!(current_state.counter, expect_number);
    }

    #[test]
    fn test_query() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        let expect_data_0 = to_binary(&CounterResponse { counter: 0 }).unwrap();
        let expect_data_1 = to_binary(&CounterResponse { counter: 1 }).unwrap();

        let msg = InstantiateMsg {};
        let _resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // query one time.
        let msg = QueryMsg::Counter {};
        let resp = query(deps.as_ref(), env.clone(), msg).unwrap();
        assert_eq!(resp, expect_data_0);

        // query two time
        let msg = ExecuteMsg::Update {};
        let _resp = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        let msg = QueryMsg::Counter {};
        let resp = query(deps.as_ref(), env, msg).unwrap();
        assert_eq!(resp, expect_data_1);
    }
}
