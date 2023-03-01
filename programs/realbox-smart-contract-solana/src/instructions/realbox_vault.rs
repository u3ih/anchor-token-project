use crate::errors::ErrorCode;
use crate::schema::*;
use crate::utils::*;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::{self, Mint, Token};

#[derive(Accounts)]
pub struct AgentByToken<'info> {
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
    pub realbox_vault: Account<'info, RealboxVaultState>,
    pub token_program: Program<'info, Token>,
    /// CHECK: This is the account base token
    #[account(
        init_if_needed,
        payer = owner_address,
        associated_token::mint = mint_base,
        associated_token::authority = owner_address
    )]
    pub base_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is the account receive token
    #[account(
        init_if_needed,
        payer = owner_address,
        associated_token::mint = mint_token,
        associated_token::authority = owner_address
    )]
    pub token_account: Account<'info, TokenAccount>,
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
    ctx: Context<AgentByToken>,
    amount: u64,
    price: u64,
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

    mint_token_to(
        mint_token.to_account_info(),
        ctx.accounts.token_account.to_account_info(),
        ctx.accounts.owner_address.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        total_supply,
    )?;

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
            total_refund_token += refund_token * tx.unit_price;
        }
        if claim_token > 0 && tx.channel == SalesChannels::DirectOnchain {
            collect_amount += claim_token * tx.unit_price;
        }
        processed_token += tx.amount;
    }

    realbox_vault.processed_token = processed_token;

    if collect_amount > 0 && treasury_fee > 0 {
        let fee_amount: u64 = (collect_amount * treasury_fee) / 10000;
        total_refund_token += fee_amount;
        collect_amount -= fee_amount;
    }
    if collect_amount > 0 {
        total_refund_token += collect_amount;
    }

    // refund to admin
    mint_token_to(
        ctx.accounts.mint_base.to_account_info(),
        ctx.accounts.base_token_account.to_account_info(),
        ctx.accounts.owner_address.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        total_refund_token,
    )?;

    Ok(())
}

pub fn agent_return_token(ctx: Context<RealboxVaultInfo>, amount: u64, tx_id: usize) -> Result<()> {
    let realbox_vault = &mut ctx.accounts.realbox_vault;
    let state = realbox_vault.current_state();
    let current_supply = &mut realbox_vault.current_supply.clone();

    let tx_infos = &mut realbox_vault.tx_infos;

    require!(tx_id < tx_infos.len(), ErrorCode::InvalidTransactionId);

    require!(
        state == CrowdFundingState::PrivateStarted
            || state == CrowdFundingState::PublicStarted
            || state == CrowdFundingState::Ended,
        ErrorCode::InvalidState
    );
    let tx_info = &mut tx_infos[tx_id];
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
        // vaultToken.burnFrom(msg.sender, _amount);
    }
    Ok(())
}

// /**
//      * @notice Claim profit
//      * @param _profitId: profit id to claim
//      */
// function claimProfit(uint256 _profitId) external nonReentrant onlyState(CrowdFundingState.Unfrozen) {
//     ProfitInfo storage profitInfo = _profitInfo[_profitId];
//     require(profitInfo.amountPerUnit > 0, 'RealboxVault: Invalid profit id');
//     UserInfo storage userInfo = _userInfo[msg.sender][_profitId];
//     require(!userInfo.claimed, 'RealboxVault: Profit claimed');
//     uint256 balance = vaultToken.balanceOfAt(msg.sender, _profitId);
//     require(balance > 0, 'RealboxVault: No token at snapshot');
//     userInfo.amount = balance.mul(profitInfo.amountPerUnit);
//     userInfo.claimed = true;
//     profitInfo.token.safeTransfer(address(msg.sender), userInfo.amount);
//     emit ClaimProfit(_profitId, msg.sender, userInfo.amount);
// }

// /**
//  * @notice Share new profit
//  * @param _token: address of shared token
//  * @param _amount: amount of shared token
//  * @dev Owner must have allowance for this contract of at least `_amount`.
//  */
// function shareProfit(IERC20 _token, uint256 _amount) external onlyOwner onlyState(CrowdFundingState.Unfrozen) {
//     lastProfitId = vaultToken.snapshot();
//     _token.safeTransferFrom(address(msg.sender), address(this), _amount);
//     _profitInfo[lastProfitId] = ProfitInfo(_token, _amount.div(vaultToken.totalSupply()));
//     emit ShareProfit(lastProfitId, address(_token), _amount);
// }

// /**
//  * @notice Withdraw RealboxNFT items from vault
//  * @param _tokenId: id of token to withdraw
//  */
// function withdrawNft(uint256 _tokenId) external onlyOwner {
//     realx.safeTransferFrom(address(this), msg.sender, _tokenId);
//     emit AdminWithdrawNft(address(realx), _tokenId);
// }
