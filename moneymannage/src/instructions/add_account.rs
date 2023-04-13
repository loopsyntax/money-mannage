use anchor_lang::{
    prelude::*,
    solana_program::{
        pubkey::Pubkey,
        system_instruction,
    }
};
use crate::state::accounts::*;
use crate::errors::ErrorCode;

// Define a function that adds a new account.
// The function takes a `Context` object, which holds all accounts related to the transaction,
// and an `amount` that is the number of lamports to transfer to the new account.
pub fn add_account(
    ctx: Context<AddAccount>,
    amount: u64
) -> Result<()> {
    // Check if the user's public key already exists in the main account's list of keys.
    let mut key_exist: bool = false;
    for i in ctx.accounts.main_account.vec_keys.clone() {
        if i == ctx.accounts.user.key() { key_exist = true }
    }

    // Check if the amount to be transferred is less than or equal to the user's account balance.
    require!(amount <= AccountInfo::lamports(&ctx.accounts.user.to_account_info()), ErrorCode::AmountError);

    // Check if the user's public key does not already exist in the main account's list of keys.
    require!(key_exist == false, ErrorCode::PubkeyError);

    // Get a mutable reference to the main account.
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;

    // Generate a program-derived address (PDA) for the main account.
    let (pda, _bump) = Pubkey::find_program_address(&[b"Main Account"], ctx.program_id);

    // Transfer the specified amount of lamports from the user's account to the PDA.
    anchor_lang::solana_program::program::invoke(
        &system_instruction::transfer(&ctx.accounts.user.key(), &pda.key(), amount),
        &[ctx.accounts.user.to_account_info(), ctx.accounts.main_account_info.to_account_info().clone()],
    ).expect("Error");

    // Add the user's public key and the amount transferred to the main account's lists.
    main_account.vec_keys.push(ctx.accounts.user.key());
    main_account.vec_ammount.push(amount);
    main_account.total_ammount += amount;
    main_account.len += 40;

    // Return success.
    Ok(())
}

// Define a struct that represents the accounts needed for adding a new account.
#[derive(Accounts)]
pub struct AddAccount<'info> {
    #[account(
        mut,
        seeds = [b"Main Account"],
        bump = main_account.bump_original,
        realloc = 8 + main_account.len as usize + 40,
        realloc::payer = user,
        realloc::zero = false,
    )]
    pub main_account: Account<'info, MainAccount>,

    // This account must be signed by the user.
    #[account(mut, signer)]
    pub user: AccountInfo<'info>,

    // This is the main account that is being modified.
    #[account(mut)]
    pub main_account_info: AccountInfo<'info>,

    // This is the Solana System program.
    pub system_program: Program<'info, System>,
}
