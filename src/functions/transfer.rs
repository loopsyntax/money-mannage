use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        signature::Signature,
        pubkey::Pubkey,
    },
    Program,
};
use anyhow::Result;

pub fn transfer(
    program: &Program,
    amount: u64
) -> Result<()> {
    let (main_account, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[b"Main Account"], &program.id());
    let tx: Signature = program
        .request()
        .accounts(moneymannage::accounts::Transfer {
            main_account,
            user: program.payer(),
            to: program.payer(),
            main_account_info: main_account,
            system_program: system_program::ID,
        })
        .args(moneymannage::instruction::Transfer {
            amount
        })
        .send()?;
    println!("------------------------------------------------------------");
    println!("Tx: {}", tx);
    println!("------------------------------------------------------------");
    println!("Main Account: {}", main_account);
    println!("------------------------------------------------------------");
    Ok(())
}