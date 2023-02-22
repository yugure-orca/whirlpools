use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

use crate::state::Whirlpool;

#[derive(Accounts)]
#[instruction(reward_index: u8)]
pub struct InitializeReward<'info> {
    #[account(address = whirlpool.reward_infos[reward_index as usize].authority)]
    pub reward_authority: Signer<'info>,

    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(mut)]
    pub whirlpool: Box<Account<'info, Whirlpool>>,

    pub reward_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = funder,
        token::mint = reward_mint,
        token::authority = whirlpool
    )]
    pub reward_vault: Box<Account<'info, TokenAccount>>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

// v0.22.0 breaking, ProgramResult --> Result<()>
// https://github.com/coral-xyz/anchor/blob/9044b9b8cde7be87cc9c1ca1867b9a5f2791e103/CHANGELOG.md#breaking-5
pub fn handler(ctx: Context<InitializeReward>, reward_index: u8) -> Result<()> {
    let whirlpool = &mut ctx.accounts.whirlpool;

    Ok(whirlpool.initialize_reward(
        reward_index as usize,
        ctx.accounts.reward_mint.key(),
        ctx.accounts.reward_vault.key(),
    )?)
}
