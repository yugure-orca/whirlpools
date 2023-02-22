use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

use crate::{state::*, util::mint_position_token_and_remove_authority};

#[derive(Accounts)]
#[instruction(bumps: OpenPositionBumps)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,

    // to avoid compile error...
    /// CHECK: the account that will be the owner of the position can be arbitrary
    pub owner: UncheckedAccount<'info>,

    #[account(init,
      payer = funder,
      space = Position::LEN,
      seeds = [b"position".as_ref(), position_mint.key().as_ref()],
      // v0.21.0 breaking, providing a target bump is not allowed (with init)
      // https://github.com/coral-xyz/anchor/blob/9044b9b8cde7be87cc9c1ca1867b9a5f2791e103/CHANGELOG.md#breaking-6
      bump,
    )]
    pub position: Box<Account<'info, Position>>,

    #[account(init,
        payer = funder,
        // error: space is not required for initializing an spl account
        // v0.23.0 breaking, Remove space calculation using ... #1519
        // https://github.com/coral-xyz/anchor/blob/9044b9b8cde7be87cc9c1ca1867b9a5f2791e103/CHANGELOG.md#breaking-4
        mint::authority = whirlpool,
        mint::decimals = 0,
    )]
    pub position_mint: Account<'info, Mint>,

    #[account(init,
      payer = funder,
      associated_token::mint = position_mint,
      associated_token::authority = owner,
    )]
    pub position_token_account: Box<Account<'info, TokenAccount>>,

    pub whirlpool: Box<Account<'info, Whirlpool>>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

/*
  Opens a new Whirlpool Position.
*/
pub fn handler(
    ctx: Context<OpenPosition>,
    _bumps: OpenPositionBumps,
    tick_lower_index: i32,
    tick_upper_index: i32,
) -> Result<()> {
  // v0.22.0 breaking, ProgramResult --> Result<()>
  // https://github.com/coral-xyz/anchor/blob/9044b9b8cde7be87cc9c1ca1867b9a5f2791e103/CHANGELOG.md#breaking-5

    let whirlpool = &ctx.accounts.whirlpool;
    let position_mint = &ctx.accounts.position_mint;
    let position = &mut ctx.accounts.position;

    position.open_position(
        whirlpool,
        position_mint.key(),
        tick_lower_index,
        tick_upper_index,
    )?;

    mint_position_token_and_remove_authority(
        whirlpool,
        position_mint,
        &ctx.accounts.position_token_account,
        &ctx.accounts.token_program,
    )
}
