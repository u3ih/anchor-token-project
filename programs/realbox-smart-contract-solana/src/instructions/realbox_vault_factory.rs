use crate::errors::ErrorCode;
use crate::schema::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use std::mem::size_of;

use crate::utils::*;
#[derive(Accounts)]
pub struct InitializeMintRealBoxVault<'info> {
    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = payer,
        mint::freeze_authority = payer,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(vault_token_name:String)]
pub struct RealboxVaultInit<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = owner_address,
        space = 8 + 4 + vault_token_name.len() + 32 * 5 + 8 * 9 + 1 + size_of::<TxInfo>() * TxInfo::MAX_ITEMS_AMOUNT,
        seeds = [vault_token_name.as_bytes()],
        bump
    )]
    pub realbox_vault: Account<'info, RealboxVaultState>,
    /// CHECK: This is the realbox NFT address
    pub realx: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub base_token: Program<'info, Token>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub owner_address: Signer<'info>,
}

#[derive(Accounts)]
pub struct RealboxVaultSetTreasury<'info> {
    #[account(
        mut,
        has_one = owner_address
    )]
    pub realbox_vault: Account<'info, RealboxVaultState>,
    /// CHECK: This is the treasury address
    pub treasury_address: UncheckedAccount<'info>,
    #[account(mut)]
    pub owner_address: Signer<'info>,
}

pub fn initialize_mint_realbox_vault(ctx: Context<InitializeMintRealBoxVault>) -> Result<()> {
    Ok(())
}

/**
 * @notice deploy vault
 * @param realx: address of RealboxNFT
 * @param _salesInfo: token sales information
 * @param _tokenInfo: vault token information
 * @param _ownerAddress: address of vault owner
 */
pub fn deploy_vault(
    ctx: Context<RealboxVaultInit>,
    vault_token_name: String,
    sales_info: SalesInfo,
) -> Result<()> {
    msg!("init realbox vault");
    let current_time = current_timestamp().unwrap() as u64;
    let SalesInfo {
        base_token,
        public_unit_price,
        min_supply,
        max_supply,
        private_start_time,
        public_start_time,
        end_time,
    } = sales_info;
    
    require!(
        current_time < private_start_time,
        ErrorCode::PrivateTimeLowerThanCurrentTime
    );
    require!(
        private_start_time <= public_start_time,
        ErrorCode::PrivateTimeLowerThanCurrentTime
    );
    require!(
        public_start_time < end_time,
        ErrorCode::PrivateTimeLowerThanCurrentTime
    );
    require!(
        min_supply <= max_supply,
        ErrorCode::PrivateTimeLowerThanCurrentTime
    );

    let realbox_vault = &mut ctx.accounts.realbox_vault;

    realbox_vault.token_program = ctx.accounts.token_program.clone().key();
    realbox_vault.realx = ctx.accounts.realx.clone().key();
    realbox_vault.owner_address = ctx.accounts.owner_address.key();

    // sale info
    realbox_vault.sales_info.base_token = ctx.accounts.base_token.clone().key();
    realbox_vault.sales_info.public_unit_price = public_unit_price;
    realbox_vault.sales_info.min_supply = min_supply;
    realbox_vault.sales_info.max_supply = max_supply;
    realbox_vault.sales_info.private_start_time = private_start_time;
    realbox_vault.sales_info.public_start_time = public_start_time;
    realbox_vault.sales_info.end_time = end_time;

    // vault info
    realbox_vault.state = CrowdFundingState::Initialized;
    realbox_vault.current_supply = 0;
    realbox_vault.total_supply = 0;
    realbox_vault.vault_token_name = vault_token_name;

    Ok(())
}

pub fn set_treasury(ctx: Context<RealboxVaultSetTreasury>, treasury_fee: u64) -> Result<()> {
    msg!("update realbox vault treasury");
    require!(treasury_fee <= 10000, ErrorCode::TreasuryFeeTooBig);
    let realbox_vault = &mut ctx.accounts.realbox_vault;
    realbox_vault.treasury_address = ctx.accounts.treasury_address.key();
    realbox_vault.treasury_fee = treasury_fee;
    Ok(())
}

// #[derive(Accounts)]
// pub struct MintToken<'info> {
//     /// CHECK: This is the token that we want to mint
//     #[account(mut)]
//     pub mint: Account<'info, Mint>,
//     /// CHECK: This is the token account that we want to mint tokens to
//     #[account(
//         mut,
//         // constraint = realbox_vault.base_token == base_token.key()
//     )]
//     pub realbox_vault: Account<'info, RealboxVaultState>,
//     pub base_token: Program<'info, Token>,
//     /// CHECK: this is not dangerous besause we dont read or write from this account
//     #[account(mut)]
//     pub token_account: AccountInfo<'info>,
//     /// CHECK: the authority of the mint account
//     pub token_program: Program<'info, Token>,
//     #[account(mut)]
//     pub authority: Signer<'info>,
// }
