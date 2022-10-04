#[cfg(test)]
mod bank_tests {
    use crate::cosmos::bank::DenomUnit;
    use crate::cosmos::Cosmos;
    use crate::lcd::Lcd;

    #[test]
    fn balances() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let balances = cosmos
            .bank
            .balances("ki1khdhz2ek2h2g8xqggl0p6gyyhkalwu8usr5tl8".to_owned())
            .unwrap();
        assert!(!balances.balances.is_empty());
    }

    #[test]
    fn balance_by_denom() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let balance = cosmos
            .bank
            .balance_by_denom(
                "ki1khdhz2ek2h2g8xqggl0p6gyyhkalwu8usr5tl8".to_owned(),
                "uxki".to_owned(),
            )
            .unwrap();
        assert_eq!(balance.balance.denom, "uxki");
    }

    #[test]
    fn spendable_balances() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let balances = cosmos
            .bank
            .spendable_balances("ki1khdhz2ek2h2g8xqggl0p6gyyhkalwu8usr5tl8".to_owned())
            .unwrap();
        assert!(!balances.balances.is_empty());
    }

    #[test]
    fn supply() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let supply = cosmos.bank.supply().unwrap();
        assert!(!supply.supply.is_empty());
    }

    #[test]
    fn supply_by_denom() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let supply = cosmos.bank.supply_by_denom("uxki".to_string()).unwrap();

        assert_eq!(supply.amount.denom, "uxki");
        assert!(supply.amount.amount > 1_000_000_000_000_000u128);
    }

    #[test]
    fn params() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let params = cosmos.bank.params().unwrap();
        assert!(params.params.default_send_enabled);
    }

    #[test]
    fn metadata_by_denom() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let metadata = cosmos
            .bank
            .denom_metadata_by_denom("uxki".to_string())
            .unwrap();

        assert_eq!(
            metadata.metadata.description,
            "The native staking token of the KiChain."
        );
        assert_eq!(metadata.metadata.base, "uxki");
        assert_eq!(metadata.metadata.name, "");
        assert_eq!(metadata.metadata.display, "xki");
        assert_eq!(metadata.metadata.symbol, "");
        assert_eq!(metadata.metadata.denom_units.len(), 3);
        assert_eq!(
            metadata.metadata.denom_units[0],
            DenomUnit {
                denom: "uxki".to_string(),
                exponent: 0,
                aliases: vec!["microxki".to_string()]
            }
        );
        assert_eq!(
            metadata.metadata.denom_units[1],
            DenomUnit {
                denom: "mxki".to_string(),
                exponent: 3,
                aliases: vec!["millixki".to_string()]
            }
        );
        assert_eq!(
            metadata.metadata.denom_units[2],
            DenomUnit {
                denom: "xki".to_string(),
                exponent: 6,
                aliases: vec![]
            }
        );
    }

    #[test]
    fn metadata() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let metadata = cosmos.bank.denom_metadata().unwrap();

        assert_eq!(metadata.metadatas.len(), 1);
        assert_eq!(
            metadata.metadatas[0].description,
            "The native staking token of the KiChain."
        );
        assert_eq!(metadata.metadatas[0].base, "uxki");
        assert_eq!(metadata.metadatas[0].name, "");
        assert_eq!(metadata.metadatas[0].display, "xki");
        assert_eq!(metadata.metadatas[0].symbol, "");
        assert_eq!(metadata.metadatas[0].denom_units.len(), 3);
        assert_eq!(
            metadata.metadatas[0].denom_units[0],
            DenomUnit {
                denom: "uxki".to_string(),
                exponent: 0,
                aliases: vec!["microxki".to_string()]
            }
        );
        assert_eq!(
            metadata.metadatas[0].denom_units[1],
            DenomUnit {
                denom: "mxki".to_string(),
                exponent: 3,
                aliases: vec!["millixki".to_string()]
            }
        );
        assert_eq!(
            metadata.metadatas[0].denom_units[2],
            DenomUnit {
                denom: "xki".to_string(),
                exponent: 6,
                aliases: vec![]
            }
        );
    }
}
