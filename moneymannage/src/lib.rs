use anchor_lang::prelude::*;
use instructions::*;
use crate::errors::ErrorCode;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("9s8JNZx3e3LbUaxqAd6ENJVXCzjRgC1oiezTNd6cXxLS");

#[program]
pub mod moneymannage {
    use super::*;
    pub fn create_main_account(
        ctx: Context<Create>,
        amount: u64
    ) -> Result<()> {
        instructions::create_main_account::create_main_account(
            ctx,
            amount
        )
    }
    pub fn add_account(
        ctx: Context<AddAccount>,
        amount: u64
    ) -> Result<()> {
        instructions::add_account::add_account(
            ctx,
            amount
        )
    }
    pub fn transfer(
        ctx: Context<Transfer>,
        amount: u64
    ) -> Result<()> {
        instructions::transfer::transfer(
            ctx,
            amount
        )
    }
}