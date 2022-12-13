use crate::{
    content::github::GitHub as Ctnt,
    statement::github::GitHub as Stmt,
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
#[ts(export, export_to = "bindings/proof/GitHub.ts")]
pub struct GitHub {
    pub gist_id: String,
    pub statement: Stmt,
}

impl Statement for GitHub {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<Ctnt> for GitHub {
    fn to_content(&self, statement: &str, signature: &str) -> Result<Ctnt, ProofError> {
        Ok(Ctnt {
            gist_id: self.gist_id.clone(),
            handle: self.statement.handle.clone(),
            subject: self.statement.subject.clone(),
            statement: statement.to_owned(),
            signature: signature.to_owned(),
        })
    }
}
