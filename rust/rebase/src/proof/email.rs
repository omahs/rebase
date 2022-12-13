use crate::{
    content::email::Email as Ctnt,
    statement::email::Email as Stmt,
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
#[ts(export, export_to = "bindings/proof/Email.ts")]
pub struct Email {
    pub auth: String,
    pub signature: String,
    pub statement: Stmt,
    pub timestamp: String,
}

impl Statement for Email {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<Ctnt> for Email {
    fn to_content(&self, statement: &str, signature: &str) -> Result<Ctnt, ProofError> {
        Ok(Ctnt {
            email: self.statement.email.clone(),
            subject: self.statement.subject.clone(),
            statement: statement.to_owned(),
            signature: signature.to_owned(),
        })
    }
}
