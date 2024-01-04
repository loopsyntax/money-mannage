use crate::utils::NEW_SIZE;
use anchor_lang::prelude::*;

#[account]
pub struct MainAccount {
    pub bump_original: u8,     // 1
    pub len: u16,              // 2
    pub authority: Pubkey,     // 32
    pub vec_keys: Vec<Pubkey>, // 4 + 32 + 32
    pub vec_amount: Vec<u64>,  // 4 + 8 + 8
    pub total_amount: u64,     // 8
}

impl MainAccount {
    pub const SIZE: usize = 1 + 2 + 32 + 4 + 32 + 32 + 4 + 8 + 8 + 8;

    pub fn set_bump_original(&mut self, bump: u8) {
        self.bump_original = bump;
    }

    pub fn set_authority(&mut self, authority: Pubkey) {
        self.authority = authority;
    }

    pub fn add_key(&mut self, key: Pubkey) {
        self.vec_keys.push(key)
    }

    pub fn add_amount(&mut self, amount: u64) {
        self.vec_amount.push(amount);
    }

    pub fn add_total_amount(&mut self, amount: u64) {
        self.total_amount += amount;
    }

    pub fn sub_total_amount(&mut self, amount: u64) {
        self.total_amount -= amount;
    }

    pub fn add_len(&mut self) {
        self.len += NEW_SIZE;
    }

    pub fn set_len(&mut self, len: u16) {
        self.len = len;
    }

    pub fn sub_vec_amount(&mut self, index: usize, amount: u64) {
        self.vec_amount[index] -= amount;
    }
}
