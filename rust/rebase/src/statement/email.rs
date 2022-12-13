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
#[ts(export, export_to = "bindings/statement/Email.ts")]
pub struct Email {
    pub email: String,
    pub subject: Subjects,
}

impl Statement for Email {
    fn generate_statement(&self) -> Result<String, StatementError> {
        Ok(format!(
            "{} is linked to the {} {}",
            self.email,
            self.subject.statement_title()?,
            self.subject.display_id()?
        ))
    }
}
