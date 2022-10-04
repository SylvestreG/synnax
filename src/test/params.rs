#[cfg(test)]
mod params_tests {
    use crate::cosmos::Cosmos;
    use crate::lcd::Lcd;

    #[test]
    fn evidence() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let params = cosmos
            .params
            .params("distribution".to_string(), "0".to_string())
            .unwrap();

        assert_eq!(params.param.subspace, "distribution");
        assert_eq!(params.param.key, "0");
        assert_eq!(params.param.value, "");
    }
}
