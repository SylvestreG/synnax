use crate::cosmos::types::Coin;
use crate::lcd::Lcd;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SendEnabled {
    pub denom: String,
    pub enabled: bool,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AllBalanceResponse {
    pub balances: Vec<Coin>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct BalanceResponse {
    pub balance: Coin,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TotalSupplyResponse {
    pub supply: Vec<Coin>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TotalSupplyByDenomResponse {
    pub amount: Coin,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Params {
    pub send_enabled: Vec<SendEnabled>,
    pub default_send_enabled: bool,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ParamsResponse {
    pub params: Params,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DenomUnit {
    pub denom: String,
    pub exponent: u32,
    pub aliases: Vec<String>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Metadata {
    pub description: String,
    pub denom_units: Vec<DenomUnit>,
    pub base: String,
    pub display: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DenomsMetadataResponse {
    pub metadatas: Vec<Metadata>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DenomMetadataResponse {
    pub metadata: Metadata,
}

pub struct Bank<'a> {
    lcd: &'a Lcd,
}

impl<'a> Bank<'a> {
    pub fn new(lcd: &'a Lcd) -> Self {
        Bank { lcd }
    }

    pub fn balance_by_denom(
        &self,
        address: String,
        denom: String,
    ) -> Result<BalanceResponse, anyhow::Error> {
        self.lcd.get::<BalanceResponse>(format!(
            "/cosmos/bank/v1beta1/balances/{}/by_denom?denom={}",
            address, denom
        ))
    }

    pub fn balances(&self, address: String) -> Result<AllBalanceResponse, anyhow::Error> {
        self.lcd
            .get::<AllBalanceResponse>(format!("/cosmos/bank/v1beta1/balances/{}", address))
    }

    pub fn spendable_balances(&self, address: String) -> Result<AllBalanceResponse, anyhow::Error> {
        self.lcd.get::<AllBalanceResponse>(format!(
            "/cosmos/bank/v1beta1/spendable_balances/{}",
            address
        ))
    }

    pub fn supply(&self) -> Result<TotalSupplyResponse, anyhow::Error> {
        self.lcd
            .get::<TotalSupplyResponse>("/cosmos/bank/v1beta1/supply".to_string())
    }

    pub fn supply_by_denom(
        &self,
        denom: String,
    ) -> Result<TotalSupplyByDenomResponse, anyhow::Error> {
        self.lcd
            .get::<TotalSupplyByDenomResponse>(format!("/cosmos/bank/v1beta1/supply/{}", denom))
    }

    pub fn params(&self) -> Result<ParamsResponse, anyhow::Error> {
        self.lcd
            .get::<ParamsResponse>("/cosmos/bank/v1beta1/params".to_string())
    }

    pub fn denom_metadata_by_denom(
        &self,
        denom: String,
    ) -> Result<DenomMetadataResponse, anyhow::Error> {
        self.lcd
            .get::<DenomMetadataResponse>(format!("/cosmos/bank/v1beta1/denoms_metadata/{}", denom))
    }

    pub fn denom_metadata(&self) -> Result<DenomsMetadataResponse, anyhow::Error> {
        self.lcd
            .get::<DenomsMetadataResponse>("/cosmos/bank/v1beta1/denoms_metadata".to_string())
    }
}
