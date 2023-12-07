use anchor_lang::prelude::*;

pub mod contexts;
use contexts::*;

pub mod state;
pub mod error;

declare_id!("7NBN3gJgmoe2x9un3KKH7yeEvV3TSrDavvQ9SN8VvQ3D");

#[program]
pub mod anchor_escrow {

    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.deposit(deposit)?;
        ctx.accounts.save_escrow(seed, receive, ctx.bumps.escrow)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw()?;
        ctx.accounts.close_vault()?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund()?;
        ctx.accounts.close_vault()?;
        Ok(())
    }
}
