<div align="center">

  ![money-mannage](money-mannage.gif)

  # Money mannage

  A crypto money management program🛡

</div>

---

We are in an era of decentralization where security and scalability are essential for the management of cryptographic assets. That's why we have created a decentralized program that runs on the Solana blockchain and aims to facilitate the management of users' funds.

Our program consists of three main functions that can be adapted to security needs and the implementation of a decentralized consensus-based architecture. The first function creates a main account through which API requests can be made to obtain account data. The second function allows for the addition of more accounts to the main account for management and monitoring by the owner. The last function allows for the transfer of SOL to any recipient.

In the future, blockchain software will play a fundamental role in managing large funds, and our proposal offers a suitable administrative solution for these cases. Therefore, we believe that the consensus of valid wallets or accounts will be a more appropriate architecture for managing funds and accounts of greater volume.

In summary, our decentralized program offers a secure and scalable solution for managing cryptographic assets in Solana, and we believe it will be a key tool in the future of decentralized finance.

## Create a main account

```rust

```

The create_main_account function performs several operations to create the MainAccount. First, it checks that the amount is less than or equal to the number of lamports in the user account ctx.accounts.user. If the amount exceeds the number of lamports in the user account, the function returns an ErrorCode::AmountError. If the amount is less than or equal, continue with account creation.

The function uses the ctx.accounts.main_account object to create the MainAccount. This account is initialized using the macro #[account(init, seeds = [b"Main Account"], bump, payer = user, space = MainAccount::SIZE + 8)]. The account is initialized with the "Main Account" seed, which is used in conjunction with the program identifier to generate the MainAccount address. The MainAccount account also requires a storage space of size MainAccount::SIZE + 8. Payment for account creation is made using the user account ctx.accounts.user.

The function then uses the Pubkey::find_program_address function to generate a derived program address (PDA) and bump value. The PDA is generated using the "Main Account" seed and the program identifier. The offset value is a random value used to ensure that the PDA account address is unique.

The function then uses the program's invoke function to perform a transfer of lamports from the user account to the PDA account. The invoke function calls the system_instruction::transfer function to perform the transfer. The transfer is made using the user account and the MainAccount.


## Add an account

```rust

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

## Transfer SOL to an account

```rust

```
This transfer function is used to transfer funds from one main account ("main_account") to another specified account ("to"), while verifying that the user initiating the transfer is a valid owner of the main account.The function takes the following parameters:

- ctx – The context of the transaction, which includes information about the accounts involved and other relevant data.
- amount: The amount of funds to be transferred.
The function first checks if the owner of the main account is authorized to make this transfer by checking if their public key is in the list of public keys associated with the main account. If no match is found, a "PubkeyError" is thrown.

The function then verifies that the amount of funds to be transferred is less than or equal to the current balance of the main account. If the amount is greater, an "AmountError" error is thrown.

The function updates the balances of the accounts involved and the information of the main account. The amount of funds transferred is deducted from the main account balance and added to the balance of the specified destination ("to") account. The total amount of funds in the main account is also updated.
