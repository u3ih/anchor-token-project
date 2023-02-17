use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
pub struct MintToken<'info> {
    /// CHECK: This is the token that we want to mint
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    /// CHECK: This is the token account that we want to mint tokens to
    #[account(mut)]
    pub token_account: AccountInfo<'info>,
    /// CHECK: the authority of the mint account
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    pub token_program: Program<'info, Token>,
    /// CHECK: this is not dangerous besause we dont read or write from this account
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    /// CHECK: this is not dangerous besause we dont read or write from this account
    #[account(mut)]
    pub to: AccountInfo<'info>,
    /// CHECK: this is not dangerous besause we dont read or write from this account
    #[account(mut)]
    pub signer: Signer<'info>,
}
