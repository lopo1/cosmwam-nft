
mod error;
mod execute;
pub mod helpers;
pub mod msg;
mod query;
pub mod state;
mod tests;

use schemars::JsonSchema;
pub use crate::error::ContractError;
pub use crate::msg::{ExecuteMsg, InstantiateMsg, MintMsg, MinterResponse, QueryMsg};
pub use crate::state::Cw721Contract;
use cosmwasm_std::Empty;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

pub type Extension = Option<String>;
pub type Cw721MetadataContract<'a> = Cw721Contract<'a, Extension, Empty>;
// pub type ExecuteMsg = ExecuteMsg<Extension>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    // This is a simple type to let us handle empty extensions

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        Cw721MetadataContract::default().instantiate(deps, env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension>,
    ) -> Result<Response, ContractError> {
        Cw721MetadataContract::default().execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Cw721MetadataContract::default().query(deps, env, msg)
    }
}
