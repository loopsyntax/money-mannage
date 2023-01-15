use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
    solana_program::system_instruction,
}; 
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn create_main_account(
    ctx: Context<Create>,
    amount: u64
) -> Result<()> {
    require!(amount <= AccountInfo::lamports(&ctx.accounts.user.to_account_info()), ErrorCode::AmountError);
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    let (pda, bump) = Pubkey::find_program_address(&[b"Main Account"], ctx.program_id);
    anchor_lang::solana_program::program::invoke(
        &system_instruction::transfer(&ctx.accounts.user.key(), &pda.key(), amount),
        &[ctx.accounts.user.to_account_info(), ctx.accounts.main_account_info.to_account_info().clone()],
    ).expect("Error");
    main_account.bump_original = bump;
    main_account.len = 1 + 2 + 32 + 4 + 32 + 32 + 4 + 8 + 8 + 8;
    main_account.authority = ctx.accounts.user.key();
    main_account.vec_keys.push(ctx.accounts.user.key());
    main_account.vec_ammount.push(amount);
    main_account.total_ammount += amount;
    Ok(())
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, seeds = [b"Main Account"], bump, payer = user, space = MainAccount::SIZE + 8)]
    pub main_account: Account<'info, MainAccount>,
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub user: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub main_account_info: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}