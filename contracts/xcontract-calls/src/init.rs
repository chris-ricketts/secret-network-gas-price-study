use cosmwasm_std::{Api, Env, Extern, HumanAddr, InitResponse, InitResult, Querier, Storage};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state;

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct InitMsg {
    // forwarding contract address and code hash
    // if this is present then this contract should delegate
    // to the forwarding address contract
    forwarding_address: Option<(HumanAddr, String)>,
}

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: InitMsg,
) -> InitResult {
    if let Some(forwarding_addr) = msg.forwarding_address {
        state::forwarding_addr().save(&mut deps.storage, &forwarding_addr)?;
    }

    Ok(InitResponse::default())
}
