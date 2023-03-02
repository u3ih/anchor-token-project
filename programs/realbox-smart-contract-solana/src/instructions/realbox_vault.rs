use crate::errors::ErrorCode;
use crate::schema::*;
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::{self, Mint, Token};

#[derive(Accounts)]
pub struct AgentActionToken<'info> {
    #[account(
        mut,
        has_one = owner_address,
        has_one = mint_token,
        has_one = token_program
    )]
    pub realbox_vault: Account<'info, RealboxVaultState>,
    #[account(mut)]
    pub mint_token: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    /// CHECK: This is the account receive token
    #[account(mut)]
    pub token_account: AccountInfo<'info>,
    #[account(mut)]
    pub owner_address: Signer<'info>,
}

#[derive(Accounts)]
pub struct RealboxVaultInfo<'info> {
    #[account(
        mut,
        has_one = owner_address
    )]
    pub realbox_vault: Account<'info, RealboxVaultState>,
    #[account(mut)]
    pub owner_address: Signer<'info>,
}

/**
 * @notice claim or refund
 * @param mint: address of token mint
 * @param base_token: base token
 * @param base_token_account: ATA base token
 * @param token_account: ATA token mint
 */
#[derive(Accounts)]
pub struct ClaimOrRefund<'info> {
    #[account(mut)]
    pub mint_token: Account<'info, Mint>,
    #[account(mut)]
    pub mint_base: Account<'info, Mint>,
    #[account(
        mut,
        has_one = owner_address,
        has_one = mint_token,
        has_one = mint_base
    )]
    pub realbox_vault: Box<Account<'info, RealboxVaultState>>,
    pub token_program: Program<'info, Token>,
    /// CHECK: This is the account base token
    #[account(
        init_if_needed,
        payer = owner_address,
        associated_token::mint = mint_base,
        associated_token::authority = owner_address
    )]
    pub base_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: This is the account receive token
    #[account(
        init_if_needed,
        payer = owner_address,
        associated_token::mint = mint_token,
        associated_token::authority = owner_address
    )]
    pub token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: This is the account receive fee
    #[account(
        init_if_needed,
        payer = owner_address,
        associated_token::mint = mint_base,
        associated_token::authority = treasury_address
    )]
    pub treasury_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: this is not dangerous besause we dont read or write from this account
    #[account(mut)]
    pub treasury_address: UncheckedAccount<'info>,
    /// CHECK: this is not dangerous besause we dont read or write from this account
    pub associated_token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub owner_address: Signer<'info>,
}

pub fn mint_token_to<'info>(
    mint: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let cpi_accounts = token::MintTo {
        mint,
        to,
        authority,
    };
    let cpi_program = token_program;
    // Create the CpiContent we need for the request
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::mint_to(cpi_ctx, amount)?;
    Ok(())
}

/**
 * @notice Buy vault token through agents
 * @param _amount: amount of vault token to buy
 * @param _price: price of vault token
 * @param _channel: sales channel, must be Indirect or DirectOffchain
 * @param _uid: user identity
 * @dev Caller must have trusted agent role.
 */
pub fn agent_buy_token(
    ctx: Context<AgentActionToken>,
    amount: u64,
    price: f64,
    channel: SalesChannels,
    uid: String,
) -> Result<()> {
    require!(uid.len() > 0, ErrorCode::UidMustNotEmpty);
    let realbox_vault = &mut ctx.accounts.realbox_vault;
    let state = realbox_vault.current_state();
    require!(
        state == CrowdFundingState::PrivateStarted || state == CrowdFundingState::PublicStarted,
        ErrorCode::InvalidState
    );
    require!(
        channel == SalesChannels::Indirect || channel == SalesChannels::DirectOffchain,
        ErrorCode::InvalidSalesChannel,
    );

    realbox_vault.buy_token(amount, price, channel, uid)?;

    if channel == SalesChannels::Indirect {
        realbox_vault.processed_token += amount;
        mint_token_to(
            ctx.accounts.mint_token.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.owner_address.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            amount,
        )?
    };
    Ok(())
}

/**
 * @notice Finalized the crowdfunding, cancel if not raise enough minSupply.
 * @param _total_supply: amount of vault token success raised, the remain will be refunded
 */
