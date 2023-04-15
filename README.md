<div align="center">

  ![money-mannage](money-mannage.gif)

  <h1 id="title">💷Money mannage📊</h1>
  <h4>A crypto money management program🛡️</h4>
</div>

---

We are in an era of decentralization where security and scalability are essential for the management of cryptographic assets. That's why we have created a decentralized program that runs on the Solana blockchain and aims to facilitate the management of users' funds.

Our program consists of three main functions that can be adapted to security needs and the implementation of a decentralized consensus-based architecture. The first function creates a main account through which API requests can be made to obtain account data. The second function allows for the addition of more accounts to the main account for management and monitoring by the owner. The last function allows for the transfer of SOL to any recipient.

In the future, blockchain software will play a fundamental role in managing large funds, and our proposal offers a suitable administrative solution for these cases. Therefore, we believe that the consensus of valid wallets or accounts will be a more appropriate architecture for managing funds and accounts of greater volume.

In summary, our decentralized program offers a secure and scalable solution for managing cryptographic assets in Solana, and we believe it will be a key tool in the future of decentralized finance.

<h3 align="center">Create a main account📖</h3>

```rust
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
```

This function create_main_account creates an account of type MainAccount using a user account that is specified in the context (ctx). The function expects two arguments: ctx of type Context<Create> and amount of type u64. The function returns a Result<()> object indicating whether the account creation operation was successful or not.

The create_main_account function performs several operations to create the MainAccount. First, it checks that the amount is less than or equal to the number of lamports in the user account ctx.accounts.user. If the amount exceeds the number of lamports in the user account, the function returns an ErrorCode::AmountError. If the amount is less than or equal, continue with account creation.

The function uses the ctx.accounts.main_account object to create the MainAccount. This account is initialized using the macro #[account(init, seeds = [b"Main Account"], bump, payer = user, space = MainAccount::SIZE + 8)]. The account is initialized with the "Main Account" seed, which is used in conjunction with the program identifier to generate the MainAccount address. The MainAccount account also requires a storage space of size MainAccount::SIZE + 8. Payment for account creation is made using the user account ctx.accounts.user.

The function then uses the Pubkey::find_program_address function to generate a derived program address (PDA) and bump value. The PDA is generated using the "Main Account" seed and the program identifier. The offset value is a random value used to ensure that the PDA account address is unique.

The function then uses the program's invoke function to perform a transfer of lamports from the user account to the PDA account. The invoke function calls the system_instruction::transfer function to perform the transfer. The transfer is made using the user account and the MainAccount.

After the transfer, the function updates the MainAccount account data. The bump_original variable is set to the previously generated offset value. The len variable is set to a constant value indicating the size of the MainAccount structure. The authority variable is set to the public key of the user account. The function also adds the public key of the user account to vec_keys and the transferred amount to vec_ammount. The total_ammount variable is updated with the amount transferred.

Finally, the function returns a Result<()> object indicating whether the account creation operation was successful or not.

<h3 align="center">Add an account🔐</h3>

```rust
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
```

In general terms, this function is responsible for adding an account to a "Main Account" and transferring funds to said newly created account.

The function takes two arguments: the ctx context and the amount of funds to transfer amount. The context contains information about the account of the user calling the function, the program account, the account being created (in this case, the user account), and the account that is storing the created accounts (in this case , the account main_account).

In terms of functionality, the function performs the following steps:

- Checks if the public key of the user's account already exists in the key vector stored in main_account.
- Verify that the amount of funds to be transferred does not exceed the funds available in the user's account.
- Verifies that the public key of the user's account does not exist in main_account.
- Transfers the specified funds from the user's account to the program (represented by the public address pda), using the system_instruction::transfer function.
- Adds the public key of the newly created user's account and the amount transferred to the vector of keys and amounts stored in main_account.
- Updates the total funds in main_account.
- Returns a Result<()> object indicating whether the operation completed successfully or not.

<h3 align="center">Transfer SOL to an account</h3>

```rust
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
```
This "transfer" function is used to transfer funds from one main account ("main_account") to another specified account ("to"), while verifying that the user initiating the transfer is a valid owner of the main account.

The function takes the following parameters:

- ctx – The context of the transaction, which includes information about the accounts involved and other relevant data.
- amount: The amount of funds to be transferred.
The function first checks if the owner of the main account is authorized to make this transfer by checking if their public key is in the list of public keys associated with the main account. If no match is found, a "PubkeyError" is thrown.

The function then verifies that the amount of funds to be transferred is less than or equal to the current balance of the main account. If the amount is greater, an "AmountError" error is thrown.

The function then verifies that the main account is the correct account, using a derived program address that is generated from a seed (the string "Main Account" in this case).

Finally, the function updates the balances of the accounts involved and the information of the main account. The amount of funds transferred is deducted from the main account balance and added to the balance of the specified destination ("to") account. The total amount of funds in the main account is also updated.
