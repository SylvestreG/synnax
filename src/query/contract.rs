use crate::cosmos::types::Coin;
use crate::cosmos::wasm::{StateEntry, WasmContractResponse};
use crate::cosmos::Cosmos;
use core::str;
use log::info;
use std::collections::BTreeMap;

#[derive(Clone)]
pub enum ItemOrMap {
    Item { value: String },
    Map { map: BTreeMap<String, String> },
}

#[derive(Clone)]
pub struct Contract {
    pub contract: WasmContractResponse,
    pub funds: Vec<Coin>,
    pub state: BTreeMap<String, ItemOrMap>,
}

impl Contract {
    fn decodei8(val: String) -> String {
        let mut val = u8::from_str_radix(val.as_str(), 16).unwrap();
        let mut val_i8: i8;

        let is_postive = val & 0x80 == 0x80;
        val &= 0x7f;
        if !is_postive {
            val = 0x80 - val;
            val_i8 = val as i8;
            val_i8 = -val_i8;
        } else {
            val_i8 = val as i8;
        }

        val_i8.to_string()
    }
    fn decodei16(val: String) -> String {
        let mut val = u16::from_str_radix(val.as_str(), 16).unwrap();
        let mut val_i16: i16;

        let is_postive = val & 0x8000 == 0x8000;
        val &= 0x7fff;
        if !is_postive {
            val = 0x8000 - val;
            val_i16 = val as i16;
            val_i16 = -val_i16;
        } else {
            val_i16 = val as i16;
        }

        val_i16.to_string()
    }
    fn decodei32(val: String) -> String {
        let mut val = u32::from_str_radix(val.as_str(), 16).unwrap();
        let mut val_i32: i32;

        let is_postive = val & 0x80000000 == 0x80000000;
        val &= 0x7fffffff;
        if !is_postive {
            val = 0x80000000 - val;
            val_i32 = val as i32;
            val_i32 = -val_i32;
        } else {
            val_i32 = val as i32;
        }

        val_i32.to_string()
    }

    fn decodei64(val: String) -> String {
        let mut val = u64::from_str_radix(val.as_str(), 16).unwrap();
        let mut val_i64: i64;

        let is_postive = val & 0x8000000000000000 == 0x8000000000000000;
        val &= 0x7fffffffffffffff;
        if !is_postive {
            val = 0x8000000000000000 - val;
            val_i64 = val as i64;
            val_i64 = -val_i64;
        } else {
            val_i64 = val as i64;
        }

        val_i64.to_string()
    }

    fn get_string_or_int(key: String) -> Result<String, anyhow::Error> {
        if &key.as_str()[0..2] == "00" {
            let res = match key.len() {
                2 => u8::from_str_radix(key.as_str(), 16).unwrap().to_string(),
                4 => u16::from_str_radix(key.as_str(), 16).unwrap().to_string(),
                8 => u32::from_str_radix(key.as_str(), 16).unwrap().to_string(),
                16 => u64::from_str_radix(key.as_str(), 16).unwrap().to_string(),
                32 => u128::from_str_radix(key.as_str(), 16).unwrap().to_string(),
                _ => "unknown".to_string(),
            };
            return Ok(res);
        }

        let data = hex::decode(key.as_str()).expect("Decoding failed");
        if let Ok(ret) = str::from_utf8(&data) {
            return Ok(ret.to_string());
        }

        let res = match key.len() {
            2 => Contract::decodei8(key),
            4 => Contract::decodei16(key),
            8 => Contract::decodei32(key),
            16 => Contract::decodei64(key),
            _ => "unknown".to_string(),
        };

        Ok(res)
    }

    fn get_map_key_index(key: String) -> Result<Option<(String, String)>, anyhow::Error> {
        if key.len() > 4 {
            let size = u16::from_str_radix(&key.as_str()[0..4], 16);

            if let Ok(len) = size {
                // to digit by character in hexa
                let hex_len = ((len + 2) * 2) as usize;

                if hex_len < key.len() {
                    let value = &key.as_str()[hex_len as usize..key.len()];
                    let key = &key.as_str()[4usize..hex_len as usize];
                    let parsed_key = str::from_utf8(&hex::decode(&key)?)
                        .expect("INVALID_VALUE")
                        .to_string();

                    return Ok(Some((
                        parsed_key,
                        Contract::get_string_or_int(value.to_string())?,
                    )));
                }
            }
        }
        Ok(None)
    }

    fn pretty_print_json(data: String) -> String {
        let json: Result<serde_json::Value, _> = serde_json::from_str(data.as_str());
        if let Ok(js) = json {
            let pretty_json = serde_json::to_string_pretty(&js);
            if let Ok(pretty) = pretty_json {
                return pretty;
            }
        }

        data
    }

    pub fn new(cosmos: Cosmos, address: String) -> Result<Self, anyhow::Error> {
        let mut all_data: Vec<StateEntry> = vec![];
        let contract = cosmos.wasm.contract(address.clone())?;
        let mut cosmos_data = cosmos.wasm.contract_state(address.to_string(), None)?;
        let funds = cosmos.bank.balances(address.to_string())?.balances;
        all_data.append(&mut cosmos_data.models);

        while let Some(key) = cosmos_data.pagination.next_key {
            info!("key");
            cosmos_data = cosmos.wasm.contract_state(address.to_string(), Some(key))?;
            all_data.append(&mut cosmos_data.models);
        }

        info!("found {} elems", all_data.len());

        let mut state = BTreeMap::new();

        for data in all_data {
            log::trace!("decoding {} => {}", data.key, data.value);
            let res = Contract::get_map_key_index(data.key.clone());

            let base64_data = base64::decode(data.value.as_str()).expect("INVALID_VALUE");
            let decoded_value = str::from_utf8(&base64_data).expect("INVALID_VALUE");
            let decoded_value = Contract::pretty_print_json(decoded_value.to_string());
            let decoded_key;

            if let Ok(Some((key, index))) = res {
                let entry = state.entry(key.clone()).or_insert(ItemOrMap::Map {
                    map: BTreeMap::new(),
                });
                match entry {
                    ItemOrMap::Map { map } => {
                        map.insert(index.clone(), decoded_value.to_string());
                    }
                    ItemOrMap::Item { value } => log::error!("invalid item for map {}", value),
                }
            } else {
                let temp = hex::decode(data.key.as_str()).expect("Decoding failed");
                decoded_key = str::from_utf8(&temp).expect("INVALID_KEY").to_string();
                if state.contains_key(&decoded_key) {
                    log::error!("error overwriting {}", decoded_key);
                }
                state.insert(
                    decoded_key.clone(),
                    ItemOrMap::Item {
                        value: decoded_value.to_string(),
                    },
                );
            }
        }

        for (key, val) in &state {
            match val {
                ItemOrMap::Item { value } => log::info!("{} => {}", key, value),
                ItemOrMap::Map { map } => log::info!("{} => {} entries", key, map.len()),
            }
        }

        Ok(Contract {
            contract,
            state,
            funds,
        })
    }
}
