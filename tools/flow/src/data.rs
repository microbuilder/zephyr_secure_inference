//! Decode the cose examples and determine if we can encrypt/decrypt data in compliance with these
//! examples.

use serde::Deserialize;
use serde_json::Value;
use std::{collections::BTreeMap, fs::File, path::Path};

use crate::Result;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Example {
    pub title: String,
    pub description: Option<String>,
    pub fail: Option<bool>,
    pub input: Inputs,
    pub intermediates: Option<Intermediates>,
    pub output: Outputs,
}

impl Example {
    /// Get the recipient key out of the example.
    pub fn get_keys(&self) -> Vec<&BTreeMap<String, String>> {
        match &self.input.item {
            InputData::Enveloped(env) => env.recipients.iter().map(|e| &e.key).collect(),
            InputData::Sign(env) => env.signers.iter().map(|e| &e.key).collect(),
            // Note that despite being sign0, we'll put it into a vec to get
            // consistent handling.
            InputData::Sign1(env) => vec![&env.key],
        }
    }

    /// Load an example from a json file.
    pub fn from_json_file<P: AsRef<Path>>(name: P) -> Result<Example> {
        let file = File::open(name)?;
        Ok(serde_json::from_reader(file)?)
    }

    /// For debugging
    #[allow(dead_code)]
    pub fn value_from_json_file<P: AsRef<Path>>(name: P) -> Result<Value> {
        let file = File::open(name)?;
        Ok(serde_json::from_reader(file)?)
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Inputs {
    pub plaintext: String,
    detached: Option<bool>,
    #[serde(flatten)]
    item: InputData,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub enum InputData {
    #[serde(rename = "enveloped")]
    Enveloped(Enveloped),
    #[serde(rename = "sign")]
    Sign(Sign),
    // Yes, this is erroneously called sign0 in the sample data.
    #[serde(rename = "sign0")]
    Sign1(Sign1),
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Enveloped {
    alg: Option<String>,
    // headers: Headers,
    recipients: Vec<Recipient>,
    fail: Option<bool>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Sign {
    pub protected: BTreeMap<String, Value>,
    pub signers: Vec<Signer>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Sign1 {
    pub key: BTreeMap<String, String>,
    pub protected: BTreeMap<String, Value>,
    pub unprotected: BTreeMap<String, Value>,
    pub alg: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Recipient {
    alg: Option<String>,
    fail: Option<bool>,
    // Headers
    key: BTreeMap<String, String>,
    // sender_key
    // failures
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Intermediates {
    #[serde(rename = "ToMax_hex")]
    pub tomax: Option<String>,
    #[serde(rename = "CEK_hex")]
    pub cek: Option<String>,
    #[serde(rename = "AAD_hex")]
    pub aad: Option<String>,
    pub recipients: Option<Vec<IntermediateRecipient>>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct IntermediateRecipient {
    // TODO: This should be optional, figure out how to encode an option hex string.
    #[serde(rename = "Context_hex")]
    #[serde(with = "hex")]
    pub context: Vec<u8>,
    #[serde(rename = "Secret_hex")]
    pub secret: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Outputs {
    #[serde(with = "hex")]
    pub cbor: Vec<u8>,
    cbor_diag: Option<String>,
    content: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Signer {
    pub key: BTreeMap<String, String>,
    pub protected: BTreeMap<String, String>,
    pub unprotected: BTreeMap<String, String>,
}
