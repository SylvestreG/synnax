use crate::lcd::Lcd;
use serde::Deserialize;
use serde_aux::prelude::*;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Params {
    pub mint_denom: String,
    pub inflation_rate_change: String,
    pub inflation_max: String,
    pub inflation_min: String,
    pub goal_bonded: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub blocks_per_year: u64,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ParamsResponse {
    pub params: Params,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct InflationResponse {
    pub inflation: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AnnualProvisionsResponse {
    pub annual_provisions: String,
}

pub struct Mint<'a> {
    lcd: &'a Lcd,
}

impl<'a> Mint<'a> {
    pub fn new(lcd: &'a Lcd) -> Self {
        Mint { lcd }
    }

    pub fn params(&self) -> Result<ParamsResponse, anyhow::Error> {
        self.lcd
            .get::<ParamsResponse>("/cosmos/mint/v1beta1/params".to_string())
    }

    pub fn inflation(&self) -> Result<InflationResponse, anyhow::Error> {
        self.lcd
            .get::<InflationResponse>("/cosmos/mint/v1beta1/inflation".to_string())
    }

    pub fn annual_provisions(&self) -> Result<AnnualProvisionsResponse, anyhow::Error> {
        self.lcd
            .get::<AnnualProvisionsResponse>("/cosmos/mint/v1beta1/annual_provisions".to_string())
    }
}
