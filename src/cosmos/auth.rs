use crate::cosmos::types::Pagination;
use crate::lcd::Lcd;
use serde::Deserialize;
use serde_aux::prelude::*;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Account {
    #[serde(rename(deserialize = "@type"))]
    pub account_type: String,
    pub address: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub account_number: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub sequence: u64,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AccountResponse {
    pub account: Account,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AccountsResponse {
    pub accounts: Vec<Account>,
    pub pagination: Pagination,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Params {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub max_memo_characters: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub tx_sig_limit: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub tx_size_cost_per_byte: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub sig_verify_cost_ed25519: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub sig_verify_cost_secp256k1: u32,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ParamsResponse {
    pub params: Params,
}

pub struct Auth<'a> {
    lcd: &'a Lcd,
}

impl<'a> Auth<'a> {
    pub fn new(lcd: &'a Lcd) -> Self {
        Auth { lcd }
    }

    pub fn accounts(&self) -> Result<AccountsResponse, anyhow::Error> {
        self.lcd
            .get::<AccountsResponse>("/cosmos/auth/v1beta1/accounts".to_string())
    }

    pub fn account_by_address(&self, address: String) -> Result<AccountResponse, anyhow::Error> {
        self.lcd
            .get::<AccountResponse>(format!("/cosmos/auth/v1beta1/accounts/{}", address))
    }

    pub fn params(&self) -> Result<ParamsResponse, anyhow::Error> {
        self.lcd
            .get::<ParamsResponse>("/cosmos/auth/v1beta1/params".to_string())
    }
}
