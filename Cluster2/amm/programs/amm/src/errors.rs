use anchor_lang::error_code;
use constant_product_curve::CurveError;

#[error_code]
pub enum AmmError {
    #[msg("DefaultError")]
    DefaultError,
    #[msg("BumpError")]
    BumpError,
    #[msg("InvalidFee")]
    InvalidFee,
    #[msg("Offer expired")]
    OfferExpired,
    #[msg("Zero balance")]
    ZeroBalance,
    #[msg("Pool locked")]
    PoolLocked,
    #[msg("Curve error")]
    CurveError,
}

impl From<CurveError> for AmmError {
    fn from(_: CurveError) -> Self {
        AmmError::CurveError
    }
}
