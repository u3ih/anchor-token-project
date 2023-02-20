use crate::utils::*;
use anchor_lang::prelude::*;

/**
 * @param sender: pubkey of user
 * @param amount: amount of vault token
 * @param unit_price: price of vault token
 * @param channel: sales channel, must be Indirect or DirectOffchain
 * @param uid: user identity
 */
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TxInfo {
    sender: Pubkey,
    amount: u64,
    unit_price: u64,
    channel: SalesChannels,
    uid: String,
}

impl TxInfo {
    pub const MAX_ITEMS_AMOUNT: usize = 100;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SalesInfo {
    pub base_token: Pubkey,
    pub public_unit_price: u64,
    pub min_supply: u64,
    pub max_supply: u64,
    pub private_start_time: u64,
    pub public_start_time: u64,
    pub end_time: u64,
}
/**
* @param vault_token_name: Name of vault
* @param owner_address: address of vault owner
* @param base_token: address of base token
* @param token_program: address of token vault
* @param realx: address of RealboxNFT
* @param state: state of Vault
* @param tx_info: Transaction info

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
    pub token_program: Pubkey,
    pub realx: Pubkey,
    pub state: CrowdFundingState,
    pub tx_info: Vec<TxInfo>,
    pub sales_info: SalesInfo,
    // supply info
    pub current_supply: u64,
    pub total_supply: u64,
    // treasury
    pub treasury_address: Pubkey,
    pub treasury_fee: u64,
}

impl RealboxVaultState {
    /**
     * @dev Current state of the crowdfunding
     */
    pub fn current_state(&self) -> CrowdFundingState {
        let state = self.state;
        let private_start_time = self.sales_info.private_start_time;
        let public_start_time = self.sales_info.public_start_time;
        let end_time = self.sales_info.end_time;
        let current_time = current_timestamp().unwrap() as u64;

        if state == CrowdFundingState::Initialized {
            if current_time >= end_time {
                return CrowdFundingState::Ended;
            };
            if current_time >= public_start_time {
                return CrowdFundingState::PublicStarted;
            };
            if current_time >= private_start_time {
                return CrowdFundingState::PrivateStarted;
            }
        }
        return state;
    }

    pub fn only_state(&self, state: CrowdFundingState) -> bool {
        if self.current_state() == state {
            return true;
        }
        return false;
    }
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
