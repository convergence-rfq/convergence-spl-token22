use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenAccount;

use crate::errors::ProtocolError;

use super::protocol::BaseAssetIndex;

#[account]
pub struct Rfq {
    pub taker: Pubkey,
    pub maker: Pubkey,
    pub maker_token: Pubkey,
    pub taker_token: Pubkey,
    pub token_program: Pubkey,
    pub order_type: OrderType,
    pub fixed_size: FixedSize,
    pub quote_asset: QuoteAsset,
    pub creation_timestamp: i64,

    pub active_window: u32,
    pub settling_window: u32,

    pub expected_legs_size: u16,
    pub expected_legs_hash: [u8; 32],
    pub state: StoredRfqState,
    pub non_response_taker_collateral_locked: u64,
    pub total_taker_collateral_locked: u64,
    pub total_responses: u32,
    pub cleared_responses: u32,
    pub confirmed_responses: u32,
    pub whitelist: Pubkey,

    pub reserved: [u8; 224],

    pub print_trade_provider: Option<Pubkey>, // move higher after replacing with nullable wrapper
    pub legs: Vec<Leg>,                       // TODO add limit for this size
}

impl Rfq {
    pub const MAX_LEGS_AMOUNT: u8 = 25;
    pub const MAX_LEGS_SIZE: u16 = 4096;
    pub const LEN: usize = 224 + 32;

    pub fn is_settled_as_print_trade(&self) -> bool {
        self.print_trade_provider.is_some()
    }

    pub fn get_state(&self) -> Result<RfqState> {
        let state = match self.state {
            StoredRfqState::Constructed => RfqState::Constructed,
            StoredRfqState::ValidatedByPrintTradeProvider => {
                RfqState::ValidatedByPrintTradeProvider
            }
            StoredRfqState::Active => {
                let current_time = Clock::get()?.unix_timestamp;
                if !self.active_window_ended(current_time) {
                    RfqState::Active
                } else if self.confirmed_responses == 0 {
                    RfqState::Expired
                } else if !self.settle_window_ended(current_time) {
                    RfqState::Settling
                } else {
                    RfqState::SettlingEnded
                }
            }
            StoredRfqState::Canceled => RfqState::Canceled,
        };
        Ok(state)
    }

    pub fn active_window_ended(&self, current_time: i64) -> bool {
        current_time >= self.creation_timestamp + self.active_window as i64
    }

    pub fn settle_window_ended(&self, current_time: i64) -> bool {
        current_time >= self.get_settle_window_end()
    }

    pub fn get_settle_window_end(&self) -> i64 {
        self.creation_timestamp + self.active_window as i64 + self.settling_window as i64
    }

    pub fn is_fixed_size(&self) -> bool {
        matches!(self.fixed_size, FixedSize::BaseAsset { .. })
    }

    pub fn get_asset_instrument_data(&self, asset_identifier: AssetIdentifier) -> &Vec<u8> {
        match asset_identifier {
            AssetIdentifier::Leg { leg_index } => &self.legs[leg_index as usize].data,
            AssetIdentifier::Quote => &self.quote_asset.data,
        }
    }

    pub fn get_asset_instrument_index(&self, asset_identifier: AssetIdentifier) -> Option<u8> {
        match asset_identifier {
            AssetIdentifier::Leg { leg_index } => self.legs[leg_index as usize]
                .settlement_type_metadata
                .get_instrument_index(),
            AssetIdentifier::Quote => self
                .quote_asset
                .settlement_type_metadata
                .get_instrument_index(),
        }
    }

