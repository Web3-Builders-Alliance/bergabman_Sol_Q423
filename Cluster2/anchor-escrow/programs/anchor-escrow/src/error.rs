use anchor_lang::error_code;

#[error_code]
pub enum EscrowError {
    #[msg("Deposit to escrow failed")]
    DepositFailed,
    #[msg("Refund from escrow failed")]
    RefundFailed,
    #[msg("Withdraw of tokens failed")]
    WithdrawFailed,
    #[msg("Unable to get escrow bump")]
    EscrowBumpError,
    #[msg("Failed to close the vault account")]
    CloseVaultFailed,
}
