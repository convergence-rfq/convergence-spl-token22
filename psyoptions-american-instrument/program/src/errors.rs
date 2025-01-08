use anchor_lang::prelude::*;

#[error_code]
pub enum PsyoptionsAmericanError {
    #[msg("Invalid data size")]
    InvalidDataSize,
    #[msg("Passed mint account does not match")]
    PassedMintDoesNotMatch,
    #[msg("Passed account is not an associated token account of a receiver")]
    InvalidReceiver,
    #[msg("Passed address is not of a party first to prepare for settlement")]
    NotFirstToPrepare,
    #[msg("Passed metadata account does not match")]
    PassedAmericanMetaDoesNotMatch,
    #[msg("Passed underlying amount per contract does not match")]
    PassedUnderlyingAmountPerContractDoesNotMatch,
    #[msg("Passed underlying amount per contract decimals does not match")]
    PassedUnderlyingAmountPerContractDecimalsDoesNotMatch,
    #[msg("Passed strike price does not match")]
    PassedStrikePriceDoesNotMatch,
    #[msg("Passed strike price decimals does not match")]
    PassedStrikePriceDecimalsDoesNotMatch,
    #[msg("Passed expiration timestamp does not match")]
    PassedExpirationTimestampDoesNotMatch,
    #[msg("Stablecoin as base asset is not supported")]
    StablecoinAsBaseAssetIsNotSupported,
    #[msg("Passed decimals does not match")]
    DecimalsAmountDoesNotMatch,
    #[msg("Base Asset doesnt match")]
    BaseAssetDoesNotMatch,
    #[msg("Invalid token program")]
    InvalidTokenProgram,
}
