use crate::lcd::Lcd;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Param {
    pub subspace: String,
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ParamsResponse {
    pub param: Param,
}

pub struct Params<'a> {
    lcd: &'a Lcd,
}

impl<'a> Params<'a> {
    pub fn new(lcd: &'a Lcd) -> Self {
        Params { lcd }
    }

    pub fn params(&self, subspace: String, key: String) -> Result<ParamsResponse, anyhow::Error> {
        self.lcd.get::<ParamsResponse>(format!(
            "/cosmos/params/v1beta1/params?subspace={}&key={}",
            subspace, key
        ))
    }
}
