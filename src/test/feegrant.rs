#[cfg(test)]
mod feegrant_tests {
    use crate::cosmos::types::Coin;
    use crate::cosmos::Cosmos;
    use crate::lcd::Lcd;

    #[test]
    fn allowance() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let allowance = cosmos
            .feegrant
            .allowance(
                "ki1f2q7swkt8lexl2trkl88g5kc5lhmxf0tqvlrm8".to_string(),
                "ki10ztpacs9u2retxkr2e2z5gsdrhw86c0gl26tsd".to_string(),
            )
            .unwrap();
        assert_eq!(
            allowance.allowance.granter,
            "ki1f2q7swkt8lexl2trkl88g5kc5lhmxf0tqvlrm8"
        );
        assert_eq!(
            allowance.allowance.grantee,
            "ki10ztpacs9u2retxkr2e2z5gsdrhw86c0gl26tsd"
        );
        assert_eq!(
            allowance.allowance.allowance.allowance_type,
            "/cosmos.feegrant.v1beta1.BasicAllowance"
        );
        assert_eq!(
            allowance.allowance.allowance.expiration,
            "2022-10-30T15:04:05Z"
        );
        assert_eq!(
            allowance.allowance.allowance.spend_limit,
            vec![Coin {
                denom: "utki".to_string(),
                amount: 1000
            }]
        );
    }
}
