use anchor_lang::prelude::*;
#[account]
pub struct Proposal {
    pub proposer: Pubkey,
    pub external_data: String,
}
