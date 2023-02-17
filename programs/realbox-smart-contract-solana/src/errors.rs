use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("RealboxVaultFactory: treasuryFee must not be higher than 10000 (100%)")]
    TreasuryFeeTooBig,
    #[msg("RealboxVaultFactory: privateStartTime must be higher than current time")]
    PrivateTimeLowerThanCurrentTime,
    #[msg("RealboxVaultFactory: publicStartTime must be higher than privateStartTime")]
    PublicTimeLowerThanPrivateTime,
    #[msg("RealboxVaultFactory: publicStartTime must be lower than endTime")]
    EndTimeLowerThanPublicTime,
    #[msg("RealboxVaultFactory: minSupply must not be higher than maxSupply")]
    InvalidSupply,
}
