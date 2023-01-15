use anyhow::Result;
pub mod functions;

fn main() -> Result<()> {
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]//cargo test create_main_account -- --show-output
    fn create_main_account() {
        use anchor_client::{
            solana_sdk::{pubkey::Pubkey, signature::read_keypair_file},
            Client, Cluster,
        };
        use std::rc::Rc;
        use std::str::FromStr;
        let program = Client::new(
            Cluster::Devnet,
            Rc::new(
                read_keypair_file(&*shellexpand::tilde(
                    "C:/Users/Mateo/.config/solana/id.json",
                ))
                .expect("Example requires a keypair file"),
            ),
        )
        .program(Pubkey::from_str("9s8JNZx3e3LbUaxqAd6ENJVXCzjRgC1oiezTNd6cXxLS").unwrap());
        functions::create_main_account::create_main_account(
            &program,
            5000006
        )
        .unwrap();
    }
    #[test]//cargo test add_account -- --show-output
    fn add_account() {
        use anchor_client::{
            solana_sdk::{pubkey::Pubkey, signature::read_keypair_file},
            Client, Cluster,
        };
        use std::rc::Rc;
        use std::str::FromStr;
        let program = Client::new(
            Cluster::Devnet,
            Rc::new(
                read_keypair_file(&*shellexpand::tilde(
                    "C:/Users/Mateo/.config/solana/id.json",
                ))
                .expect("Example requires a keypair file"),
            ),
        )
        .program(Pubkey::from_str("9s8JNZx3e3LbUaxqAd6ENJVXCzjRgC1oiezTNd6cXxLS").unwrap());
        functions::add_account::add_account(
            &program,
            2072004
        )
        .unwrap();
    }
    #[test]//cargo test transfer -- --show-output
    fn transfer() {
        use anchor_client::{
            solana_sdk::{pubkey::Pubkey, signature::read_keypair_file},
            Client, Cluster,
        };
        use std::rc::Rc;
        use std::str::FromStr;
        let program = Client::new(
            Cluster::Devnet,
            Rc::new(
                read_keypair_file(&*shellexpand::tilde(
                    "C:/Users/Mateo/.config/solana/id.json",
                ))
                .expect("Example requires a keypair file"),
            ),
        )
        .program(Pubkey::from_str("9s8JNZx3e3LbUaxqAd6ENJVXCzjRgC1oiezTNd6cXxLS").unwrap());
        functions::transfer::transfer(
            &program,
            2072004
        )
        .unwrap();
    }
}