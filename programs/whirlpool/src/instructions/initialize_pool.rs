use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

#[derive(Accounts)]
#[instruction(bumps: WhirlpoolBumps, tick_spacing: u16)]
pub struct InitializePool<'info> {
    pub whirlpools_config: Box<Account<'info, WhirlpoolsConfig>>,

    pub token_mint_a: Account<'info, Mint>,
    pub token_mint_b: Account<'info, Mint>,

    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(init,
      seeds = [
        b"whirlpool".as_ref(),
        whirlpools_config.key().as_ref(),
        token_mint_a.key().as_ref(),
        token_mint_b.key().as_ref(),
        tick_spacing.to_le_bytes().as_ref()
      ],
      // v0.21.0 breaking, providing a target bump is not allowed (with init)
      // https://github.com/coral-xyz/anchor/blob/9044b9b8cde7be87cc9c1ca1867b9a5f2791e103/CHANGELOG.md#breaking-6
      bump,
      payer = funder,
      space = Whirlpool::LEN)]
    pub whirlpool: Box<Account<'info, Whirlpool>>,

    #[account(init,
      payer = funder,
      token::mint = token_mint_a,
      token::authority = whirlpool)]
    pub token_vault_a: Box<Account<'info, TokenAccount>>,

    #[account(init,
      payer = funder,
      token::mint = token_mint_b,
      token::authority = whirlpool)]
    pub token_vault_b: Box<Account<'info, TokenAccount>>,

    #[account(has_one = whirlpools_config)]
    pub fee_tier: Account<'info, FeeTier>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

// v0.22.0 breaking, ProgramResult --> Result<()>
// https://github.com/coral-xyz/anchor/blob/9044b9b8cde7be87cc9c1ca1867b9a5f2791e103/CHANGELOG.md#breaking-5
pub fn handler(
    ctx: Context<InitializePool>,
    // to ignore passed bump now (it may be invalid)
    _bumps: WhirlpoolBumps,
    tick_spacing: u16,
    initial_sqrt_price: u128,
) -> Result<()> {
    let token_mint_a = ctx.accounts.token_mint_a.key();
    let token_mint_b = ctx.accounts.token_mint_b.key();

    let whirlpool = &mut ctx.accounts.whirlpool;
    let whirlpools_config = &ctx.accounts.whirlpools_config;

    let default_fee_rate = ctx.accounts.fee_tier.default_fee_rate;

    // we need to verify bump passed is valid to avoid to store invalid bump into the account
    // case1) verify: ctx.bumps.get("whirlpool") == bump
    // case2) completely ignore it and use ctx.bumps.get("whirlpool")
    // The following is case2.
    let bump = *ctx.bumps.get("whirlpool").unwrap();

    Ok(whirlpool.initialize(
        whirlpools_config,
        bump,
        tick_spacing,
        initial_sqrt_price,
        default_fee_rate,
        token_mint_a,
        ctx.accounts.token_vault_a.key(),
        token_mint_b,
        ctx.accounts.token_vault_b.key(),
    )?)
}
