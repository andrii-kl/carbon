use super::super::types::*;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x09")]
pub struct CollectionAuthorityRecord {
    pub key: Key,
    pub bump: u8,
    pub update_authority: Option<solana_pubkey::Pubkey>,
}
