#[cfg(test)]
mod authz_tests {
    use crate::cosmos::Cosmos;
    use crate::lcd::Lcd;

    #[test]
    fn grantee() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let grants = cosmos
            .authz
            .grantee_grants("ki1a9j9ncrky0mt7hgdtuyzm08yvd3mdu0xrpjtf4".to_string())
            .unwrap();

        assert!(grants.pagination.total >= 2);
        assert_eq!(
            grants.grants[0].grantee,
            "ki1a9j9ncrky0mt7hgdtuyzm08yvd3mdu0xrpjtf4"
        );
        assert_eq!(grants.grants[0].expiration, "2023-06-24T14:49:02Z");
    }

    #[test]
    fn granter() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let grants = cosmos
            .authz
            .granter_grants("ki1cfy5hq7n35et7geqkc2d3xxjz6sl8dp8p5yejl".to_string())
            .unwrap();

        assert!(grants.pagination.total >= 2);
        assert_eq!(
            grants.grants[0].granter,
            "ki1cfy5hq7n35et7geqkc2d3xxjz6sl8dp8p5yejl"
        );
        assert_eq!(grants.grants[0].expiration, "2023-06-24T14:51:46Z");
    }
}
