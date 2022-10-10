pub mod auth;
pub mod authz;
pub mod bank;
pub mod distribution;
pub mod evidence;
pub mod feegrant;
pub mod gov;
pub mod mint;
pub mod params;
pub mod types;
pub mod wasm;

use crate::cosmos::auth::Auth;
use crate::cosmos::authz::Authz;
use crate::cosmos::bank::Bank;
use crate::cosmos::distribution::Distribution;
use crate::cosmos::evidence::Evidence;
use crate::cosmos::feegrant::Feegrant;
use crate::cosmos::gov::Gov;
use crate::cosmos::mint::Mint;
use crate::cosmos::params::Params;
use crate::cosmos::wasm::Wasm;
use crate::lcd::Lcd;

pub struct Cosmos<'a> {
    pub auth: Auth<'a>,
    pub authz: Authz<'a>,
    pub bank: Bank<'a>,
    pub distribution: Distribution<'a>,
    pub evidence: Evidence<'a>,
    pub feegrant: Feegrant<'a>,
    pub gov: Gov<'a>,
    pub mint: Mint<'a>,
    pub params: Params<'a>,
    pub wasm: Wasm<'a>,
}

impl<'a> Cosmos<'a> {
    pub fn new(lcd: &'a Lcd) -> Self {
        Cosmos {
            auth: Auth::new(lcd),
            authz: Authz::new(lcd),
            bank: Bank::new(lcd),
            evidence: Evidence::new(lcd),
            feegrant: Feegrant::new(lcd),
            gov: Gov::new(lcd),
            mint: Mint::new(lcd),
            distribution: Distribution::new(lcd),
            params: Params::new(lcd),
            wasm: Wasm::new(lcd),
        }
    }
}
