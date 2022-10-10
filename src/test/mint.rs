#[cfg(test)]
mod mint_tests {
    use crate::cosmos::mint::Params;
    use crate::cosmos::Cosmos;
    use crate::lcd::Lcd;

    #[test]
    fn params() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let params = cosmos.mint.params().unwrap();
        assert_eq!(
            params.params,
            Params {
                mint_denom: "uxki".to_string(),
                inflation_rate_change: "0.120000000000000000".to_string(),
                inflation_max: "0.140000000000000000".to_string(),
                inflation_min: "0.020000000000000000".to_string(),
                goal_bonded: "0.670000000000000000".to_string(),
                blocks_per_year: 6311520,
            }
        );
    }

    #[test]
    fn inflation() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let inflation = cosmos.mint.inflation().unwrap();
        assert_eq!(inflation.inflation, "0.140000000000000000");
    }

    #[test]
    fn annual_provisions() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let annual_provisions = cosmos.mint.annual_provisions().unwrap();
        let prov = annual_provisions.annual_provisions.parse::<f32>().unwrap();
        assert!(prov > 142613936805330.760000000000000000);
    }
}
