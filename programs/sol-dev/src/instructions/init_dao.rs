use crate::state::*;
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct InitDao<'info> {
    #[account(init, payer = payer, space = 8 + std::mem::size_of::<Dao>())]
    pub dao: Box<Account<'info, Dao>>,
    pub dao_manager: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