pub fn finalize(ctx: Context<RealboxVaultInfo>, _total_supply: u64) -> Result<()> {
    let realbox_vault = &mut ctx.accounts.realbox_vault;
    require!(
        realbox_vault.only_state(CrowdFundingState::Ended),
        ErrorCode::InvalidState
    );
    let current_supply = realbox_vault.current_supply;
    let min_supply = realbox_vault.sales_info.min_supply;
    if current_supply < min_supply {
        realbox_vault.state = CrowdFundingState::Canceled;
        realbox_vault.total_supply = 0;
    } else {
        require!(min_supply <= _total_supply, ErrorCode::InvalidSupply);
        require!(_total_supply <= current_supply, ErrorCode::InvalidSupply);
        realbox_vault.state = CrowdFundingState::Finalized;
        realbox_vault.total_supply = _total_supply;
        realbox_vault.token_state = TokenState::Locked; // lock token
    }
    Ok(())
}

pub fn claim_or_refund(ctx: Context<ClaimOrRefund>) -> Result<()> {
    let realbox_vault = &mut ctx.accounts.realbox_vault;
    let mint_token = ctx.accounts.mint_token.clone();
    require!(
        realbox_vault.only_state(CrowdFundingState::Finalized),
        ErrorCode::InvalidState
    );

    let mut collect_amount = 0;
    let total_supply = realbox_vault.total_supply;
    let mut processed_token = realbox_vault.processed_token.clone();
    let treasury_fee = realbox_vault.treasury_fee;
    let mut total_refund_token = 0;
    let mut total_claim_token = 0;

    for tx in realbox_vault.tx_infos.iter() {
        let mut claim_token = 0;
        let mut refund_token = 0;
        if processed_token + tx.amount <= total_supply {
            claim_token = tx.amount;
        } else if processed_token < total_supply {
            claim_token = total_supply - processed_token;
            refund_token = tx.amount - claim_token;
        } else {
            refund_token = tx.amount;
        }
        if refund_token > 0 && tx.channel == SalesChannels::DirectOnchain {
            total_refund_token += (refund_token as f64 * tx.unit_price) as u64;
        }
        if claim_token > 0 {
            if tx.channel == SalesChannels::DirectOnchain {
                collect_amount += (claim_token as f64 * tx.unit_price) as u64;
            }
            if tx.channel != SalesChannels::Indirect {
                total_claim_token += claim_token;
                processed_token += tx.amount;
            }
        }
    }

    if collect_amount > 0 && treasury_fee > 0 {
        let fee_amount: u64 = (collect_amount * treasury_fee) / 10000;
        let transfer_instruction = token::Transfer {
            from: ctx.accounts.base_token_account.to_account_info(),
            to: ctx.accounts.treasury_account.to_account_info(),
            authority: ctx.accounts.owner_address.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        // Create the CpiContent we need for the request
        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);
        token::transfer(cpi_ctx, fee_amount)?;
    }

    mint_token_to(
        mint_token.to_account_info(),
        ctx.accounts.token_account.to_account_info(),
        ctx.accounts.owner_address.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        total_claim_token,
    )?;

    if total_refund_token > 0 {
        let transfer_instruction = token::Transfer {
            from: ctx.accounts.base_token_account.to_account_info(),
            to: ctx.accounts.treasury_account.to_account_info(),
            authority: ctx.accounts.owner_address.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        // Create the CpiContent we need for the request
        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);
        token::transfer(cpi_ctx, total_refund_token)?;
    }

    realbox_vault.processed_token = processed_token;

    Ok(())
}

pub fn agent_return_token(ctx: Context<AgentActionToken>, amount: u64, tx_id: u16) -> Result<()> {
    let realbox_vault = &mut ctx.accounts.realbox_vault;
    let idx = tx_id as usize;
    let state = realbox_vault.current_state();
    let current_supply = &mut realbox_vault.current_supply.clone();

    let tx_infos = &mut realbox_vault.tx_infos;

    require!(idx < tx_infos.len(), ErrorCode::InvalidTransactionId);

    require!(
        state == CrowdFundingState::PrivateStarted
            || state == CrowdFundingState::PublicStarted
            || state == CrowdFundingState::Ended,
        ErrorCode::InvalidState
    );
    let tx_info = &mut tx_infos[idx];
    require!(
        amount > 0 && amount <= tx_info.amount,
        ErrorCode::InvalidAmount
    );
    require!(
        tx_info.channel == SalesChannels::Indirect
            || tx_info.channel == SalesChannels::DirectOffchain,
        ErrorCode::InvalidSalesChannel
    );
    tx_info.amount -= amount;
    *current_supply -= amount;
    if tx_info.channel == SalesChannels::Indirect {
        let burn_instruction = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                from: ctx.accounts.token_account.to_account_info(),
                mint: ctx.accounts.mint_token.to_account_info(),
                authority: ctx.accounts.owner_address.to_account_info(),
            },
        );
        token::burn(burn_instruction, amount)?;
        realbox_vault.processed_token -= amount;
    }
    Ok(())
}
