use anchor_lang::prelude::*;

/**
 * @param vault_token_name: Name of vault
 * @param owner_address: address of vault owner
 * @param base_token: address of base token
 * @param token_program: address of token vault
 * @param realx: address of RealboxNFT

 * @param public_unit_price: price per vault token in public sale (in base token)
 * @param min_supply: the minimum amount of vault token sold for the crowdfunding to be success
 * @param max_supply: the maximum amount of vault token in existence
 * @param private_start_time: the start block for the private sale
 * @param public_start_time: the start block for the public sale
 * @param end_time: the end block for the crowdfunding

 * @param treasury_address: address to collect fee
 * @param treasury_fee: crowdfunding fee (100 = 1%, 500 = 5%, 5 = 0.05%)
 */
#[account]
pub struct RealboxVaultState {
    pub vault_token_name: String,
    pub owner_address: Pubkey,
    pub base_token: Pubkey,
    pub token_program: Pubkey,
    pub realx: Pubkey,
    // vault info
    pub public_unit_price: u64,
    pub min_supply: u64,
    pub max_supply: u64,
    pub private_start_time: u64,
    pub public_start_time: u64,
    pub end_time: u64,
    // treasury
    pub treasury_address: Pubkey,
    pub treasury_fee: u64,
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
