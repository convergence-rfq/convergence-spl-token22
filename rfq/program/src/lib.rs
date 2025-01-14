use anchor_lang::prelude::*;

pub mod common;
pub mod errors;
pub mod instructions;
pub mod seeds;
pub mod state;
pub mod utils;

use instructions::*;

declare_id!("RFQzXB4mwNqZ3kEwNbhJgxZgDknrVuTqnNYrKXXGZcN");

#[program]
pub mod rfq {
    use super::*;

    pub fn initialize_protocol(
        ctx: Context<InitializeProtocolAccounts>,
        collateral_mint: Pubkey,
        bump: u8,
        settle_fees: FeeParameters,
        default_fees: FeeParameters,
        asset_add_fee: u64,
    ) -> Result<()> {
        instructions::protocol::initialize_protocol_instruction(
            ctx,
            collateral_mint,
            bump,
            settle_fees,
            default_fees,
            asset_add_fee,
        )
    }

    pub fn create_rfq(
        ctx: Context<CreateRfqAccounts>,
        active_window: u32,
        settling_window: u32,
        expected_legs_size: u16,
        expected_legs_hash: [u8; 32],
        order_type: OrderType,
        fixed_size: FixedSize,
        quote_asset: QuoteAsset,
        legs: Vec<ApiLeg>,
    ) -> Result<()> {
        instructions::rfq::create_rfq_instruction(
            ctx,
            active_window,
            settling_window,
            expected_legs_size,
            expected_legs_hash,
            order_type,
            fixed_size,
            quote_asset,
            legs,
        )
    }

    pub fn respond_to_rfq(
        ctx: Context<RespondToRfqAccounts>,
        quote: Quote,
        taker_collateral_amount: u64,
        maker_collateral_amount: u64,
    ) -> Result<()> {
        instructions::rfq::respond_to_rfq_instruction(
            ctx,
            quote,
            taker_collateral_amount,
            maker_collateral_amount,
        )
    }

    pub fn fund_collateral(ctx: Context<FundCollateralAccounts>, amount: u64) -> Result<()> {
        instructions::collateral::fund_collateral_instruction(ctx, amount)
    }

    pub fn withdraw_collateral(ctx: Context<WithdrawCollateralAccounts>, amount: u64) -> Result<()> {
        instructions::collateral::withdraw_collateral_instruction(ctx, amount)
    }

    // Add other instruction handlers...
}
