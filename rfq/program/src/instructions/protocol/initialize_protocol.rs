use crate::{
    seeds::PROTOCOL_SEED,
    state::{FeeParameters, ProtocolState},
};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct InitializeProtocolAccounts<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [PROTOCOL_SEED.as_bytes()],
        space = ProtocolState::get_allocated_size(),
        bump
    )]
    pub protocol: Box<Account<'info, ProtocolState>>,
    /// CHECK: is a valid risk engine program id
    #[account(executable)]
    pub risk_engine: AccountInfo<'info>,
    pub collateral_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

fn validate(settle_fees: FeeParameters, default_fees: FeeParameters) -> Result<()> {
    settle_fees.validate()?;
    default_fees.validate()?;

    Ok(())
}

pub fn initialize_protocol_instruction(
    ctx: Context<InitializeProtocolAccounts>,
    settle_fees: FeeParameters,
    default_fees: FeeParameters,
    asset_add_fee: u64,
) -> Result<()> {
    let protocol = &mut ctx.accounts.protocol;
    *protocol = ProtocolState {
        authority: ctx.accounts.signer.key(),
        bump: *ctx.bumps.get("protocol").unwrap(),
        active: true,
        settle_fees,
        default_fees,
        risk_engine: Pubkey::default(),
        collateral_mint: Pubkey::default(),
        instruments: Vec::new(),
        asset_add_fee,
        reserved: [0; 1016],
        supported_token_programs: Vec::new(),
    };
    Ok(())
}
