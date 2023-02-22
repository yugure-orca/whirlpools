use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(start_tick_index: i32)]
pub struct InitializeTickArray<'info> {
    pub whirlpool: Account<'info, Whirlpool>,

    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(
      init,
      payer = funder,
      seeds = [b"tick_array", whirlpool.key().as_ref(), start_tick_index.to_string().as_bytes()],
      bump,
      space = TickArray::LEN)]
    pub tick_array: AccountLoader<'info, TickArray>,

    pub system_program: Program<'info, System>,
}

// v0.22.0 breaking, ProgramResult --> Result<()>
// https://github.com/coral-xyz/anchor/blob/9044b9b8cde7be87cc9c1ca1867b9a5f2791e103/CHANGELOG.md#breaking-5
pub fn handler(ctx: Context<InitializeTickArray>, start_tick_index: i32) -> Result<()> {
    let mut tick_array = ctx.accounts.tick_array.load_init()?;
    Ok(tick_array.initialize(&ctx.accounts.whirlpool, start_tick_index)?)
}
