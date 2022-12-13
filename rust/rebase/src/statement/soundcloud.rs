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
#[ts(export, export_to = "bindings/statement/SoundCloud.ts")]
pub struct SoundCloud {
    pub permalink: String,
    pub subject: Subjects,
}

impl Statement for SoundCloud {
    fn generate_statement(&self) -> Result<String, StatementError> {
        Ok(format!(
            "I am attesting that this SoundCloud profile https://soundcloud.com/{} is linked to the {} {}",
            self.permalink,
            self.subject.statement_title()?,
            self.subject.display_id()?
        ))
    }
}
