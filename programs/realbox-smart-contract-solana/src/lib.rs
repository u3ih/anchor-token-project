// use anchor_lang::error::ErrorCode;
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_spl::token;

pub mod schema;
pub use schema::*;
pub mod errors;
pub mod instructions;
pub use instructions::*;

pub mod utils;

declare_id!("DxBDQNyfuZT7ueaZRQmnR3MQBTj9oevdut6qeURxyrN");

#[program]
pub mod realbox_smart_contract_solana {

    use super::*;
    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        // Create the MintTo struct for our context
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        // Create the CpiContent we need for the request
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Execute anchor's helper function to mint tokens
        token::mint_to(cpi_ctx, amount)?;
        // instructions::realbox_vault::initialize(ctx, 1, 2, 3)?;
        Ok(())
    }

    pub fn transfer_token(ctx: Context<TransferToken>) -> Result<()> {
        require!(ctx.accounts.from.is_signer, ErrorCode::AccountNotSigner);
        // Create the Transfer struct for our context
        let transfer_instruction = token::Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        msg!(
            "authority account: {:?}",
            ctx.accounts.signer.to_account_info()
        );
        let cpi_program = ctx.accounts.token_program.to_account_info();
        // Create the CpiContent we need for the request
        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);

        // Execute anchor's helper function to transfer tokens
        token::transfer(cpi_ctx, 5)?;
        Ok(())
    }

    pub fn initialize_mint_realbox_vault(ctx: Context<InitializeMintRealBoxVault>) -> Result<()> {
        realbox_vault_factory::initialize_mint_realbox_vault(ctx)
    }

    pub fn deploy_vault(
        ctx: Context<RealboxVaultInit>,
        vault_token_name: String,
        treasury_address: Pubkey,
        treasury_fee: u64,
        sales_info: SalesInfo,
    ) -> Result<()> {
        instructions::realbox_vault_factory::deploy_vault(
            ctx,
            vault_token_name,
            treasury_address,
            treasury_fee,
            sales_info,
        )
    }

    pub fn set_treasury(ctx: Context<RealboxVaultSetTreasury>, treasury_fee: u64) -> Result<()> {
        realbox_vault_factory::set_treasury(ctx, treasury_fee)
    }

    pub fn agent_buy_token(
        ctx: Context<AgentByToken>,
        amount: u64,
        price: u64,
        channel: SalesChannels,
        uid: String,
    ) -> Result<()> {
        realbox_vault::agent_buy_token(ctx, amount, price, channel, uid)
    }

    pub fn finalize(ctx: Context<RealboxVaultInfo>, total_supply: u64) -> Result<()> {
        realbox_vault::finalize(ctx, total_supply)
    }

    pub fn claim_or_refund(ctx: Context<ClaimOrRefund>) -> Result<()> {
        realbox_vault::claim_or_refund(ctx)
    }
}
