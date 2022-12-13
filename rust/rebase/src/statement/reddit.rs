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
#[ts(export, export_to = "bindings/statement/Reddit.ts")]
pub struct Reddit {
    pub handle: String,
    pub subject: Subjects,
}

impl Statement for Reddit {
    fn generate_statement(&self) -> Result<String, StatementError> {
        Ok(format!(
            "I am attesting that this Reddit handle {} is linked to the {} {}",
            self.handle,
            self.subject.statement_title()?,
            self.subject.display_id()?
        ))
    }
}
