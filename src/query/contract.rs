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
    fn get_string_or_int(key: String) -> Result<String, anyhow::Error> {
        if &key.as_str()[0..2] == "00" {
            let possible_int = vec![2usize, 4usize, 8usize, 16usize, 32usize];
            if possible_int.contains(&key.len()) {
                let num = match key.len() {
                    2usize => i128::from_str_radix(&key.as_str()[0..2], 16),
                    4usize => i128::from_str_radix(&key.as_str()[0..4], 16),
                    8usize => i128::from_str_radix(&key.as_str()[0..8], 16),
                    16usize => i128::from_str_radix(&key.as_str()[0..16], 16),
                    32usize => i128::from_str_radix(&key.as_str()[0..32], 16),
                    64usize => i128::from_str_radix(&key.as_str()[0..32], 16),
                    _ => i128::from_str_radix(key.as_str(), 16),
                };

                if let Ok(valid_number) = num {
                    return Ok(valid_number.to_string());
                }
            }
        }

        let data = hex::decode(key.as_str()).expect("Decoding failed");
        Ok(str::from_utf8(&data).expect("DECODE FAILURE").to_string())
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
