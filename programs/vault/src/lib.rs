use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

pub mod instructions;
pub mod state;

use crate::instructions::*;
use crate::state::*;

declare_id!("3cEBFGTEPx7ybv4jw1vGDnodqEu65HWWfepr3AcurtGP");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()?;
        Ok(())
    }
}

pub mod escrow {
    // use super::*;

    // pub fn make(ctx: Context<Make>, seed: u64, receive_amount: u64) -> Result<()> {
    //     ctx.accounts
    //         .init_escrow_state(seed, receive_amount, ctx.bumps)?;
    //     ctx.accounts.deposit(deposit_amount)?;
    //     Ok(())
    // }

    // // taker wants to swap there tokens
    // // you do not need to store them in your vault
    // just swap!
    // pub fn take(ctx: Context<Take>, amount: u64) -> Result<()> {
    //     ctx.accounts.take(amount)?;
    //     Ok(())
    // }
}
