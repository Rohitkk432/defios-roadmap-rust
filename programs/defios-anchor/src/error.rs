use anchor_lang::error_code;

#[error_code]
pub enum DefiOSError {
    // 6000
    #[msg("Invalid Signature")]
    SignatureVerificationFailed,

    // 6004
    #[msg("Insufficient funds for staking")]
    InsufficientStakingFunds,

    // 6005
    #[msg("Cannot stake/unstake on a closed objective")]
    ObjectiveClosedAlready
}
