use crate::cosmos::types::Pagination;
use crate::lcd::Lcd;
use serde::Deserialize;
use serde_aux::prelude::*;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EvidenceData {
    #[serde(rename(deserialize = "@type"))]
    pub evidemce_type: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub height: u128,
    pub time: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub power: u128,
    pub consensus_address: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct EvidenceResponse {
    pub evidence: Vec<EvidenceData>,
    pub pagination: Pagination,
}

pub struct Evidence<'a> {
    lcd: &'a Lcd,
}

impl<'a> Evidence<'a> {
    pub fn new(lcd: &'a Lcd) -> Self {
        Evidence { lcd }
    }

    pub fn evidence(&self) -> Result<EvidenceResponse, anyhow::Error> {
        self.lcd
            .get::<EvidenceResponse>("/cosmos/evidence/v1beta1/evidence".to_string())
    }
}
