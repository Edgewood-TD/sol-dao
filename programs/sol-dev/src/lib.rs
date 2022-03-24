use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use metaplex_token_metadata::state::Metadata;
use std::str::FromStr;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod sol_dev {
    use super::*;
    pub fn init_member(ctx: Context<InitMember>) -> Result<()> {
        let dao = &mut ctx.accounts.dao;

        assert_eq!(
            dao.dao_manager,
            ctx.accounts.creator.key(),
            "Not DAO Manager"
        );
        dao.member_count += 1;
        Ok(())
    }
    pub fn init_dao(ctx: Context<InitDao>, dao_name: String) -> Result<()> {
        let dao = &mut ctx.accounts.dao;
        dao.name = dao_name;
        dao.dao_manager = ctx.accounts.dao_manager.key();
        Ok(())
    }
}
#[event]
pub struct MyEvent {
    pub data: u64,
    #[index]
    pub label: String,
}
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
    /*
        //This account is to check the Member holds the NFT(s)
    pub nft_account: Box<Account<'info, TokenAccount>>,
        */
    #[account(mut)]
    pub creator: Signer<'info>,

    //misc
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitDao<'info> {
    #[account(init, payer = payer, space = 8 + std::mem::size_of::<Dao>())]
    pub dao: Box<Account<'info, Dao>>,
    pub dao_manager: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfigDao<'info> {
    #[account(mut)]
    pub dao: Box<Account<'info, Dao>>,
    /// CHECK: Manager is responsible for checking this address is valid
    pub nft_whitelist_creator: AccountInfo<'info>,
}

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

#[account]
pub struct Member {
    /// sole control over gem whitelist, un/locking the vaults, and bank flags
    /// can update itself to another Pubkey
    pub dao: Pubkey,

    pub data: String,

    /// reserved for future updates, has to be /8
    _reserved: [u8; 64],
}
#[account]
pub struct Proposal {
    pub proposer: Pubkey,
    pub external_data: String,
}

fn assert_valid_metadata(
    gem_metadata: &AccountInfo,
    gem_mint: &Pubkey,
) -> core::result::Result<Metadata, ProgramError> {
    let metadata_program = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap();
    let seed = &[
        b"metadata".as_ref(),
        metadata_program.as_ref(),
        gem_mint.as_ref(),
    ];
    let (metadata_addr, _bump) = Pubkey::find_program_address(seed, &metadata_program);
    Metadata::from_account_info(gem_metadata)
}

fn verify_holder_amount(
    nft_token_account: TokenAccount,
    nft_mint: Pubkey,
    nft_holder: Pubkey,
) -> Result<()> {
    if nft_token_account.amount == 1
        && nft_token_account.mint == nft_mint
        && nft_holder == nft_token_account.owner
    {
        return Ok(());
    } else {
        msg!("invalid Hold Amount!");
        return Err(error!(ErrorCode::InvalidHolder));
    }
}

fn verify_creator(
    nft_mint: Pubkey,
    metadata_account: &AccountInfo,
    valid_creator: Pubkey,
) -> Result<()> {
    if &Pubkey::find_program_address(
        &[
            "metadata".as_bytes(),
            &metaplex_token_metadata::ID.to_bytes(),
            &nft_mint.to_bytes(),
        ],
        &metaplex_token_metadata::ID,
    )
    .0 != &metadata_account.key()
    {
        msg!("invalid metadata account!");
        return Err(error!(ErrorCode::InvalidMetadata));
    }

    let metadata_account = Metadata::from_account_info(&metadata_account)?;
    let creators = metadata_account.data.creators.unwrap();

    let cndy = creators.first().unwrap();
    let candy_machine = cndy.address;

    if valid_creator != candy_machine {
        msg!(format!(
            "Creator in vault: {}, creator in metadata: {}",
            valid_creator, candy_machine
        )
        .as_str());
        msg!("invalid creator!");
        return Err(error!(ErrorCode::InvalidCreator));
    }

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Metadata")]
    InvalidMetadata,
    #[msg("NFT is not whitelisted")]
    InvalidCreator,
    #[msg("Account not holding provided NFT")]
    InvalidHolder,
}
