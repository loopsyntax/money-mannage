// Import necessary libraries
use anchor_lang::{
    prelude::*,
    solana_program::{
        pubkey::Pubkey,
        system_instruction,
    }
};
// Import local modules
use crate::state::accounts::*;
use crate::errors::ErrorCode;

// Define a function to create a main account
pub fn create_main_account(
    ctx: Context<Create>, // Context provides information about the transaction and accounts
    amount: u64 // Amount to transfer to the main account
) -> Result<()> { // Result is a type that can hold either a successful value or an error value
    // Check if the user has enough lamports
    require!(amount <= AccountInfo::lamports(&ctx.accounts.user.to_account_info()), ErrorCode::AmountError);

    // Get a mutable reference to the main account
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;

    // Find the program address and bump
    let (pda, bump) = Pubkey::find_program_address(&[b"Main Account"], ctx.program_id);

    // Invoke a system transfer instruction to transfer lamports from user to pda account
    anchor_lang::solana_program::program::invoke(
        &system_instruction::transfer(&ctx.accounts.user.key(), &pda.key(), amount), // Transfer instruction
        &[ctx.accounts.user.to_account_info(), ctx.accounts.main_account_info.to_account_info().clone()], // Accounts to include
    ).expect("Error"); // Handle any errors

    // Update main account fields
    main_account.bump_original = bump;
    main_account.len = 1 + 2 + 32 + 4 + 32 + 32 + 4 + 8 + 8 + 8;
    main_account.authority = ctx.accounts.user.key();
    main_account.vec_keys.push(ctx.accounts.user.key());
    main_account.vec_ammount.push(amount);
    main_account.total_ammount += amount;

    Ok(()) // Return successful value
}

// Define a struct for account initialization
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, seeds = [b"Main Account"], bump, payer = user, space = MainAccount::SIZE + 8)]
    pub main_account: Account<'info, MainAccount>, // Main account to be created
    #[account(mut, signer)] // Mutable and requires a signer
    pub user: AccountInfo<'info>, // User account
    #[account(mut)] // Mutable account
    pub main_account_info: AccountInfo<'info>, // Main account info
    pub system_program: Program<'info, System>, // System program info
}
