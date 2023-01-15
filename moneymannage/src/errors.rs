use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Your proposal is not superior to the existing one")]AmountError,
    #[msg("The pubkey already exists")]PubkeyError,
}