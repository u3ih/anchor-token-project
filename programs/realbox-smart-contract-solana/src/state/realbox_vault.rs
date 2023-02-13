use anchor_lang::prelude::*;

#[account]
pub struct RealboxVaultState {
    pub owner_address: Pubkey,
    pub vault_id: u64,
    pub realx: u64,
    pub base_token: u64,
    pub public_unit_price: u64,
    pub min_supply: u64,
    pub max_supply: u64,
    pub private_start_block: u64,
    pub public_start_block: u64,
    pub end_block: u64,
}

#[derive(Accounts)]
pub struct RealboxVault<'info> {
    pub realbox_vault: Account<'info, RealboxVaultState>,
}
