use crate::GlobalAccount;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

#[derive(Accounts)]
pub struct Create2<'info> {
    #[account(mut, constraint = user.key() == global.params.dex_bot)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        payer = user,
        token::mint = mint,
        token::authority = global,
        seeds = [b"associated-bonding-curve", mint.key().as_ref()],
        bump,
    )]
    pub associated_bonding_curve: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub global: Box<Account<'info, GlobalAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Create2<'info> {
    pub fn mint_to_pool(&self, token_amount: u64) -> Result<()> {
        let global_seeds = &[b"global".as_ref(), &[self.global.bump]];
        let global_signer = [&global_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.mint.to_account_info(),
                to: self.associated_bonding_curve.to_account_info(),
                authority: self.global.to_account_info(),
            },
            &global_signer,
        );
        token::mint_to(cpi_ctx, token_amount)
    }
}