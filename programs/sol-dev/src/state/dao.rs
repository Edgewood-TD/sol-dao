use anchor_lang::prelude::*;
#[account]
pub struct Dao {
    pub version: u16,
    pub name: String,

    pub dao_manager: Pubkey,

    /// 1) creator from this list
    pub whitelisted_creators: Pubkey,

    /// 2) mint from this list
    pub whitelisted_mints: Pubkey,

    /// total vault count registered with this bank
    pub member_count: u32,
}