    pub fn validate_token_accounts(
        &self,
        maker_token_account: &Account<TokenAccount>,
        taker_token_account: &Account<TokenAccount>,
    ) -> Result<()> {
        require!(
            maker_token_account.mint == self.maker_token 
            && taker_token_account.mint == self.taker_token,
            ProtocolError::InvalidTokenAccount
        );
        
        require!(
            maker_token_account.owner == self.maker 
            && taker_token_account.owner == self.taker,
            ProtocolError::InvalidTokenAccount
        );
        
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct QuoteAsset {
    pub pubkey: Pubkey,
    pub settlement_type_metadata: SettlementTypeMetadata,
    pub data: Vec<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Leg {
    pub settlement_type_metadata: SettlementTypeMetadata,
    pub base_asset_index: BaseAssetIndex,
    pub data: Vec<u8>,
    pub amount: u64,
    pub amount_decimals: u8,
    pub side: LegSide,

    pub reserved: [u8; 64],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ApiLeg {
    pub settlement_type_metadata: SettlementTypeMetadata,
    pub base_asset_index: BaseAssetIndex,
    pub data: Vec<u8>,
    pub amount: u64,
    pub amount_decimals: u8,
    pub side: LegSide,
}

impl From<ApiLeg> for Leg {
    fn from(value: ApiLeg) -> Self {
        let ApiLeg {
            settlement_type_metadata,
            base_asset_index,
            data,
            amount,
            amount_decimals,
            side,
        } = value;

        Self {
            settlement_type_metadata,
            base_asset_index,
            data,
            amount,
            amount_decimals,
            side,
            reserved: [0; 64],
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub enum SettlementTypeMetadata {
    Instrument { instrument_index: u8 },
    PrintTrade { instrument_type: u8 },
}

impl SettlementTypeMetadata {
    pub fn is_empty(&self) -> bool {
        match self {
            SettlementTypeMetadata::PrintTrade { .. } => true,
            SettlementTypeMetadata::Instrument { .. } => false,
        }
    }

    pub fn get_instrument_index(&self) -> Option<u8> {
        match self {
            SettlementTypeMetadata::Instrument { instrument_index } => Some(*instrument_index),
            SettlementTypeMetadata::PrintTrade { .. } => None,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub enum FixedSize {
    None { padding: u64 },
    BaseAsset { legs_multiplier_bps: u64 },
    QuoteAsset { quote_amount: u64 },
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub enum OrderType {
    Buy,
    Sell,
    TwoWay,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub enum StoredRfqState {
    Constructed,
    ValidatedByPrintTradeProvider,
    Active,
    Canceled,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RfqState {
    Constructed,
    ValidatedByPrintTradeProvider,
    Active,
    Canceled,
    Expired,
    Settling,
    SettlingEnded,
}

impl RfqState {
    pub fn assert_state_in<const N: usize>(&self, expected_states: [Self; N]) -> Result<()> {
        if !expected_states.contains(self) {
            msg!(
                "Rfq state: {:?}, expected state: {:?}",
                self,
                expected_states
            );
            err!(ProtocolError::RfqIsNotInRequiredState)
        } else {
            Ok(())
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub enum LegSide {
    Long,
    Short,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AssetIdentifier {
    Leg { leg_index: u8 },
    Quote,
}

impl AssetIdentifier {
    pub fn from_bytes(bytes: (u8, u8)) -> Self {
        match bytes {
            (0, leg_index) => AssetIdentifier::Leg { leg_index },
            (1, 0) => AssetIdentifier::Quote,
            _ => panic!("Invalid bytes for AssetIdentifier"),
        }
    }

    pub fn to_bytes(self) -> [u8; 2] {
        match self {
            AssetIdentifier::Leg { leg_index } => [0, leg_index],
            AssetIdentifier::Quote => [1, 0],
        }
    }
}

impl AnchorSerialize for AssetIdentifier {
    fn serialize<W: std::io::prelude::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.to_bytes())?;

        Ok(())
    }
}

impl AnchorDeserialize for AssetIdentifier {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let bytes: (u8, u8) = AnchorDeserialize::deserialize(buf)?;
        Ok(Self::from_bytes(bytes))
    }

    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let bytes: (u8, u8) = AnchorDeserialize::deserialize_reader(reader)?;
        Ok(Self::from_bytes(bytes))
    }
}
