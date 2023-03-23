use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
};
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn transfer(
    ctx: Context<Transfer>,
    amount: u64
) -> Result<()> {
    let mut key_exist: bool = false;
    for i in ctx.accounts.main_account.vec_keys.clone() {
    if i == ctx.accounts.user.key() { key_exist = true }
}
    require!(key_exist == false, ErrorCode::PubkeyError);
    let index = ctx.accounts.main_account.vec_keys
    .clone().iter().position(|&x| x == ctx.accounts.user.key()).unwrap();
    let (pda, _bump) = Pubkey::find_program_address(&[b"Main Account"], ctx.program_id);
    require!(amount <= ctx.accounts.main_account.vec_ammount[index], ErrorCode::AmountError);
    require!(pda == ctx.accounts.main_account_info.key(), ErrorCode::PubkeyError);
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    **ctx.accounts.main_account_info.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.to.to_account_info().try_borrow_mut_lamports()? += amount;
    main_account.vec_ammount[index] -= amount;
    main_account.total_ammount -= amount;
    Ok(())
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut, seeds = [b"Main Account"], bump = main_account.bump_original)]
    pub main_account: Account<'info, MainAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is account is the receiver
    #[account(mut)]
    pub to: AccountInfo<'info>,
    /// CHECK: This is the main_account
    #[account(mut)]
    pub main_account_info: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
