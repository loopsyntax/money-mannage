<div align="center">

  ![money-mannage](money-mannage.gif)

  <h1 id="title">Money mannage</h1>
  <h4>A crypto money management program</h4>
</div>

---

We are in an era of decentralization where security and scalability are essential for the management of cryptographic assets. That's why we have created a decentralized program that runs on the Solana blockchain and aims to facilitate the management of users' funds.

Our program consists of three main functions that can be adapted to security needs and the implementation of a decentralized consensus-based architecture. The first function creates a main account through which API requests can be made to obtain account data. The second function allows for the addition of more accounts to the main account for management and monitoring by the owner. The last function allows for the transfer of SOL to any recipient.

In the future, blockchain software will play a fundamental role in managing large funds, and our proposal offers a suitable administrative solution for these cases. Therefore, we believe that the consensus of valid wallets or accounts will be a more appropriate architecture for managing funds and accounts of greater volume.

In summary, our decentralized program offers a secure and scalable solution for managing cryptographic assets in Solana, and we believe it will be a key tool in the future of decentralized finance.

<h3 align="center">Create a main account<h3>

```rust
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
    /// CHECK: This is the signer
    #[account(mut, signer)]
    pub user: AccountInfo<'info>,
    /// CHECK: This is main_account
    #[account(mut)]
    pub main_account_info: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
```

<h3 align="center">Add an account</h3>

```rust
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
    /// CHECK: This is the signer
    #[account(mut, signer)]
    pub user: AccountInfo<'info>,
    /// CHECK: This is the main_account
    #[account(mut)]
    pub main_account_info: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
```

<h3 align="center">Transfer SOL to an account</h3>

```rust
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

```
