use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
};
use crate::state::accounts::*;
use crate::errors::ErrorCode;

// Defines a function to transfer funds between accounts
pub fn transfer(
    ctx: Context<Transfer>,  // Context object containing accounts and program ID
    amount: u64              // Amount to transfer
) -> Result<()> {           // Returns a Result indicating success or failure
    let mut key_exist: bool = false;
    // Check if user key exists in the main account's list of keys
    for i in ctx.accounts.main_account.vec_keys.clone() {
        if i == ctx.accounts.user.key() { key_exist = true }
    }
    // Check if user key does not exist in the main account's list of keys
    require!(key_exist == false, ErrorCode::PubkeyError);
    // Get index of user key in the main account's list of keys
    let index = ctx.accounts.main_account.vec_keys
        .clone().iter().position(|&x| x == ctx.accounts.user.key()).unwrap();
    // Generate program derived address (PDA) for the "Main Account" seed
    let (pda, _bump) = Pubkey::find_program_address(&[b"Main Account"], ctx.program_id);
    // Check if transfer amount is less than or equal to the amount in the user's account
    require!(amount <= ctx.accounts.main_account.vec_ammount[index], ErrorCode::AmountError);
    // Check if the PDA matches the main account's account info
    require!(pda == ctx.accounts.main_account_info.key(), ErrorCode::PubkeyError);
    // Mutable reference to the main account
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    // Decrease lamports of the main account info by the transfer amount
    **ctx.accounts.main_account_info.to_account_info().try_borrow_mut_lamports()? -= amount;
    // Increase lamports of the receiver account info by the transfer amount
    **ctx.accounts.to.to_account_info().try_borrow_mut_lamports()? += amount;
    // Decrease the user's account balance by the transfer amount
    main_account.vec_ammount[index] -= amount;
    // Decrease the total amount in the main account by the transfer amount
    main_account.total_ammount -= amount;
    // Return success
    Ok(())
}

#[derive(Accounts)]
// Defines a struct containing accounts required for the transfer function
pub struct Transfer<'info> {
    #[account(mut, seeds = [b"Main Account"], bump = main_account.bump_original)]
    pub main_account: Account<'info, MainAccount>,   // Main account to transfer funds from
    #[account(mut)]
    pub user: Signer<'info>,    // Signer account of the user transferring funds
    /// CHECK: This is account is the receiver
    #[account(mut)]
    pub to: AccountInfo<'info>, // Account info of the receiver account
    /// CHECK: This is the main_account
    #[account(mut)]
    pub main_account_info: AccountInfo<'info>,   // Account info of the main account
    pub system_program: Program<'info, System>,  // System program account
}
