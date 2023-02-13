use anchor_lang::prelude::*;
use crate::state::realbox_vault::*;

pub fn initialize(
    ctx: Context<RealboxVault>,
    vault_id: u64,
    realx: u64,
    base_token: u64,
    // public_unit_price: u64,
    // min_supply: u64,
    // max_supply: u64,
    // private_start_block: u64,
    // public_start_block: u64,
    // end_block: u64,
) -> Result<()> {
    msg!("init realbox vault");
    let realbox_vault = &mut ctx.accounts.realbox_vault;
    // realbox_vault.owner_address = ctx.accounts.initializer.key();
    realbox_vault.vault_id = vault_id;
    realbox_vault.realx = realx;
    realbox_vault.base_token = base_token;
    msg!("{}", realbox_vault.vault_id);
    Ok(())
}
