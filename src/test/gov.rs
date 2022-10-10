#[cfg(test)]
mod gov_tests {
    use crate::cosmos::gov::{
        DepositParams, ParamsResponse, TallyParams, TallyResult, VotingParams,
    };
    use crate::cosmos::types::Coin;
    use crate::cosmos::Cosmos;
    use crate::lcd::Lcd;

    #[test]
    fn proposals() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let proposals = cosmos.gov.proposals().unwrap();
        assert!(proposals.proposals.len() >= 2);
    }

    #[test]
    fn proposal() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let proposal = cosmos.gov.proposal(1).unwrap();
        assert_eq!(proposal.proposal.proposal_id, 1);
        assert_eq!(proposal.proposal.status, "PROPOSAL_STATUS_PASSED");
        assert_eq!(
            proposal.proposal.final_tally_result,
            TallyResult {
                yes: "430961562446809".to_string(),
                no: "0".to_string(),
                abstain: "0".to_string(),
                no_with_veto: "0".to_string(),
            }
        );
        assert_eq!(
            proposal.proposal.deposit_end_time,
            "2022-06-17T13:08:32.166135052Z"
        );
        assert_eq!(
            proposal.proposal.submit_time,
            "2022-06-03T13:08:32.166135052Z"
        );
        assert_eq!(
            proposal.proposal.total_deposit,
            [Coin {
                denom: "uxki".to_string(),
                amount: 1000001000,
            }]
        );
        assert_eq!(
            proposal.proposal.voting_end_time,
            "2022-06-10T13:53:39.514058422Z"
        );
        assert_eq!(
            proposal.proposal.voting_start_time,
            "2022-06-03T13:53:39.514058422Z"
        )
    }

    #[test]
    fn votes() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let votes = cosmos.gov.votes(2).unwrap();
        assert!(votes.votes.is_empty());
    }

    #[test]
    fn params() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        assert!(cosmos.gov.params("test".to_string()).is_err());
        let tallying = cosmos.gov.params("tallying".to_string()).unwrap();
        let deposit = cosmos.gov.params("deposit".to_string()).unwrap();
        let voting = cosmos.gov.params("voting".to_string()).unwrap();

        assert_eq!(
            tallying,
            ParamsResponse {
                voting_params: VotingParams {
                    voting_period: "0s".to_string(),
                },
                deposit_params: DepositParams {
                    min_deposit: vec![],
                    max_deposit_period: "0s".to_string(),
                },
                tally_params: TallyParams {
                    quorum: "0.334000000000000000".to_string(),
                    threshold: "0.500000000000000000".to_string(),
                    veto_threshold: "0.334000000000000000".to_string(),
                },
            }
        );
        assert_eq!(
            voting,
            ParamsResponse {
                voting_params: VotingParams {
                    voting_period: "259200s".to_string(),
                },
                deposit_params: DepositParams {
                    min_deposit: vec![],
                    max_deposit_period: "0s".to_string(),
                },
                tally_params: TallyParams {
                    quorum: "0.000000000000000000".to_string(),
                    threshold: "0.000000000000000000".to_string(),
                    veto_threshold: "0.000000000000000000".to_string(),
                },
            }
        );
        assert_eq!(
            deposit,
            ParamsResponse {
                voting_params: VotingParams {
                    voting_period: "0s".to_string(),
                },
                deposit_params: DepositParams {
                    min_deposit: vec![Coin {
                        denom: "uxki".to_string(),
                        amount: 500000000000,
                    },],
                    max_deposit_period: "604800s".to_string(),
                },
                tally_params: TallyParams {
                    quorum: "0.000000000000000000".to_string(),
                    threshold: "0.000000000000000000".to_string(),
                    veto_threshold: "0.000000000000000000".to_string(),
                },
            }
        );
    }

    #[test]
    fn deposit() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        assert!(cosmos
            .gov
            .deposit(1, "ki1khdhz2ek2h2g8xqggl0p6gyyhkalwu8usr5tl8".to_string())
            .is_err());
    }

    #[test]
    fn deposits() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let deposits = cosmos.gov.deposits(1).unwrap();
        assert!(deposits.deposits.is_empty());
    }

    #[test]
    fn tally() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let tally = cosmos.gov.tally_result(2).unwrap();
        assert_eq!(tally.tally.yes, "513662868572639");
        assert_eq!(tally.tally.no, "0");
        assert_eq!(tally.tally.abstain, "0");
        assert_eq!(tally.tally.no_with_veto, "0");
    }
}
