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
#[ts(export, export_to = "bindings/statement/GitHub.ts")]
pub struct GitHub {
    pub handle: String,
    pub subject: Subjects,
}

impl Statement for GitHub {
    fn generate_statement(&self) -> Result<String, StatementError> {
        Ok(format!(
            "I am attesting that this GitHub handle {} is linked to the {} {}",
            self.handle,
            self.subject.statement_title()?,
            self.subject.display_id()?
        ))
    }
}
