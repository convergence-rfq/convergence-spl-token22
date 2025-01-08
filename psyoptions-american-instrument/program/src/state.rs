use anchor_lang::prelude::*;
use risk_engine::state::OptionCommonData;
use anchor_spl::token_interface::{TokenAccount, TokenInterface};

pub const TOKEN_DECIMALS: u8 = 0;

#[derive(AnchorDeserialize)]
pub struct ParsedLegData {
    pub option_common_data: OptionCommonData,
    pub mint_address: Pubkey,
    pub american_meta_address: Pubkey,
}

impl ParsedLegData {
    pub const SERIALIZED_SIZE: usize = OptionCommonData::SERIALIZED_SIZE + 32 + 32;
}

#[account]
pub struct OptionMarket {
    pub token_program: Pubkey,
}

impl OptionMarket {
    pub fn validate_token_program(&self, token_program: &Pubkey) -> Result<()> {
        require!(
            *token_program == spl_token::ID || *token_program == spl_token_2022::ID,
            ErrorCode::InvalidTokenProgram
        );
        Ok(())
    }
}
