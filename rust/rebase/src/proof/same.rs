use crate::{
    content::same::Same as Ctnt,
    statement::same::Same as Stmt,
    types::{
        error::{ProofError, StatementError},
        types::{Proof, Statement},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[serde(rename = "proof")]
#[ts(export, export_to = "bindings/proof/Same.ts")]
pub struct Same {
    pub statement: Stmt,
    pub signature1: String,
    pub signature2: String,
}

impl Statement for Same {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<Ctnt> for Same {
    fn to_content(&self, _statement: &str, _signature: &str) -> Result<Ctnt, ProofError> {
        Ok(Ctnt {
            id1: self.statement.id1.clone(),
            id2: self.statement.id2.clone(),
            statement: self.generate_statement()?,
            signature1: self.signature1.clone(),
            signature2: self.signature2.clone(),
        })
    }
}
