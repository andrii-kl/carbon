use super::super::types::*;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x08")]
pub struct UseAuthorityRecord {
    pub key: Key,
    pub allowed_uses: u64,
    pub bump: u8,
}
