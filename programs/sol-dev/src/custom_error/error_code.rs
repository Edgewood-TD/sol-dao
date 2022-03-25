use anchor_lang::prelude::*;
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Metadata")]
    InvalidMetadata,
    #[msg("NFT is not whitelisted")]
    InvalidCreator,
    #[msg("Account not holding provided NFT")]
    InvalidHolder,
    #[msg("Proposal Not Found")]
    ProposalNotFound,
}
