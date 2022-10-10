use crate::cosmos::types::{Coin, Pagination};
use crate::lcd::Lcd;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Allowance {
    #[serde(rename(deserialize = "@type"))]
    pub allowance_type: String,
    pub spend_limit: Vec<Coin>,
    pub expiration: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Grant {
    pub granter: String,
    pub grantee: String,
    pub allowance: Allowance,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AllowancesByGranterResponse {
    pub allowances: Vec<Grant>,
    pub pagination: Pagination,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AllowancesResponse {
    pub allowances: Vec<Grant>,
    pub pagination: Pagination,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AllowanceResponse {
    pub allowance: Grant,
}

pub struct Feegrant<'a> {
    lcd: &'a Lcd,
}

impl<'a> Feegrant<'a> {
    pub fn new(lcd: &'a Lcd) -> Self {
        Feegrant { lcd }
    }

    pub fn allowance(
        &self,
        granter: String,
        grantee: String,
    ) -> Result<AllowanceResponse, anyhow::Error> {
        self.lcd.get::<AllowanceResponse>(format!(
            "/cosmos/feegrant/v1beta1/allowance/{}/{}",
            granter, grantee
        ))
    }

    pub fn allowances(&self, grantee: String) -> Result<AllowancesResponse, anyhow::Error> {
        self.lcd
            .get::<AllowancesResponse>(format!("/cosmos/feegrant/v1beta1/allowances/{}", grantee))
    }

    pub fn allowances_by_granter(
        &self,
        granter: String,
    ) -> Result<AllowancesByGranterResponse, anyhow::Error> {
        self.lcd.get::<AllowancesByGranterResponse>(format!(
            "/cosmos/feegrant/v1beta1/issued/{}",
            granter
        ))
    }
}
