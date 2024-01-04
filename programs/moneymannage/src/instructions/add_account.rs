use crate::{
    state::accounts::*,
    utils::{ANCHOR_BUFFER, MAIN_ACCOUNT, NEW_SIZE},
};
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, pubkey::Pubkey, system_instruction::transfer},
};

pub fn add_account(ctx: Context<AddAccount>, amount: u64) -> Result<()> {
    let user_accont_info: AccountInfo = ctx.accounts.user.to_account_info();
    let lamports_user: u64 = AccountInfo::lamports(&user_accont_info);
    let user: Pubkey = ctx.accounts.user.key();
    let main_account_info: AccountInfo = ctx.accounts.main_account_info.to_account_info();
    let (pda, _bump) = Pubkey::find_program_address(&[MAIN_ACCOUNT], ctx.program_id);
    let mut key_exist: bool = false;
    for i in ctx.accounts.main_account.vec_keys.clone() {
        if i == ctx.accounts.user.key() {
            key_exist = true
        }
    }
    // validations
    require_gte!(lamports_user, amount);
    require_eq!(key_exist, false);
    // tx
    invoke(
        &transfer(&user, &pda.key(), amount),
        &[user_accont_info, main_account_info.clone()],
    )
    .expect("Error in tx");
    //update state
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    main_account.add_key(user);
    main_account.add_amount(amount);
    main_account.add_total_amount(amount);
    main_account.add_len();
    Ok(())
}

#[derive(Accounts)]
pub struct AddAccount<'info> {
    #[account(
        mut,
        seeds = [MAIN_ACCOUNT],
        bump = main_account.bump_original,
        realloc = ANCHOR_BUFFER + main_account.len as usize + NEW_SIZE as usize,
        realloc::payer = user,
        realloc::zero = false,
    )]
    pub main_account: Account<'info, MainAccount>,
    #[account(mut, signer)]
    /// CHECK: safe
    pub user: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: safe
    pub main_account_info: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
