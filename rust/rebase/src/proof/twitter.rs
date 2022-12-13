use crate::{
    content::twitter::Twitter as Ctnt,
    statement::twitter::Twitter as Stmt,
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
#[ts(export, export_to = "bindings/proof/Twitter.ts")]
pub struct Twitter {
    pub statement: Stmt,
    pub tweet_url: String,
}

impl Statement for Twitter {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<Ctnt> for Twitter {
    fn to_content(&self, statement: &str, signature: &str) -> Result<Ctnt, ProofError> {
        Ok(Ctnt {
            handle: self.statement.handle.clone(),
            subject: self.statement.subject.clone(),
            tweet_url: self.tweet_url.to_owned(),
            statement: statement.to_owned(),
            signature: signature.to_owned(),
        })
    }
}
