use anchor_lang::prelude::*;

use crate::state::WhirlpoolsConfig;

#[derive(Accounts)]
pub struct SetFeeAuthority<'info> {
    #[account(mut)]
    pub whirlpools_config: Account<'info, WhirlpoolsConfig>,

    #[account(address = whirlpools_config.fee_authority)]
    pub fee_authority: Signer<'info>,

    // to avoid compile error...
    /// CHECK: the account that will be new authority can be arbitrary
    pub new_fee_authority: UncheckedAccount<'info>,
}

/// Set the fee authority. Only the current fee authority has permission to invoke this instruction.
pub fn handler(ctx: Context<SetFeeAuthority>) -> Result<()> {
    // v0.22.0 breaking, ProgramResult --> Result<()>
    // https://github.com/coral-xyz/anchor/blob/9044b9b8cde7be87cc9c1ca1867b9a5f2791e103/CHANGELOG.md#breaking-5

    Ok(ctx
        .accounts
        .whirlpools_config
        .update_fee_authority(ctx.accounts.new_fee_authority.key()))
}
