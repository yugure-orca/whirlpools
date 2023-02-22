use anchor_lang::prelude::*;

use crate::state::WhirlpoolsConfig;

#[derive(Accounts)]
pub struct SetDefaultProtocolFeeRate<'info> {
    #[account(mut)]
    pub whirlpools_config: Account<'info, WhirlpoolsConfig>,

    #[account(address = whirlpools_config.fee_authority)]
    pub fee_authority: Signer<'info>,
}

// v0.22.0 breaking, ProgramResult --> Result<()>
// https://github.com/coral-xyz/anchor/blob/9044b9b8cde7be87cc9c1ca1867b9a5f2791e103/CHANGELOG.md#breaking-5
pub fn handler(
    ctx: Context<SetDefaultProtocolFeeRate>,
    default_protocol_fee_rate: u16,
) -> Result<()> {
    Ok(ctx
        .accounts
        .whirlpools_config
        .update_default_protocol_fee_rate(default_protocol_fee_rate)?)
}
