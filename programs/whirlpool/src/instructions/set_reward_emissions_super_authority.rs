use anchor_lang::prelude::*;

use crate::state::WhirlpoolsConfig;

#[derive(Accounts)]
pub struct SetRewardEmissionsSuperAuthority<'info> {
    #[account(mut)]
    pub whirlpools_config: Account<'info, WhirlpoolsConfig>,

    #[account(address = whirlpools_config.reward_emissions_super_authority)]
    pub reward_emissions_super_authority: Signer<'info>,

    // to avoid compile error...
    /// CHECK: the account that will be new authority can be arbitrary
    pub new_reward_emissions_super_authority: UncheckedAccount<'info>,
}

// v0.22.0 breaking, ProgramResult --> Result<()>
// https://github.com/coral-xyz/anchor/blob/9044b9b8cde7be87cc9c1ca1867b9a5f2791e103/CHANGELOG.md#breaking-5
pub fn handler(ctx: Context<SetRewardEmissionsSuperAuthority>) -> Result<()> {
    Ok(ctx
        .accounts
        .whirlpools_config
        .update_reward_emissions_super_authority(
            ctx.accounts.new_reward_emissions_super_authority.key(),
        ))
}
