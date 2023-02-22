use crate::state::WhirlpoolsConfig;
use crate::{errors::ErrorCode, math::MAX_FEE_RATE};
use anchor_lang::prelude::*;

#[account]
pub struct FeeTier {
    pub whirlpools_config: Pubkey,
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}

impl FeeTier {
    pub const LEN: usize = 8 + 32 + 4;

    pub fn initialize(
        &mut self,
        whirlpools_config: &Account<WhirlpoolsConfig>,
        tick_spacing: u16,
        default_fee_rate: u16,
        // v0.22.0 breaking, add Anchor's Result (anchor's Result = std Result<T, Error>)
        // https://github.com/coral-xyz/anchor/blob/9044b9b8cde7be87cc9c1ca1867b9a5f2791e103/CHANGELOG.md#breaking-5
        // https://github.com/coral-xyz/anchor/pull/1462
    ) -> Result<()> {
        self.whirlpools_config = whirlpools_config.key();
        self.tick_spacing = tick_spacing;
        self.update_default_fee_rate(default_fee_rate)?;
        Ok(())
    }

    // v0.22.0 breaking, add Anchor's Result (anchor's Result = std Result<T, Error>)
    // https://github.com/coral-xyz/anchor/blob/9044b9b8cde7be87cc9c1ca1867b9a5f2791e103/CHANGELOG.md#breaking-5
    // https://github.com/coral-xyz/anchor/pull/1462
    pub fn update_default_fee_rate(&mut self, default_fee_rate: u16) -> Result<()> {
        if default_fee_rate > MAX_FEE_RATE {
            return Err(ErrorCode::FeeRateMaxExceeded.into());
        }
        self.default_fee_rate = default_fee_rate;

        Ok(())
    }
}
