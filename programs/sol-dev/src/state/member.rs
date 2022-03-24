use anchor_lang::prelude::*;
#[account]
pub struct Member {
    pub dao: Pubkey,

    pub data: String,

    /// reserved for future updates, has to be /8
    _reserved: [u8; 64],
}
