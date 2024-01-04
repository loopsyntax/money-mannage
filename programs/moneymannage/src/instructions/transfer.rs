use crate::{state::accounts::*, utils::MAIN_ACCOUNT};
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
    let (pda, _bump) = Pubkey::find_program_address(&[MAIN_ACCOUNT], ctx.program_id);
    let main_account: Pubkey = ctx.accounts.main_account_info.key();
    let to: AccountInfo = ctx.accounts.to.to_account_info();
    let main_account_info: AccountInfo = ctx.accounts.main_account_info.to_account_info();
    let mut key_exist: bool = false;
    for i in &ctx.accounts.main_account.vec_keys {
        if i == &ctx.accounts.user.key() {
            key_exist = true
        }
    }
    require_eq!(key_exist, false);
    let index = ctx
        .accounts
        .main_account
        .vec_keys
        .iter()
        .position(|&x| x == ctx.accounts.user.key())
        .unwrap();
    let amount_in_pda: u64 = ctx.accounts.main_account.vec_amount[index];
    require_gte!(amount_in_pda, amount);
    require_keys_eq!(pda, main_account);
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    // transfer
    **main_account_info.try_borrow_mut_lamports()? -= amount;
    **to.try_borrow_mut_lamports()? += amount;
    // update state
    main_account.sub_vec_amount(index, amount);
    main_account.sub_total_amount(amount);
    Ok(())
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut, seeds = [MAIN_ACCOUNT], bump = main_account.bump_original)]
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
