#[cfg(test)]
mod distribution_tests {
    use crate::cosmos::Cosmos;
    use crate::lcd::Lcd;

    #[test]
    fn params() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let params = cosmos.distribution.params().unwrap();
        assert!(params.params.withdraw_addr_enabled);
        assert_eq!(params.params.base_proposer_reward, "0.920000000000000000");
        assert_eq!(params.params.community_tax, "0.040000000000000000");
        assert_eq!(params.params.bonus_proposer_reward, "0.040000000000000000")
    }

    #[test]
    fn community_pool() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let pool = cosmos.distribution.community_pool().unwrap();
        assert!(!pool.pool.is_empty());
        assert_eq!(pool.pool.first().unwrap().denom, "uxki");
    }

    #[test]
    fn validator_outstanding_rewards() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let outstanding_rewards = cosmos
            .distribution
            .validator_outstanding_rewards(
                "kivaloper1y2znqwwcw43zneg6zk0rvadzy6q890m4dhzpsh".to_string(),
            )
            .unwrap();
        assert!(!outstanding_rewards.rewards.rewards.is_empty());
        assert_eq!(outstanding_rewards.rewards.rewards[0].denom, "uxki");
    }
}
