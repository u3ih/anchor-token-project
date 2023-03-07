use crate::errors::ErrorCode;
use crate::schema::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
};
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
    #[account(
        init_if_needed,
        payer = payer, 
        associated_token::mint = mint, 
        associated_token::authority = payer
    )]
    pub token_account: Account<'info, TokenAccount>,
    /// CHECK: this is not dangerous besause we dont read or write from this account
    pub associated_token_program: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(vault_token_name:String)]
pub struct RealboxVaultInit<'info> {
    #[account(mut)]
    pub mint_token: Account<'info, Mint>,
    #[account(mut)]
    pub base_token: Account<'info, Mint>,
    #[account(
        init,
        payer = owner_address,
        space = 8 + 4 + vault_token_name.len() + 32 * 7 + 8 * 10 + 1 + 1 + 4 + size_of::<TxInfo>() * TxInfo::MAX_ITEMS_AMOUNT,
        seeds = [vault_token_name.as_bytes()],
        bump
    )]
    pub realbox_vault: Account<'info, RealboxVaultState>,
    /// CHECK: This is the realbox NFT address
    pub realx: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
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
 * @param salesInfo: token sales information
 * @param tokenInfo: vault token information
 * @param ownerAddress: address of vault owner
 */
pub fn deploy_vault(
    ctx: Context<RealboxVaultInit>,
    vault_token_name: String,
    treasury_address: Pubkey,
    treasury_fee: u64,
    sales_info: SalesInfo,
) -> Result<()> {
    let current_time = current_timestamp().unwrap() as u64;
    let SalesInfo {
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
    require!(treasury_fee <= 10000, ErrorCode::TreasuryFeeTooBig);

    let realbox_vault = &mut ctx.accounts.realbox_vault;

    realbox_vault.token_program = ctx.accounts.token_program.clone().key();
    realbox_vault.realx = ctx.accounts.realx.clone().key();
    realbox_vault.owner_address = ctx.accounts.owner_address.key();
    realbox_vault.mint_token = ctx.accounts.mint_token.key();
    realbox_vault.mint_base = ctx.accounts.base_token.key();

    // sale info
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
    realbox_vault.treasury_address = treasury_address;
    realbox_vault.treasury_fee = treasury_fee;

    Ok(())
}

pub fn set_treasury(ctx: Context<RealboxVaultSetTreasury>, treasury_fee: u64) -> Result<()> {
    require!(treasury_fee <= 10000, ErrorCode::TreasuryFeeTooBig);
    let realbox_vault = &mut ctx.accounts.realbox_vault;
    realbox_vault.treasury_address = ctx.accounts.treasury_address.key();
    realbox_vault.treasury_fee = treasury_fee;
    Ok(())
}
