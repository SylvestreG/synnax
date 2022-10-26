#[cfg(test)]
mod contract_tests {
    use crate::cosmos::Cosmos;
    use crate::lcd::Lcd;
    use crate::query::contract::Contract;

    #[test]
    fn contract() {
        env_logger::init();
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);
        let contract = Contract::new(
            cosmos,
            "ki1enae36xr05z5rtfjjyw737jr5x4ej65asnw8le4wumsl45m4m0lqs6q0tx".to_string(),
        );
        assert!(contract.is_ok());
    }
}
