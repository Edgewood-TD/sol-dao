use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub dao: Box<Account<'info, Dao>>,
    #[account(init_if_needed,
        payer = proposer,
        space = 8 + std::mem::size_of::<Proposal>())]
    pub proposal: Box<Account<'info, Proposal>>,
    pub nft_account: Account<'info, TokenAccount>,
    pub nft_mint: Account<'info, Mint>,
    /// CHECK:
    pub nft_metadata_account: AccountInfo<'info>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    //misc
    pub system_program: Program<'info, System>,
}
