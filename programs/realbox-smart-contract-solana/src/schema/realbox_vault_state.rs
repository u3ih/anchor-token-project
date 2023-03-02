use crate::errors::ErrorCode;
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
    pub sender: Pubkey,
    pub amount: u64,
    pub unit_price: f64,
    pub channel: SalesChannels,
    pub uid: String,
}

impl TxInfo {
    pub const MAX_ITEMS_AMOUNT: usize = 100;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SalesInfo {
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
* @param tx_infos: Transaction info

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
    pub mint_token: Pubkey,
    pub mint_base: Pubkey,
    pub realx: Pubkey,
    pub state: CrowdFundingState,
    pub tx_infos: Vec<TxInfo>,
    pub sales_info: SalesInfo,
    pub token_state: TokenState,
    // supply info
    pub processed_token: u64,
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

    pub fn buy_token(
        &mut self,
        amount: u64,
        price: f64,
        channel: SalesChannels,
        uid: String,
    ) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);
        require!(
            amount <= self.sales_info.max_supply - self.current_supply,
            ErrorCode::AmountHigherRemainingSupply
        );
        let tx = TxInfo {
            sender: self.owner_address,
            amount,
            unit_price: price,
            channel: channel,
            uid,
        };
        self.tx_infos.push(tx);
        self.current_supply += amount;
        Ok(())
    }

    pub fn unlocked(&mut self) -> Result<()> {
        self.token_state = TokenState::Unlocked;
        self.state = CrowdFundingState::Unfrozen;
        Ok(())
    }

    pub fn check_locked(&self) -> bool {
        if self.token_state == TokenState::Locked {
            return true;
        };
        false
    }
}
