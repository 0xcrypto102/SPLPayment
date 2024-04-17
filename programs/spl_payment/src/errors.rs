use anchor_lang::error_code;

#[error_code]
pub enum SplPaymentError {
    #[msg("SplPaymentError: Not allowed owner")]
    NotAllowedOwner,

    #[msg("SplPaymentError: Over max deposit amont")]
    MaxDepositAmount,

    #[msg("SplPaymentError: InvalidAmount")]
    InvalidAmount,

    #[msg("SplPaymentError: Should depsoit than 0")]
    ZeroAmount,

    #[msg("SplPaymentError: The token mint address is not correct")]
    InvalidTokenAddress
}