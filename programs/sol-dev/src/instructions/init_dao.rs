use crate::state::*;
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct InitDao<'info> {
    #[account(init_if_needed,seeds = [
            b"dao".as_ref(),
            dao_manager.key().as_ref(),
        ],
        bump, payer = dao_manager, space = 8 +10232)]
    pub dao: Box<Account<'info, Dao>>,

    #[account(mut)]
    pub dao_manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}
