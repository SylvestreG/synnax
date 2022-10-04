#[cfg(test)]
mod evidence_tests {
    use crate::cosmos::Cosmos;
    use crate::lcd::Lcd;

    #[test]
    fn evidence() {
        let lcd = Lcd::new("https://api-mainnet.blockchain.ki".to_string()).unwrap();
        let cosmos = Cosmos::new(&lcd);

        let evidence = cosmos.evidence.evidence().unwrap();

        assert_eq!(evidence.pagination.total, 1);
        assert_eq!(evidence.evidence[0].power, 200723);
        assert_eq!(evidence.evidence[0].height, 11316735);
        assert_eq!(evidence.evidence[0].time, "2022-09-01T22:04:26.317651814Z");
        assert_eq!(
            evidence.evidence[0].evidemce_type,
            "/cosmos.evidence.v1beta1.Equivocation"
        );
        assert_eq!(
            evidence.evidence[0].consensus_address,
            "kivalcons1gljaqtldcvqc2dmwu28pdt4ghed38qyz39ts6s"
        );
    }
}
