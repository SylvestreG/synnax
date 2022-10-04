use crate::cosmos::types::DecCoin;
use crate::lcd::Lcd;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Params {
    pub community_tax: String,
    pub base_proposer_reward: String,
    pub bonus_proposer_reward: String,
    pub withdraw_addr_enabled: bool,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ParamsResponse {
    pub params: Params,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CommunityPoolResponse {
    pub pool: Vec<DecCoin>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ValidatorOutstandingRewards {
    pub rewards: Vec<DecCoin>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ValidatorOutstandingRewardsResponse {
    pub rewards: ValidatorOutstandingRewards,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ValidatorAccumulatedCommission {
    pub commission: Vec<DecCoin>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ValidatorCommissionResponse {
    pub commission: ValidatorAccumulatedCommission,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ValidatorSlashEvent {
    pub validator_period: u64,
    pub fraction: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ValidatorSlashesResponse {
    pub slashes: Vec<ValidatorSlashEvent>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DelegationRewardsResponse {
    pub rewards: Vec<DecCoin>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DelegationDelegatorReward {
    pub validator_address: String,
    pub reward: Vec<DecCoin>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DelegationTotalRewardsResponse {
    pub rewards: Vec<DelegationDelegatorReward>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DelegatorValidatorsResponse {
    pub validators: Vec<String>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct DelegatorWithdrawAddressResponse {
    pub withdraw_address: String,
}

pub struct Distribution<'a> {
    lcd: &'a Lcd,
}

impl<'a> Distribution<'a> {
    pub fn new(lcd: &'a Lcd) -> Self {
        Distribution { lcd }
    }

    pub fn params(&self) -> Result<ParamsResponse, anyhow::Error> {
        self.lcd
            .get::<ParamsResponse>("/cosmos/distribution/v1beta1/params".to_string())
    }

    pub fn validator_outstanding_rewards(
        &self,
        validator_address: String,
    ) -> Result<ValidatorOutstandingRewardsResponse, anyhow::Error> {
        self.lcd.get::<ValidatorOutstandingRewardsResponse>(format!(
            "/cosmos/distribution/v1beta1/validators/{}/outstanding_rewards",
            validator_address,
        ))
    }

    pub fn validator_commission(
        &self,
        validator_address: String,
    ) -> Result<ValidatorCommissionResponse, anyhow::Error> {
        self.lcd.get::<ValidatorCommissionResponse>(format!(
            "/cosmos/distribution/v1beta1/validators/{}/commission",
            validator_address,
        ))
    }

    pub fn validator_slashes(
        &self,
        validator_address: String,
    ) -> Result<ValidatorSlashesResponse, anyhow::Error> {
        self.lcd.get::<ValidatorSlashesResponse>(format!(
            "/cosmos/distribution/v1beta1/validators/{}/slashes",
            validator_address,
        ))
    }

    pub fn delegation_rewards(
        &self,
        delegator_address: String,
        validator_address: String,
    ) -> Result<DelegationRewardsResponse, anyhow::Error> {
        self.lcd.get::<DelegationRewardsResponse>(format!(
            "/cosmos/distribution/v1beta1/delegators/{}/rewards/{}",
            delegator_address, validator_address,
        ))
    }

    pub fn delegation_total_rewards(
        &self,
        delegator_address: String,
    ) -> Result<DelegationTotalRewardsResponse, anyhow::Error> {
        self.lcd.get::<DelegationTotalRewardsResponse>(format!(
            "/cosmos/distribution/v1beta1/delegators/{}/rewards",
            delegator_address,
        ))
    }

    pub fn delegator_validators(
        &self,
        delegator_address: String,
    ) -> Result<DelegatorValidatorsResponse, anyhow::Error> {
        self.lcd.get::<DelegatorValidatorsResponse>(format!(
            "/cosmos/distribution/v1beta1/delegators/{}/validators",
            delegator_address,
        ))
    }

    pub fn delegator_withdraw_address(
        &self,
        delegator_address: String,
    ) -> Result<DelegatorWithdrawAddressResponse, anyhow::Error> {
        self.lcd.get::<DelegatorWithdrawAddressResponse>(format!(
            "/cosmos/distribution/v1beta1/delegators/{}/withdraw_address",
            delegator_address,
        ))
    }

    pub fn community_pool(&self) -> Result<CommunityPoolResponse, anyhow::Error> {
        self.lcd
            .get::<CommunityPoolResponse>("/cosmos/distribution/v1beta1/community_pool".to_string())
    }
}
