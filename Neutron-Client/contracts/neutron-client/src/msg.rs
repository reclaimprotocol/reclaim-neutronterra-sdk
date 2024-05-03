use schemars::JsonSchema;
use cosmwasm_schema::{cw_serde, QueryResponses};
use serde::{Deserialize, Serialize};
mod claims;
use claims::Proof;
#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    VerifyProof {
        channel: String,
        proof: ProofMsg,
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryResultResponse)]
    QueryResult {
        identifier: String
    }
}

#[cw_serde]
pub enum IbcExecuteMsg {
    SendProof {
        proof: ProofMsg
    },
}

#[cw_serde]
pub struct QueryResultResponse {
    pub proof_result: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProofMsg {
    pub proof: Proof,
}