use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod state;
pub mod utils;

use instructions::{
    add_account::add_account, create_main_account::create_main_account, transfer::transfer,
};

declare_id!("9s8JNZx3e3LbUaxqAd6ENJVXCzjRgC1oiezTNd6cXxLS");

#[program]
pub mod moneymannage {
    use super::*;
    pub fn create_main_account_(ctx: Context<Create>, amount: u64) -> Result<()> {
        create_main_account(ctx, amount)
    }
    pub fn add_account_(ctx: Context<AddAccount>, amount: u64) -> Result<()> {
        add_account(ctx, amount)
    }
    pub fn transfer_(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        transfer(ctx, amount)
    }
}
