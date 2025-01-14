use crate::{
    common::transfer_collateral_token,
    errors::ProtocolError,
    seeds::{COLLATERAL_SEED, COLLATERAL_TOKEN_SEED, PROTOCOL_SEED},
    state::{CollateralInfo, ProtocolState},
};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct WithdrawCollateralAccounts<'info> {
    pub user: Signer<'info>,
    #[account(mut, constraint = user_tokens.mint == protocol.collateral_mint
                @ ProtocolError::NotACollateralTokenAccount)]
    pub user_tokens: Account<'info, TokenAccount>,

    #[account(seeds = [PROTOCOL_SEED.as_bytes()], bump = protocol.bump)]
    pub protocol: Box<Account<'info, ProtocolState>>,
    #[account(seeds = [COLLATERAL_SEED.as_bytes(), user.key().as_ref()], bump = collateral_info.bump)]
    pub collateral_info: Account<'info, CollateralInfo>,
    #[account(mut, seeds = [COLLATERAL_TOKEN_SEED.as_bytes(), user.key().as_ref()],
                bump = collateral_info.token_account_bump)]
    pub collateral_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, TokenInterface>,
}

fn validate(ctx: &Context<WithdrawCollateralAccounts>, amount: u64) -> Result<()> {
    let WithdrawCollateralAccounts {
        collateral_info,
        collateral_token,
        ..
    } = &ctx.accounts;

    require!(
        amount <= collateral_token.amount - collateral_info.locked_tokens_amount,
        ProtocolError::NotEnoughCollateral
    );

    Ok(())
}

pub fn withdraw_collateral_instruction(
    ctx: Context<WithdrawCollateralAccounts>,
    amount: u64,
) -> Result<()> {
    validate(&ctx, amount)?;

    let WithdrawCollateralAccounts {
        user_tokens,
        collateral_info,
        collateral_token,
        token_program,
        ..
    } = ctx.accounts;

    transfer_collateral_token(
        amount,
        collateral_token,
        user_tokens,
        collateral_info,
        token_program,
    )?;

    Ok(())
}

pub fn transfer_collateral_token<'info>(
    amount: u64,
    from: &Account<'info, TokenAccount>,
    to: &Account<'info, TokenAccount>,
    collateral_info: &Account<'info, CollateralInfo>,
    token_program: &Program<'info, TokenInterface>,
) -> Result<()> {
    anchor_spl::token_interface::transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            anchor_spl::token_interface::Transfer {
                from: from.to_account_info(),
                to: to.to_account_info(),
                authority: collateral_info.to_account_info(),
            },
            &[],
        ),
        amount,
    )
}
