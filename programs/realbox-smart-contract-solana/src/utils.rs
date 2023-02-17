use anchor_lang::prelude::*;

pub fn current_timestamp() -> Option<i64> {
    let clock = Clock::get().ok()?;
    Some(clock.unix_timestamp)
}

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
