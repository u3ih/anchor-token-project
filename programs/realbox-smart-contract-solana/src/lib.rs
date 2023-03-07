use crate::utils::*;
use anchor_lang::prelude::*;

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
        ctx: Context<AgentActionToken>,
        amount: u64,
        price: f64,
        channel: SalesChannels,
        uid: String,
    ) -> Result<()> {
        realbox_vault::agent_buy_token(ctx, amount, price, channel, uid)
    }

    pub fn finalize(ctx: Context<RealboxVaultInfo>, total_supply: u64) -> Result<()> {
        realbox_vault::finalize(ctx, total_supply)
    }

    pub fn unlock_token(ctx: Context<RealboxVaultInfo>) -> Result<()> {
        realbox_vault::unlock_token(ctx)
    }

    pub fn claim_or_refund(ctx: Context<ClaimOrRefund>) -> Result<()> {
        realbox_vault::claim_or_refund(ctx)
    }

    pub fn agent_return_token(
        ctx: Context<AgentActionToken>,
        amount: u64,
        tx_id: u16,
    ) -> Result<()> {
        realbox_vault::agent_return_token(ctx, amount, tx_id)
    }

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        creator_key: Pubkey,
        uri: String,
        title: String,
        symbol: String,
    ) -> Result<()> {
        realbox_nft::mint_nft(ctx, creator_key, uri, title, symbol)
    }
}
