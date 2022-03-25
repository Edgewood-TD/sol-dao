use crate::state::*;
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct ConfigDao<'info> {
    #[account(mut,has_one=dao_manager)]
    pub dao: Box<Account<'info, Dao>>,
    #[account(mut)]
    pub dao_manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}
