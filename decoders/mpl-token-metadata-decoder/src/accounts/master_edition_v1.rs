use super::super::types::*;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x02")]
pub struct MasterEditionV1 {
    pub key: Key,
    pub supply: u64,
    pub max_supply: Option<u64>,
    pub printing_mint: solana_pubkey::Pubkey,
    pub one_time_printing_authorization_mint: solana_pubkey::Pubkey,
}
