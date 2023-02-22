use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(tick_spacing: u16)]
pub struct InitializeFeeTier<'info> {
    pub config: Box<Account<'info, WhirlpoolsConfig>>,

    #[account(init,
      payer = funder,
      seeds = [b"fee_tier", config.key().as_ref(),
               tick_spacing.to_le_bytes().as_ref()],
      bump,
      space = FeeTier::LEN)]
    pub fee_tier: Account<'info, FeeTier>,

    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(address = config.fee_authority)]
    pub fee_authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// v0.22.0 breaking, ProgramResult --> Result<()>
// https://github.com/coral-xyz/anchor/blob/9044b9b8cde7be87cc9c1ca1867b9a5f2791e103/CHANGELOG.md#breaking-5
pub fn handler(
    ctx: Context<InitializeFeeTier>,
    tick_spacing: u16,
    default_fee_rate: u16,
) -> Result<()> {
    Ok(ctx
        .accounts
        .fee_tier
        .initialize(&ctx.accounts.config, tick_spacing, default_fee_rate)?)
}
