#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

pub mod context;
pub use context::*;

pub mod state;
pub use state::*;

declare_id!("2iRsw73vREFH28tzF633NgBfFGiiHS9adwtqkUoFxCyr");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result <()> {
        ctx.accounts.deposit(deposit)?;
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }
    
    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()
    }
}
