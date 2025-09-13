#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

pub mod context;
pub use context::*;
pub mod state;
pub use state::*;


declare_id!("9fBuiMJMms1rHv1SNbCPrYToqTdjrspXM2DFM6VZNkQ8");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result <()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result <()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result <()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result <()> {
        ctx.accounts.close()?;
        Ok(())
    }
}
