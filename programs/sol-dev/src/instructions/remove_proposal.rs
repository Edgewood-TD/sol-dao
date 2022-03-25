use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RemoveProposal<'info> {
    #[account(mut)]
    pub dao: Box<Account<'info, Dao>>,
    #[account(mut,close=proposer,has_one = proposer)]
    pub proposal: Box<Account<'info, Proposal>>,

    #[account(mut)]
    pub proposer: Signer<'info>,
    //misc
    pub system_program: Program<'info, System>,
}
