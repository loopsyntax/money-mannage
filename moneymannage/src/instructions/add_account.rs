use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
    solana_program::system_instruction,
}; 
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn add_account(
    ctx: Context<AddAccount>,
    amount: u64
) -> Result<()> {
    let mut key_exist: bool = false;
    for i in ctx.accounts.main_account.vec_keys.clone() {
    if i == ctx.accounts.user.key() { key_exist = true }
}
    require!(amount <= AccountInfo::lamports(&ctx.accounts.user.to_account_info()), ErrorCode::AmountError);
    require!(key_exist == false, ErrorCode::PubkeyError);
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    let (pda, _bump) = Pubkey::find_program_address(&[b"Main Account"], ctx.program_id);
    anchor_lang::solana_program::program::invoke(
        &system_instruction::transfer(&ctx.accounts.user.key(), &pda.key(), amount),
        &[ctx.accounts.user.to_account_info(), ctx.accounts.main_account_info.to_account_info().clone()],
    ).expect("Error");
    main_account.vec_keys.push(ctx.accounts.user.key());
    main_account.vec_ammount.push(amount);
    main_account.total_ammount += amount;
    main_account.len += 40;
    Ok(())
}

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
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub user: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub main_account_info: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}