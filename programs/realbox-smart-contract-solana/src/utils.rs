use anchor_lang::prelude::*;

pub fn current_timestamp() -> Option<i64> {
    let clock = Clock::get().ok()?;
    Some(clock.unix_timestamp)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum SalesChannels {
    DirectOnchain,
    DirectOffchain,
    Indirect,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum CrowdFundingState {
    Uninitialized,
    Initialized,
    PrivateStarted,
    PublicStarted,
    Ended,
    Finalized,
    Canceled,
    Unfrozen,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum TokenState {
    /// Token account is unlocked; operations are allowed on this account.
    Unlocked,
    /// Token account has been locked; no operations are allowed on this account.
    Locked,
    /// Token account has a `Sale` delegate set; operations are restricted.
    Listed,
}
