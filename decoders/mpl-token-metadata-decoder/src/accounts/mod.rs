use solana_pubkey::{pubkey, Pubkey};
use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;
use crate::types::Key;
use super::TokenMetadataDecoder;
pub mod collection_authority_record;
pub mod edition;
pub mod edition_marker;
pub mod edition_marker_v2;
pub mod holder_delegate_record;
pub mod master_edition_v1;
pub mod master_edition_v2;
pub mod metadata;
pub mod metadata_delegate_record;
pub mod reservation_list_v1;
pub mod reservation_list_v2;
pub mod token_owned_escrow;
pub mod token_record;
pub mod use_authority_record;

pub enum TokenMetadataAccount {
    CollectionAuthorityRecord(collection_authority_record::CollectionAuthorityRecord),
    MetadataDelegateRecord(metadata_delegate_record::MetadataDelegateRecord),
    HolderDelegateRecord(holder_delegate_record::HolderDelegateRecord),
    Edition(edition::Edition),
    EditionMarker(edition_marker::EditionMarker),
    EditionMarkerV2(edition_marker_v2::EditionMarkerV2),
    TokenOwnedEscrow(token_owned_escrow::TokenOwnedEscrow),
    MasterEditionV2(master_edition_v2::MasterEditionV2),
    MasterEditionV1(master_edition_v1::MasterEditionV1),
    Metadata(metadata::Metadata),
    TokenRecord(token_record::TokenRecord),
    ReservationListV2(reservation_list_v2::ReservationListV2),
    ReservationListV1(reservation_list_v1::ReservationListV1),
    UseAuthorityRecord(use_authority_record::UseAuthorityRecord),
}

impl AccountDecoder<'_> for TokenMetadataDecoder {
    type AccountType = TokenMetadataAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        // Guard
        let mpl_token_metadata_id: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
        if account.owner != mpl_token_metadata_id {
            return None;
        }

        let tag = match account.data.first().copied() {
            Some(b) => b,
            None => return None,
        };

        let data_enum = match tag {
            x if x == Key::MetadataV1 as u8 => {
                metadata::Metadata::deserialize(&mut account.data.as_slice())
                    .map(TokenMetadataAccount::Metadata)
            }
            x if x == Key::CollectionAuthorityRecord as u8 => {
                collection_authority_record::CollectionAuthorityRecord::deserialize(&mut account.data.as_slice())
                    .map(TokenMetadataAccount::CollectionAuthorityRecord)
            }
            x if x == Key::MetadataDelegate as u8 => {
                metadata_delegate_record::MetadataDelegateRecord::deserialize(&mut account.data.as_slice())
                    .map(TokenMetadataAccount::MetadataDelegateRecord)
            }
            x if x == Key::HolderDelegate as u8 => {
                holder_delegate_record::HolderDelegateRecord::deserialize(&mut account.data.as_slice())
                    .map(TokenMetadataAccount::HolderDelegateRecord)
            }
            x if x == Key::EditionV1 as u8 => {
                edition::Edition::deserialize(&mut account.data.as_slice())
                    .map(TokenMetadataAccount::Edition)
            }
            x if x == Key::EditionMarker as u8 => {
                edition_marker::EditionMarker::deserialize(&mut account.data.as_slice())
                    .map(TokenMetadataAccount::EditionMarker)
            }
            x if x == Key::EditionMarkerV2 as u8 => {
                edition_marker_v2::EditionMarkerV2::deserialize(&mut account.data.as_slice())
                    .map(TokenMetadataAccount::EditionMarkerV2)
            }
            x if x == Key::TokenOwnedEscrow as u8 => {
                token_owned_escrow::TokenOwnedEscrow::deserialize(&mut account.data.as_slice())
                    .map(TokenMetadataAccount::TokenOwnedEscrow)
            }
            x if x == Key::MasterEditionV2 as u8 => {
                master_edition_v2::MasterEditionV2::deserialize(&mut account.data.as_slice())
                    .map(TokenMetadataAccount::MasterEditionV2)
            }
            x if x == Key::MasterEditionV1 as u8 => {
                master_edition_v1::MasterEditionV1::deserialize(&mut account.data.as_slice())
                    .map(TokenMetadataAccount::MasterEditionV1)
            }
            x if x == Key::TokenRecord as u8 => {
                token_record::TokenRecord::deserialize(&mut account.data.as_slice())
                    .map(TokenMetadataAccount::TokenRecord)
            }
            x if x == Key::ReservationListV2 as u8 => {
                reservation_list_v2::ReservationListV2::deserialize(&mut account.data.as_slice())
                    .map(TokenMetadataAccount::ReservationListV2)
            }
            x if x == Key::ReservationListV1 as u8 => {
                reservation_list_v1::ReservationListV1::deserialize(&mut account.data.as_slice())
                    .map(TokenMetadataAccount::ReservationListV1)
            }
            x if x == Key::UseAuthorityRecord as u8 => {
                use_authority_record::UseAuthorityRecord::deserialize(&mut account.data.as_slice())
                    .map(TokenMetadataAccount::UseAuthorityRecord)
            }
            _ => None,
        }?;

        Some(carbon_core::account::DecodedAccount {
            lamports: account.lamports,
            data: data_enum,
            owner: account.owner,
            executable: account.executable,
            rent_epoch: account.rent_epoch,
        })
    }

}
