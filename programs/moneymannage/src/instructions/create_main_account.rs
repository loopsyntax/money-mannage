use crate::{
    state::accounts::*,
    utils::{ANCHOR_BUFFER, MAIN_ACCOUNT},
};
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, pubkey::Pubkey, system_instruction::transfer},
};

pub fn create_main_account(ctx: Context<Create>, amount: u64) -> Result<()> {
    let user_pubkey: Pubkey = ctx.accounts.user.key();
    let user_accont_info: AccountInfo = ctx.accounts.user.to_account_info();
    let main_account_info: AccountInfo = ctx.accounts.main_account_info.to_account_info();
    let lamports_user: u64 = AccountInfo::lamports(&user_accont_info);
    require_gte!(lamports_user, amount);
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    let (pda, bump) = Pubkey::find_program_address(&[MAIN_ACCOUNT], ctx.program_id);
    invoke(
        &transfer(&user_pubkey, &pda.key(), amount),
        &[user_accont_info, main_account_info.clone()],
    )
    .expect("Error");
    main_account.set_bump_original(bump);
    main_account.set_len(MainAccount::SIZE as u16);
    main_account.set_authority(user_pubkey);
    main_account.add_key(user_pubkey);
    main_account.add_amount(amount);
    main_account.add_total_amount(amount);
    Ok(())
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, seeds = [MAIN_ACCOUNT], bump, payer = user, space = MainAccount::SIZE + ANCHOR_BUFFER)]
    pub main_account: Account<'info, MainAccount>,
    #[account(mut, signer)]
    /// CHECK: safe
    pub user: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: safe
    pub main_account_info: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
