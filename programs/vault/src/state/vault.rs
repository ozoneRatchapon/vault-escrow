use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // InitSpace is required for PDA derivation
pub struct VaultState {
    // Vault state account
    pub vault_system_account_bump: u8, // system program beccuse we creating a new account and initially every account it owned by the system program
    pub vault_state_bump: u8,          // bump for vault state account
}
