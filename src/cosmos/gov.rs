use crate::cosmos::types::{Coin, Pagination};
use crate::lcd::Lcd;
use anyhow::bail;
use serde::Deserialize;
use serde_aux::prelude::*;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TallyResult {
    pub yes: String,
    pub no: String,
    pub abstain: String,
    pub no_with_veto: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct WeightedVoteOption {
    pub option: String,
    pub weight: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Vote {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub proposal_id: u64,
    pub voter: String,
    pub option: String,
    pub options: Vec<WeightedVoteOption>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub proposal_id: u64,
    pub content: serde_json::Value,
    pub status: String,
    pub final_tally_result: TallyResult,
    pub submit_time: String,
    pub deposit_end_time: String,
    pub total_deposit: Vec<Coin>,
    pub voting_start_time: String,
    pub voting_end_time: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ProposalResponse {
    pub proposal: Proposal,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ProposalsResponse {
    pub proposals: Vec<Proposal>,
    pub pagination: Pagination,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct VoteResponse {
    pub vote: Vote,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct VotesResponse {
    pub votes: Vec<Vote>,
    pub pagination: Pagination,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct VotingParams {
    pub voting_period: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DepositParams {
    pub min_deposit: Vec<Coin>,
    pub max_deposit_period: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TallyParams {
    pub quorum: String,
    pub threshold: String,
    pub veto_threshold: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Deposit {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub proposal_id: u64,
    pub depositor: String,
    pub amount: Vec<Coin>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ParamsResponse {
    pub voting_params: VotingParams,
    pub deposit_params: DepositParams,
    pub tally_params: TallyParams,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DepositResponse {
    pub deposit: Deposit,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DepositsResponse {
    pub deposits: Vec<Deposit>,
    pub pagination: Pagination,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct TallyResultResponse {
    pub tally: TallyResult,
}

pub struct Gov<'a> {
    lcd: &'a Lcd,
}

impl<'a> Gov<'a> {
    pub fn new(lcd: &'a Lcd) -> Self {
        Gov { lcd }
    }

    pub fn proposal(&self, proposal_id: u64) -> Result<ProposalResponse, anyhow::Error> {
        self.lcd
            .get::<ProposalResponse>(format!("/cosmos/gov/v1beta1/proposals/{}", proposal_id))
    }

    pub fn proposals(&self) -> Result<ProposalsResponse, anyhow::Error> {
        self.lcd
            .get::<ProposalsResponse>("/cosmos/gov/v1beta1/proposals".to_string())
    }

    pub fn vote(&self, proposal_id: u64, voter: String) -> Result<VoteResponse, anyhow::Error> {
        self.lcd.get::<VoteResponse>(format!(
            "/cosmos/gov/v1beta1/proposals/{}/votes/{}",
            proposal_id, voter
        ))
    }

    pub fn votes(&self, proposal_id: u64) -> Result<VotesResponse, anyhow::Error> {
        self.lcd.get::<VotesResponse>(format!(
            "/cosmos/gov/v1beta1/proposals/{}/votes",
            proposal_id
        ))
    }

    pub fn params(&self, param_type: String) -> Result<ParamsResponse, anyhow::Error> {
        if !["voting", "deposit", "tallying"].contains(&param_type.as_str()) {
            bail!("invalid params should be voting, deposit or tallying");
        }
        self.lcd
            .get::<ParamsResponse>(format!("/cosmos/gov/v1beta1/params/{}", param_type))
    }

    pub fn deposit(
        &self,
        proposal_id: u64,
        depositor: String,
    ) -> Result<DepositResponse, anyhow::Error> {
        self.lcd.get::<DepositResponse>(format!(
            "/cosmos/gov/v1beta1/proposals/{}/deposits/{}",
            proposal_id, depositor
        ))
    }

    pub fn deposits(&self, proposal_id: u64) -> Result<DepositsResponse, anyhow::Error> {
        self.lcd.get::<DepositsResponse>(format!(
            "/cosmos/gov/v1beta1/proposals/{}/deposits",
            proposal_id
        ))
    }

    pub fn tally_result(&self, proposal_id: u64) -> Result<TallyResultResponse, anyhow::Error> {
        self.lcd.get::<TallyResultResponse>(format!(
            "/cosmos/gov/v1beta1/proposals/{}/tally",
            proposal_id
        ))
    }
}
