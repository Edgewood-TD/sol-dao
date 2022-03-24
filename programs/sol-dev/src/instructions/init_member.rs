use crate::state::*;
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
#[derive(Accounts)]
pub struct InitMember<'info> {
    #[account(mut)]
    pub dao: Box<Account<'info, Dao>>,
    #[account(init, seeds = [
            b"member".as_ref(),
            creator.key().as_ref(),
						dao.key().as_ref()
        ],
        bump,
        payer = creator,
        space = 8 + std::mem::size_of::<Member>())]
    pub member: Box<Account<'info, Member>>,

    //This account is to check the Member holds the NFT(s)
    pub nft_account: Account<'info, TokenAccount>,
    pub nft_mint: Account<'info, Mint>,
    /// CHECK:
    pub nft_metadata_account: AccountInfo<'info>,
    #[account(mut)]
    pub creator: Signer<'info>,

    //misc
    pub system_program: Program<'info, System>,
}
