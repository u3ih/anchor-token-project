// use anchor_lang::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token;
use state::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("8dToPCpRXSbf36UkQknZMaMGx6iihVkSGJoK87mDweqf");

#[program]
pub mod realbox_smart_contract_solana {

    use super::*;
    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        msg!("authority: {:?}", ctx.accounts.authority.to_account_info());
        msg!("mint: {:?}", ctx.accounts.mint.to_account_info());
        msg!(
            "token_account: {:?}",
            ctx.accounts.token_account.to_account_info()
        );
        // Create the MintTo struct for our context
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        msg!("cpi_program: {:?}", cpi_program);
        // Create the CpiContent we need for the request
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Execute anchor's helper function to mint tokens
        token::mint_to(cpi_ctx, amount)?;
        // instructions::realbox_vault::initialize(ctx, 1, 2, 3)?;
        Ok(())
    }

    pub fn transfer_token(ctx: Context<TransferToken>) -> Result<()> {
        // require!(ctx.accounts.from.is_signer, ErrorCode::AccountNotSigner);
        // Create the Transfer struct for our context
        let transfer_instruction = token::Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        // Create the CpiContent we need for the request
        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);

        // Execute anchor's helper function to transfer tokens
        token::transfer(cpi_ctx, 5)?;
        Ok(())
    }
}
