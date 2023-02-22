use anchor_lang::prelude::*;

use crate::state::WhirlpoolsConfig;

#[derive(Accounts)]
pub struct SetCollectProtocolFeesAuthority<'info> {
    #[account(mut)]
    pub whirlpools_config: Account<'info, WhirlpoolsConfig>,

    #[account(address = whirlpools_config.collect_protocol_fees_authority)]
    pub collect_protocol_fees_authority: Signer<'info>,

    // to avoid compile error...
    /// CHECK: the account that will be new authority can be arbitrary
    pub new_collect_protocol_fees_authority: UncheckedAccount<'info>,
}

// v0.22.0 breaking, ProgramResult --> Result<()>
// https://github.com/coral-xyz/anchor/blob/9044b9b8cde7be87cc9c1ca1867b9a5f2791e103/CHANGELOG.md#breaking-5
pub fn handler(ctx: Context<SetCollectProtocolFeesAuthority>) -> Result<()> {
    Ok(ctx
        .accounts
        .whirlpools_config
        .update_collect_protocol_fees_authority(
            ctx.accounts.new_collect_protocol_fees_authority.key(),
        ))
}
