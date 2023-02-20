use anchor_lang::prelude::*;

pub fn current_timestamp() -> Option<i64> {
    let clock = Clock::get().ok()?;
    Some(clock.unix_timestamp)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
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
