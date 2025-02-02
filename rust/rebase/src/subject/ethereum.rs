use crate::types::{error::*, types::Subject};
use async_trait::async_trait;
use ed25519_dalek::ed25519::signature::Signature as Ed25519Sig;
use hex::FromHex;
use k256::{
    ecdsa::{
        recoverable::{Id, Signature as EcdsaSig},
        Signature as Sig,
    },
    elliptic_curve::sec1::ToEncodedPoint,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

// This represents the usage of did:pkh:eip155
#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "eip155")]
pub struct Eip155 {
    pub address: String,
    pub chain_id: String,
}

#[async_trait(?Send)]
impl Subject for Eip155 {
    fn did(&self) -> Result<String, SubjectError> {
        Ok(format!("did:pkh:eip155:{}:{}", self.chain_id, self.address))
    }

    fn display_id(&self) -> Result<String, SubjectError> {
        Ok(self.address.clone())
    }

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SubjectError> {
        // NOTE: THIS ASSUMES EIP191 SIGNING.
        // TODO: Call this out in the type system?
        let statement: Vec<u8> = format!(
            "\x19Ethereum Signed Message:\n{}{}",
            statement.as_bytes().len(),
            statement
        )
        .into();

        let signature = <[u8; 65]>::from_hex(signature.trim_start_matches("0x")).map_err(|e| {
            SubjectError::Validation(format!(
                "could not marshal signature to hex: {}",
                e.to_string()
            ))
        })?;

        let pk = EcdsaSig::new(
            &Sig::from_bytes(&signature[..64]).map_err(|e| {
                SubjectError::Validation(format!(
                    "could not process signature to recover key: {}",
                    e.to_string()
                ))
            })?,
            Id::new(&signature[64] % 27).map_err(|e| {
                SubjectError::Validation(format!(
                    "could not process signature to recover key: {}",
                    e.to_string()
                ))
            })?,
        )
        .map_err(|e| SubjectError::Validation(format!("could not recover key: {}", e.to_string())))?
        .recover_verifying_key(&statement)
        .map_err(|e| {
            SubjectError::Validation(format!(
                "could not process statement to recover key: {}",
                e.to_string()
            ))
        })?;

        let address = <[u8; 20]>::from_hex(self.address.trim_start_matches("0x")).map_err(|e| {
            SubjectError::Validation(format!(
                "could not marshal address to hex: {}",
                e.to_string()
            ))
        })?;

        if Keccak256::default()
            .chain(&pk.to_encoded_point(false).as_bytes()[1..])
            .finalize()[12..]
            != address
        {
            Err(SubjectError::Validation("signature mismatch".to_string()))
        } else {
            Ok(())
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::util::{
        test_eth_did, test_eth_did_2, test_witness_signature, test_witness_statement, TestKey,
        TestWitness,
    };

    #[tokio::test]
    async fn test_eth() {
        let subject = &test_eth_did();
        subject
            .valid_signature(
                &test_witness_statement(TestWitness::DNS, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::DNS, TestKey::Eth).unwrap(),
            )
            .await
            .unwrap();
        subject
            .valid_signature(
                &test_witness_statement(TestWitness::GitHub, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::GitHub, TestKey::Eth).unwrap(),
            )
            .await
            .unwrap();
        subject
            .valid_signature(
                &test_witness_statement(TestWitness::Reddit, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::Reddit, TestKey::Eth).unwrap(),
            )
            .await
            .unwrap();
        subject
            .valid_signature(
                &test_witness_statement(TestWitness::SoundCloud, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::SoundCloud, TestKey::Eth).unwrap(),
            )
            .await
            .unwrap();
        subject
            .valid_signature(
                &test_witness_statement(TestWitness::Twitter, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::Twitter, TestKey::Eth).unwrap(),
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_eth_fail() {
        let subject = &test_eth_did();
        match subject
            .valid_signature(
                &test_witness_statement(TestWitness::DNS, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::GitHub, TestKey::Eth).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Said invalid signature was valid"),
            Err(_) => {}
        };
        match subject
            .valid_signature(
                &test_witness_statement(TestWitness::DNS, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::Twitter, TestKey::Eth).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Said invalid signature was valid"),
            Err(_) => {}
        };
        match subject
            .valid_signature(
                &test_witness_statement(TestWitness::GitHub, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::DNS, TestKey::Eth).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Said invalid signature was valid"),
            Err(_) => {}
        };
        match subject
            .valid_signature(
                &test_witness_statement(TestWitness::GitHub, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::Twitter, TestKey::Eth).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Said invalid signature was valid"),
            Err(_) => {}
        };
        match subject
            .valid_signature(
                &test_witness_statement(TestWitness::Twitter, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::GitHub, TestKey::Eth).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Said invalid signature was valid"),
            Err(_) => {}
        };
        match subject
            .valid_signature(
                &test_witness_statement(TestWitness::Twitter, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::DNS, TestKey::Eth).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Said invalid signature was valid"),
            Err(_) => {}
        };
    }

    #[tokio::test]
    async fn test_eth_bad_key() {
        let subject = &test_eth_did_2();
        match subject
            .valid_signature(
                &test_witness_statement(TestWitness::DNS, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::DNS, TestKey::Eth).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Invalid signature permitted"),
            Err(_) => {}
        };
        match subject
            .valid_signature(
                &test_witness_statement(TestWitness::GitHub, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::GitHub, TestKey::Eth).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Invalid signature permitted"),
            Err(_) => {}
        };
        match subject
            .valid_signature(
                &test_witness_statement(TestWitness::Reddit, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::Reddit, TestKey::Eth).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Invalid signature permitted"),
            Err(_) => {}
        };
        match subject
            .valid_signature(
                &test_witness_statement(TestWitness::SoundCloud, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::SoundCloud, TestKey::Eth).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Invalid signature permitted"),
            Err(_) => {}
        };
        match subject
            .valid_signature(
                &test_witness_statement(TestWitness::Twitter, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::Twitter, TestKey::Eth).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Invalid signature permitted"),
            Err(_) => {}
        };
    }
}
