pub mod cosmos;
pub mod lcd;
pub mod query;
mod test;
/*
use crate::cosmos::Cosmos;
use crate::lcd::Lcd;
use crate::query::contract::ItemOrMap;
use query::contract::Contract;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::io;
use csv::Writer;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Deck {
    pub name: String,
    pub id: u64,
    pub nfts: Vec<String>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct CosmonStats {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum XpQuery {
    GetCosmonsStats { nfts: Vec<String> },
}

pub struct DeckExtended {
    pub(crate) name: String,
    pub(crate) id: u64,
    pub(crate) nfts: Vec<CosmonStats>,
}

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let mut wtr = Writer::from_path("cosmon_decks.csv")?;
    wtr.write_record(&["DeckID", "DeckName", "Ap", "Level", "Fp", "Xp", "Hp", "Spe", "Atq", "Luk", "Def", "Int", "Time", "Geographical", "Personality"])?;

    let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string())?;
    let cosmos = Cosmos::new(&lcd);
    let contract = Contract::new(
        cosmos,
        "ki1mf6ptkssddfmxvhdx0ech0k03ktp6kf9yk59renau2gvht3nq2gqw2adht".to_string(),
    )?;

    let cosmos = Cosmos::new(&lcd);

    let mut decks_extended: Vec<DeckExtended> = vec![];

    let entry = contract.state.get("DECKS").unwrap();

    if let ItemOrMap::Map { map } = entry {
        for (key, val) in map {
            let deck: Deck = serde_json::from_str(val)?;

            let new_deck = DeckExtended {
                name: deck.name.to_string(),
                id: deck.id,
                nfts: vec![],
            };
            let stats: Vec<Vec<CosmonStats>> = cosmos.wasm.query_smart(
                "ki1pu52z9aumq56s5gdg2ve2ahmr4fslnt27qztld83ng07s0ggvk6ste37qj".to_string(),
                XpQuery::GetCosmonsStats { nfts: deck.nfts },
            )?;

            let mut serialize_line = |id, name, nft_stats: Vec<CosmonStats>| -> Result<(), anyhow::Error> {
                wtr.serialize((id, name,
                               nft_stats.clone().into_iter().find(|x| x.key == "Ap").unwrap().value,
                               nft_stats.clone().into_iter().find(|x| x.key == "Level").unwrap().value,
                               nft_stats.clone().into_iter().find(|x| x.key == "Fp").unwrap().value,
                               nft_stats.clone().into_iter().find(|x| x.key == "Xp").unwrap().value,
                               nft_stats.clone().into_iter().find(|x| x.key == "Hp").unwrap().value,
                               nft_stats.clone().into_iter().find(|x| x.key == "Spe").unwrap().value,
                               nft_stats.clone().into_iter().find(|x| x.key == "Atq").unwrap().value,
                               nft_stats.clone().into_iter().find(|x| x.key == "Luk").unwrap().value,
                               nft_stats.clone().into_iter().find(|x| x.key == "Def").unwrap().value,
                               nft_stats.clone().into_iter().find(|x| x.key == "Int").unwrap().value,
                               nft_stats.clone().into_iter().find(|x| x.key == "Time").unwrap().value,
                               nft_stats.clone().into_iter().find(|x| x.key == "Geographical").unwrap().value,
                               nft_stats.clone().into_iter().find(|x| x.key == "Personality").unwrap().value))?;
                Ok(())
            };
            serialize_line(deck.id.clone(), deck.name.clone(), stats[0].clone())?;
            serialize_line(deck.id.clone(), deck.name.clone(), stats[1].clone())?;
            serialize_line(deck.id.clone(), deck.name.clone(), stats[2].clone())?;

            log::info!("key {} => {:#?}", key, stats);

        }
    }

    wtr.flush();
    Ok(())
}
*/
