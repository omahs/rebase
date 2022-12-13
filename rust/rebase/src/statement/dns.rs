use crate::types::{
    enums::subject::Subjects,
    error::StatementError,
    types::{Statement, Subject},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[serde(rename = "statement")]
#[ts(export, export_to = "bindings/statement/Dns.ts")]
pub struct Dns {
    pub domain: String,
    pub prefix: String,
    pub subject: Subjects,
}

impl Statement for Dns {
    fn generate_statement(&self) -> Result<String, StatementError> {
        Ok(format!(
            "{} is linked to {}",
            self.domain,
            self.subject.display_id()?
        ))
    }
}
