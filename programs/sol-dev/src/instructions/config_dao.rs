use crate::state::*;
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct ConfigDao<'info> {
    #[account(mut)]
    pub dao: Box<Account<'info, Dao>>,
    /// CHECK: Manager is responsible for checking this address is valid
    pub nft_whitelist_creator: AccountInfo<'info>,
}
