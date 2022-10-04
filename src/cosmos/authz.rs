use crate::cosmos::types::Pagination;
use crate::lcd::Lcd;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Authorization {
    #[serde(rename(deserialize = "@type"))]
    pub authz_type: String,
    pub msg: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Grants {
    pub granter: String,
    pub grantee: String,
    pub authorization: Authorization,
    pub expiration: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GrantResponse {
    pub grants: Vec<Grants>,
    pub pagination: Pagination,
}

pub struct Authz<'a> {
    lcd: &'a Lcd,
}

impl<'a> Authz<'a> {
    pub fn new(lcd: &'a Lcd) -> Self {
        Authz { lcd }
    }

    pub fn grants(&self) -> Result<GrantResponse, anyhow::Error> {
        self.lcd
            .get::<GrantResponse>("/cosmos/authz/v1beta1/grants".to_string())
    }

    pub fn granter_grants(&self, granter: String) -> Result<GrantResponse, anyhow::Error> {
        self.lcd
            .get::<GrantResponse>(format!("/cosmos/authz/v1beta1/grants/granter/{}", granter))
    }

    pub fn grantee_grants(&self, grantee: String) -> Result<GrantResponse, anyhow::Error> {
        self.lcd
            .get::<GrantResponse>(format!("/cosmos/authz/v1beta1/grants/grantee/{}", grantee))
    }
}
