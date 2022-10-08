use serde::Deserialize;
use serde_aux::prelude::*;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Coin {
    pub denom: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub amount: u128,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DecCoin {
    pub denom: String,
    pub amount: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Pagination {
    pub next_key: Option<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total: u64,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Error {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub code: u64,
    pub message: String,
}
