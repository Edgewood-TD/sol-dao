use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use metaplex_token_metadata::state::Metadata;

pub mod custom_error;
pub mod instructions;
pub mod state;
pub mod utils;
pub use custom_error::error_code::ErrorCode::*;
use instructions::*;
declare_id!("Daoa93bPj45FmxkeeKA7cginUou4MKSdYha8ApHehx4j");

#[program]
pub mod sol_dev {
    use super::*;
    pub fn init_member(ctx: Context<InitMember>) -> Result<()> {
        let dao = &mut ctx.accounts.dao;

        verify_creator(
            ctx.accounts.nft_mint.key(),
            &ctx.accounts.nft_metadata_account,
            dao.whitelisted_creators,
        )?;

        verify_holder_amount(
            ctx.accounts.nft_account.clone(),
            ctx.accounts.nft_mint.key(),
            ctx.accounts.creator.key(),
        )?;
        dao.member_count += 1;
        Ok(())
    }
    pub fn init_dao(ctx: Context<InitDao>, dao_name: String) -> Result<()> {
        let dao = &mut ctx.accounts.dao;
        dao.name = dao_name;
        dao.dao_manager = ctx.accounts.dao_manager.key();
        Ok(())
    }
    pub fn config_dao(ctx: Context<ConfigDao>, whitelist_creator: Pubkey) -> Result<()> {
        let dao = &mut ctx.accounts.dao;
        dao.whitelisted_creators = whitelist_creator;
        Ok(())
    }
    pub fn create_proposal(ctx: Context<CreateProposal>, proposal_uri: String) -> Result<()> {
        let dao = &mut ctx.accounts.dao;
        verify_creator(
            ctx.accounts.nft_mint.key(),
            &ctx.accounts.nft_metadata_account,
            dao.whitelisted_creators,
        )?;

        verify_holder_amount(
            ctx.accounts.nft_account.clone(),
            ctx.accounts.nft_mint.key(),
            ctx.accounts.proposer.key(),
        )?;
        let proposal = &mut ctx.accounts.proposal;
        dao.proposals.push(proposal.key());
        proposal.proposer = ctx.accounts.proposer.key();
        proposal.external_data = proposal_uri;
        Ok(())
    }
    pub fn remove_proposal(ctx: Context<RemoveProposal>) -> Result<()> {
        let dao = &mut ctx.accounts.dao;
        let proposal = &mut ctx.accounts.proposal;
        let remove_index = dao.proposals.iter().position(|&x| x == proposal.key());
        if let Some(key) = remove_index {
            dao.proposals.remove(key);
        } else {
            return Err(error!(ProposalNotFound));
        }
        Ok(())
    }
}
#[event]
pub struct MyEvent {
    pub data: u64,
    #[index]
    pub label: String,
}

/* fn assert_valid_metadata(
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
} */

fn verify_holder_amount(
    nft_token_account: Account<'_, TokenAccount>,
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
        return Err(error!(InvalidHolder));
    }
}

pub fn verify_creator(
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
        return Err(error!(InvalidMetadata));
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
        return Err(error!(InvalidCreator));
    }

    Ok(())
}
