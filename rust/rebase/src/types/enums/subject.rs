use crate::subject::{ed25519::DidWeb as Ed25519, ethereum::Eip155, solana::Solana};
use crate::types::{error::SubjectError, types::Subject};

use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "subject")]
pub enum Subjects {
    #[serde(rename = "pkh")]
    Pkh(Pkh),
    #[serde(rename = "web")]
    Web(Web),
}

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "pkh")]
pub enum Pkh {
    #[serde(rename = "eip155")]
    Eip155(Eip155),
    #[serde(rename = "solana")]
    Solana(Solana),
}

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "web")]
pub enum Web {
    #[serde(rename = "ed25519")]
    Ed25519(Ed25519),
}

#[async_trait(?Send)]
impl Subject for Subjects {
    fn did(&self) -> Result<String, SubjectError> {
        match &self {
            Subjects::Pkh(Pkh::Eip155(x)) => x.did(),
            Subjects::Pkh(Pkh::Solana(x)) => x.did(),
            Subjects::Web(Web::Ed25519(x)) => x.did(),
        }
    }

    fn display_id(&self) -> Result<String, SubjectError> {
        match &self {
            Subjects::Pkh(Pkh::Eip155(x)) => x.display_id(),
            Subjects::Pkh(Pkh::Solana(x)) => x.display_id(),
            Subjects::Web(Web::Ed25519(x)) => x.display_id(),
        }
    }

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SubjectError> {
        match &self {
            Subjects::Pkh(Pkh::Eip155(x)) => x.valid_signature(statement, signature).await,
            Subjects::Pkh(Pkh::Solana(x)) => x.valid_signature(statement, signature).await,
            Subjects::Web(Web::Ed25519(x)) => x.valid_signature(statement, signature).await,
        }
    }
}

// NOTE: This being here allows all internationalization to occur in the Statements -> Subects,
// dependency rather than spread between Statement and Subject.
impl Subjects {
    pub fn statement_title(&self) -> Result<String, SubjectError> {
        match &self {
            Subjects::Pkh(Pkh::Eip155(_)) => Ok("Ethereum Address".to_string()),
            Subjects::Pkh(Pkh::Solana(_)) => Ok("Solana Address".to_string()),
            Subjects::Web(Web::Ed25519(_)) => Ok("Ed25519 Web Key".to_string()),
        }
    }
}
