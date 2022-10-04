use crate::cosmos::types::Pagination;
use crate::lcd::Lcd;
use core::str;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AbsoluteTxPosition {
    block_height: u64,
    tx_index: u64,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct WasmContractResult {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub code_id: u64,
    pub creator: String,
    pub admin: String,
    pub label: String,
    pub created: Option<AbsoluteTxPosition>,
    pub ibc_port_id: String,
    pub extension: serde_json::Value,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct StateEntry {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct WasmContractStateResponse {
    pub models: Vec<StateEntry>,
    pub pagination: Pagination,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct WasmContractResponse {
    pub address: String,
    pub contract_info: WasmContractResult,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ContractCodeHistoryEntry {
    pub operation: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub code_id: u64,
    // Updated Tx position when the operation was executed.
    pub updated: Option<AbsoluteTxPosition>,
    pub msg: serde_json::Value,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct WasmContractHistoryResponse {
    pub entries: Vec<ContractCodeHistoryEntry>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct WasmContractsByCodeResponse {
    pub contracts: Vec<String>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AccessConfig {
    permission: String,
    address: String,
    addresses: Vec<String>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CodeInfoResponse {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub code_id: u64,
    pub creator: String,
    pub data_hash: String,
    pub instantiate_permission: Option<AccessConfig>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct WasmCodeResponse {
    pub code_info: CodeInfoResponse,
    pub data: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct WasmCodesResponse {
    pub code_infos: Vec<CodeInfoResponse>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct WasmPinnedCodesResponse {
    pub code_ids: Vec<u64>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct WasmParamsResponse {
    pub code_upload_access: AccessConfig,
    pub instantiate_default_permission: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct QuerySmartDataResponse {
    pub data: serde_json::Value,
}

pub struct Wasm<'a> {
    lcd: &'a Lcd,
}

impl<'a> Wasm<'a> {
    pub fn new(lcd: &'a Lcd) -> Self {
        Wasm { lcd }
    }

    pub fn contract(&self, contract: String) -> Result<WasmContractResponse, anyhow::Error> {
        self.lcd
            .get::<WasmContractResponse>(format!("/cosmwasm/wasm/v1/contract/{}", contract))
    }

    pub fn contract_history(
        &self,
        address: String,
    ) -> Result<WasmContractHistoryResponse, anyhow::Error> {
        self.lcd.get::<WasmContractHistoryResponse>(format!(
            "/cosmwasm/wasm/v1/contract/{}/history",
            address
        ))
    }

    pub fn contracts_by_code_id(
        &self,
        code_id: u64,
    ) -> Result<WasmContractsByCodeResponse, anyhow::Error> {
        self.lcd.get::<WasmContractsByCodeResponse>(format!(
            "/cosmwasm/wasm/v1/code/{}/contracts",
            code_id
        ))
    }

    pub fn contract_state(
        &self,
        address: String,
        next_key: Option<String>,
    ) -> Result<WasmContractStateResponse, anyhow::Error> {
        let pagination = if let Some(key) = next_key {
            format!("?pagination.key={}", key)
        } else {
            "".to_string()
        };
        self.lcd.get::<WasmContractStateResponse>(format!(
            "/cosmwasm/wasm/v1/contract/{}/state{}",
            address, pagination
        ))
    }

    pub fn code(&self, code_id: u64) -> Result<WasmCodeResponse, anyhow::Error> {
        self.lcd
            .get::<WasmCodeResponse>(format!("/cosmwasm/wasm/v1/code/{}", code_id))
    }

    pub fn codes(&self) -> Result<WasmCodesResponse, anyhow::Error> {
        self.lcd
            .get::<WasmCodesResponse>("/cosmwasm/wasm/v1/code".to_string())
    }

    pub fn pinned_codes(&self) -> Result<WasmPinnedCodesResponse, anyhow::Error> {
        self.lcd
            .get::<WasmPinnedCodesResponse>("/cosmwasm/wasm/v1/codes/pinned".to_string())
    }

    pub fn query_raw<U: DeserializeOwned>(
        &self,
        address: String,
        msg: String,
    ) -> Result<U, anyhow::Error> {
        let encoded_query = base64::encode(msg);
        log::info!("query {}", encoded_query);
        let ret = self.lcd.get::<QuerySmartDataResponse>(format!(
            "/cosmwasm/wasm/v1/contract/{}/raw/{}",
            address, encoded_query
        ));

        Ok(serde_json::from_str::<U>(ret?.data.to_string().as_str())?)
    }

    pub fn query_smart<T: Serialize + Clone, U: DeserializeOwned>(
        &self,
        address: String,
        msg: T,
    ) -> Result<U, anyhow::Error> {
        let encoded_query = base64::encode(serde_json::to_string(&msg)?);
        log::info!("query {}", encoded_query);
        let ret = self.lcd.get::<QuerySmartDataResponse>(format!(
            "/cosmwasm/wasm/v1/contract/{}/smart/{}",
            address, encoded_query
        ));

        Ok(serde_json::from_str::<U>(ret?.data.to_string().as_str())?)
    }

    pub fn params(&self) -> Result<WasmParamsResponse, anyhow::Error> {
        self.lcd
            .get::<WasmParamsResponse>("/cosmwasm/wasm/v1/codes/params".to_string())
    }
}
