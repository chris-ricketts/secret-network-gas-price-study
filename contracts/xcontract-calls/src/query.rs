use cosmwasm_std::{Api, Binary, Extern, Querier, QueryResult, StdResult, Storage};
use schemars::JsonSchema;

use crate::state;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, JsonSchema)]
pub enum QueryMsg {
    Load { key: Binary },
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, JsonSchema)]
pub struct QueryAnswer {
    pub key: Binary,
    pub value: Option<Binary>,
}

impl secret_toolkit::utils::Query for QueryMsg {
    const BLOCK_SIZE: usize = crate::consts::BLOCK_SIZE;
}

pub fn query<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>, msg: QueryMsg) -> QueryResult {
    let answer = match msg {
        QueryMsg::Load { key } => load(deps, key)?,
    };

    let response = cosmwasm_std::to_binary(&answer);

    secret_toolkit::utils::pad_query_result(response, crate::consts::BLOCK_SIZE)
}

fn load<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    key: Binary,
) -> StdResult<QueryAnswer> {
    let value = state::general_storage().may_load(&deps.storage, key.as_slice())?;
    Ok(QueryAnswer { key, value })
}
