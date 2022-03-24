use crate::custom_error::error_code::ErrorCode::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use metaplex_token_metadata::state::Metadata;
pub fn verify_holder_amount(
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
