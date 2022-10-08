#[cfg(test)]
mod wasm_tests {
    use crate::cosmos::wasm::AccessConfig;
    use crate::cosmos::Cosmos;
    use crate::lcd::Lcd;
    use serde::{Deserialize, Serialize};

    #[test]
    fn contract() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let contrat = cosmos
            .wasm
            .contract("ki1mf6ptkssddfmxvhdx0ech0k03ktp6kf9yk59renau2gvht3nq2gqw2adht".to_owned())
            .unwrap();
        assert_eq!(
            contrat.address,
            "ki1mf6ptkssddfmxvhdx0ech0k03ktp6kf9yk59renau2gvht3nq2gqw2adht"
        );
        assert_eq!(contrat.contract_info.code_id, 38);
        assert_eq!(contrat.contract_info.label, "CosmonDeck");
        assert_eq!(
            contrat.contract_info.admin,
            "ki1cfy5hq7n35et7geqkc2d3xxjz6sl8dp8p5yejl"
        );
        assert_eq!(
            contrat.contract_info.creator,
            "ki1cfy5hq7n35et7geqkc2d3xxjz6sl8dp8p5yejl"
        );
        assert_eq!(contrat.contract_info.created, None);
        assert_eq!(contrat.contract_info.ibc_port_id, "");
    }

    #[test]
    fn bad_contract() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let contrat = cosmos
            .wasm
            .contract("ki1mf6ptkssddfmxvhdx0ech0k03ktp6kf9yk59".to_owned());
        assert!(contrat.is_err());

        assert!(contrat
            .err()
            .unwrap()
            .to_string()
            .starts_with("decoding bech32"));
    }

    #[test]
    fn contract_history() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let history = cosmos
            .wasm
            .contract_history(
                "ki1mf6ptkssddfmxvhdx0ech0k03ktp6kf9yk59renau2gvht3nq2gqw2adht".to_string(),
            )
            .unwrap();
        assert!(history.entries.len() >= 7);
        assert_eq!(
            history.entries[0].operation,
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_INIT"
        );
        assert_eq!(history.entries[0].code_id, 6);
        assert_eq!(history.entries[0].updated, None);
        assert_eq!(
            history.entries[1].operation,
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE"
        );
        assert_eq!(history.entries[1].code_id, 13);
        assert_eq!(history.entries[1].updated, None);
        assert_eq!(
            history.entries[2].operation,
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE"
        );
        assert_eq!(history.entries[2].code_id, 14);
        assert_eq!(history.entries[2].updated, None);
        assert_eq!(
            history.entries[3].operation,
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE"
        );
        assert_eq!(history.entries[3].code_id, 19);
        assert_eq!(history.entries[3].updated, None);
        assert_eq!(
            history.entries[4].operation,
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE"
        );
        assert_eq!(history.entries[4].code_id, 25);
        assert_eq!(history.entries[4].updated, None);
        assert_eq!(
            history.entries[5].operation,
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE"
        );
        assert_eq!(history.entries[5].code_id, 33);
        assert_eq!(history.entries[5].updated, None);
        assert_eq!(
            history.entries[6].operation,
            "CONTRACT_CODE_HISTORY_OPERATION_TYPE_MIGRATE"
        );
        assert_eq!(history.entries[6].code_id, 38);
        assert_eq!(history.entries[6].updated, None);
    }

    #[test]
    fn contract_by_code_id() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let contracts = cosmos.wasm.contracts_by_code_id(35).unwrap();
        assert!(!contracts.contracts.is_empty());
        assert_eq!(
            contracts.contracts[0],
            "ki1ghyk8wtwnhu3pjtercnxr9ks9zmqz9j7y0d6xf9kwee4ctc4qz7qy9yym2"
        );
    }

    #[test]
    fn code() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let code = cosmos.wasm.code(35).unwrap();
        assert_eq!(code.code_info.code_id, 35);
        assert_eq!(
            code.code_info.creator,
            "ki12u4jtcczpg2m3nt50muh3srte7zed77qsfyng4"
        );
        assert_eq!(
            code.code_info.data_hash,
            "A4D9CF6377F05A5BBDB7E653500CA415EFBA48EFFABFA2E8D23CCF344D932A87"
        );
        assert_eq!(
            code.code_info.instantiate_permission,
            Some(AccessConfig {
                permission: "Everybody".to_string(),
                address: "".to_string(),
                addresses: None
            })
        );
    }

    #[test]
    fn codes() {
        env_logger::init();
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let codes = cosmos.wasm.codes().unwrap();
        assert!(codes.code_infos.len() > 30);
        assert_eq!(codes.code_infos[22].code_id, 23);
        assert_eq!(
            codes.code_infos[22].data_hash,
            "4A1F81CA3313B6F0D13F80DB335FEEF4EFBE87141495DF360DBD2C71A18BC595"
        );
        assert_eq!(
            codes.code_infos[22].instantiate_permission,
            Some(AccessConfig {
                permission: "Everybody".to_string(),
                address: "".to_string(),
                addresses: None
            })
        );
        assert_eq!(
            codes.code_infos[22].creator,
            "ki12u4jtcczpg2m3nt50muh3srte7zed77qsfyng4"
        );
    }

    #[test]
    fn pinned_codes() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let codes = cosmos.wasm.pinned_codes().unwrap();
        assert!(codes.code_ids.is_empty());
    }

    #[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
    #[serde(rename_all = "snake_case")]
    pub enum QueryMsg {
        GetMaxDeckByAddress {},
    }

    #[test]
    fn query_smart() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let max_decks: u64 = cosmos
            .wasm
            .query_smart(
                "ki1mf6ptkssddfmxvhdx0ech0k03ktp6kf9yk59renau2gvht3nq2gqw2adht".to_string(),
                QueryMsg::GetMaxDeckByAddress {},
            )
            .unwrap();
        assert_eq!(max_decks, 50u64);
    }
}
