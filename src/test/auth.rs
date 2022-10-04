#[cfg(test)]
mod auth_tests {
    use crate::cosmos::Cosmos;
    use crate::lcd::Lcd;

    #[test]
    fn accounts() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let accounts = cosmos.auth.accounts().unwrap();
        assert!(accounts.pagination.total > 100);
        assert!(accounts.accounts[0].address.starts_with("ki"));
    }

    #[test]
    fn account_by_address() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let account = cosmos
            .auth
            .account_by_address("ki1khdhz2ek2h2g8xqggl0p6gyyhkalwu8usr5tl8".to_string())
            .unwrap();
        assert!(account.account.address.starts_with("ki"));
        assert_eq!(account.account.account_number, 2669);
        assert!(account.account.sequence > 100);
    }

    #[test]
    fn params() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let params = cosmos.auth.params().unwrap();
        assert_eq!(params.params.sig_verify_cost_secp256k1, 1000);
        assert_eq!(params.params.max_memo_characters, 512);
        assert_eq!(params.params.tx_sig_limit, 7);
        assert_eq!(params.params.sig_verify_cost_ed25519, 590);
        assert_eq!(params.params.tx_size_cost_per_byte, 10);
    }
}
