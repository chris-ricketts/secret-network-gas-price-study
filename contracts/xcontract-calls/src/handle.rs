use cosmwasm_std::{
    Api, Binary, Env, Extern, HandleResponse, HandleResult, HumanAddr, Querier, StdResult, Storage,
};
use schemars::JsonSchema;
use secret_toolkit::utils::{HandleCallback, Query};

use crate::{query, state};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Store { key: Binary, value: Binary },
    // test the gas usage of x-contract vs in-contract storage queries
    Read { key: Binary },
}

impl secret_toolkit::utils::HandleCallback for HandleMsg {
    const BLOCK_SIZE: usize = crate::consts::BLOCK_SIZE;
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: HandleMsg,
) -> HandleResult {
    let fw_addr = state::forwarding_addr().may_load(&deps.storage)?;

    let response = match (msg, fw_addr) {
        (HandleMsg::Store { key, value }, None) => store(deps, key, value)?,
        (HandleMsg::Read { key }, Some((fw_addr, code_hash))) => {
            x_contract_read(deps, key, fw_addr, code_hash)?
        }
        (HandleMsg::Read { key }, None) => in_contract_read(deps, key)?,
        (msg, Some((fw_addr, code_hash))) => return forward_message(fw_addr, code_hash, msg),
    };

    secret_toolkit::utils::pad_handle_result(Ok(response), crate::consts::BLOCK_SIZE)
}

fn forward_message(fw_addr: HumanAddr, code_hash: String, msg: HandleMsg) -> HandleResult {
    let fw_msg = msg.to_cosmos_msg(code_hash, fw_addr, None)?;
    let response = HandleResponse {
        messages: vec![fw_msg],
        ..Default::default()
    };
    secret_toolkit::utils::pad_handle_result(Ok(response), crate::consts::BLOCK_SIZE)
}

pub fn store<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    key: Binary,
    value: Binary,
) -> StdResult<HandleResponse> {
    state::general_storage().save(&mut deps.storage, key.as_slice(), &value)?;
    Ok(HandleResponse::default())
}

fn x_contract_read<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    key: Binary,
    fw_addr: HumanAddr,
    code_hash: String,
) -> StdResult<HandleResponse> {
    let answer: query::QueryAnswer =
        query::QueryMsg::Load { key }.query(&deps.querier, code_hash, fw_addr)?;
    let data = Some(cosmwasm_std::to_binary(&answer.value)?);
    Ok(HandleResponse {
        data,
        ..Default::default()
    })
}

fn in_contract_read<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    key: Binary,
) -> StdResult<HandleResponse> {
    let answer = state::general_storage().may_load(&deps.storage, key.as_slice())?;
    let data = Some(cosmwasm_std::to_binary(&answer)?);
    Ok(HandleResponse {
        data,
        ..Default::default()
    })
}
