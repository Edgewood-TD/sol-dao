use crate::state::*;
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub dao: Box<Account<'info, Dao>>,
    #[account(init_if_needed, seeds = [
            b"proposal".as_ref(),
            proposer.key().as_ref(),
						dao.key().as_ref()
        ],
        bump,
        payer = proposer,
        space = 8 + std::mem::size_of::<Proposal>())]
    pub proposal: Box<Account<'info, Proposal>>,

    #[account(mut)]
    pub proposer: Signer<'info>,
    //misc
    pub system_program: Program<'info, System>,
}
