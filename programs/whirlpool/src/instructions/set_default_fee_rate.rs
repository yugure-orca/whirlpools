use anchor_lang::prelude::*;

use crate::state::{FeeTier, WhirlpoolsConfig};

#[derive(Accounts)]
pub struct SetDefaultFeeRate<'info> {
    pub whirlpools_config: Account<'info, WhirlpoolsConfig>,

    #[account(mut, has_one = whirlpools_config)]
    pub fee_tier: Account<'info, FeeTier>,

    #[account(address = whirlpools_config.fee_authority)]
    pub fee_authority: Signer<'info>,
}

/*
   Updates the default fee rate on a FeeTier object.
*/
pub fn handler(ctx: Context<SetDefaultFeeRate>, default_fee_rate: u16) -> Result<()> {
    // v0.22.0 breaking, ProgramResult --> Result<()>
    // https://github.com/coral-xyz/anchor/blob/9044b9b8cde7be87cc9c1ca1867b9a5f2791e103/CHANGELOG.md#breaking-5

    Ok(ctx
        .accounts
        .fee_tier
        .update_default_fee_rate(default_fee_rate)?)
}
