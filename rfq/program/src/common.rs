use anchor_lang::prelude::*;
use anchor_spl::{
    token::spl_token,
    token_interface::{TokenAccount, TokenInterface},
};

use crate::{
    errors::{ErrorCode, ProtocolError},
    state::{
        CollateralInfo, Response, Rfq, SettlementTypeMetadata,
    },
};

pub fn validate_settlement_type_metadata(
    metadata: &SettlementTypeMetadata,
    is_settled_as_print_trade: bool,
) -> Result<()> {
    if is_settled_as_print_trade {
        require!(
            metadata.is_empty(),
            ProtocolError::SettlementTypeMetadataIsNotEmpty
        );
    } else {
        require!(
            !metadata.is_empty(),
            ProtocolError::SettlementTypeMetadataIsEmpty
        );
    }
    Ok(())
}

pub fn unlock_response_collateral(
    rfq: &mut Rfq,
    response: &mut Response,
    taker_collateral_info: &mut CollateralInfo,
    maker_collateral_info: &mut CollateralInfo,
) {
    let taker_collateral = response.taker_collateral_locked;
    if taker_collateral > 0 {
        taker_collateral_info.unlock_collateral(taker_collateral);
        response.taker_collateral_locked = 0;
        rfq.total_taker_collateral_locked -= taker_collateral;
    }

    let maker_collateral = response.maker_collateral_locked;
    if maker_collateral > 0 {
        maker_collateral_info.unlock_collateral(maker_collateral);
        response.maker_collateral_locked = 0;
    }
}

pub fn validate_token_program(token_program: &AccountInfo) -> Result<()> {
    if *token_program.key == spl_token::ID || *token_program.key == spl_token_2022::ID {
        Ok(())
    } else {
        Err(ErrorCode::InvalidTokenProgram.into())
    }
}

pub fn transfer_tokens<'a>(
    from: &Account<'a, TokenAccount>,
    to: &Account<'a, TokenAccount>,
    authority: &Signer<'a>,
    token_program: &AccountInfo<'a>,
    amount: u64,
) -> Result<()> {
    validate_token_program(token_program)?;

    anchor_spl::token_interface::transfer(
        CpiContext::new(
            token_program.clone(),
            anchor_spl::token_interface::Transfer {
                from: from.to_account_info(),
                to: to.to_account_info(),
                authority: authority.to_account_info(),
            },
        ),
        amount,
    )
}

// Keep other non-print-trade related functions...
