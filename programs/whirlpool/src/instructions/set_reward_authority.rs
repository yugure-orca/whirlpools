use anchor_lang::prelude::*;

use crate::state::Whirlpool;

#[derive(Accounts)]
#[instruction(reward_index: u8)]
pub struct SetRewardAuthority<'info> {
    #[account(mut)]
    pub whirlpool: Account<'info, Whirlpool>,

    #[account(address = whirlpool.reward_infos[reward_index as usize].authority)]
    pub reward_authority: Signer<'info>,

    // to avoid compile error...
    /// CHECK: the account that will be new authority can be arbitrary
    pub new_reward_authority: UncheckedAccount<'info>,
}

// v0.22.0 breaking, ProgramResult --> Result<()>
// https://github.com/coral-xyz/anchor/blob/9044b9b8cde7be87cc9c1ca1867b9a5f2791e103/CHANGELOG.md#breaking-5
pub fn handler(ctx: Context<SetRewardAuthority>, reward_index: u8) -> Result<()> {
    Ok(ctx.accounts.whirlpool.update_reward_authority(
        reward_index as usize,
        ctx.accounts.new_reward_authority.key(),
    )?)
}
