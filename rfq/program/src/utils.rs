use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount, TokenInterface};

use crate::errors::ProtocolError;
use crate::errors::ErrorCode;

pub trait ToAccountMeta {
    fn to_account_meta(&self) -> AccountMeta;
}

impl<'info> ToAccountMeta for AccountInfo<'info> {
    fn to_account_meta(&self) -> AccountMeta {
        match self.is_writable {
            false => AccountMeta::new_readonly(*self.key, self.is_signer),
            true => AccountMeta::new(*self.key, self.is_signer),
        }
    }
}

pub fn default_as_none<T: Default + PartialEq>(value: T) -> Option<T> {
    if value != Default::default() {
        Some(value)
    } else {
        None
    }
}

pub fn none_as_default<T: Default + PartialEq>(value: Option<T>) -> Result<T> {
    match value {
        Some(value) if value == Default::default() => {
            err!(ProtocolError::DefaultValueIsNotPermitted)
        }
        Some(value) => Ok(value),
        None => Ok(Default::default()),
    }
}

pub fn validate_token_account_owner(
    token_account: &Account<TokenAccount>,
    expected_owner: &Pubkey,
) -> Result<()> {
    require!(
        token_account.owner == *expected_owner,
        ErrorCode::InvalidTokenAccount
    );
    Ok(())
}

pub fn validate_token_account_mint(
    token_account: &Account<TokenAccount>,
    expected_mint: &Pubkey,
) -> Result<()> {
    require!(
        token_account.mint == *expected_mint,
        ErrorCode::InvalidTokenAccount
    );
    Ok(())
}

pub fn validate_token_account(account: &Account<TokenAccount>) -> Result<()> {
    if account.amount == 0 {
        return Err(ErrorCode::InvalidTokenAccount.into());
    }
    Ok(())
}
