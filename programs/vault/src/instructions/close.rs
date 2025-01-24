use crate::state::VaultState;
use anchor_lang::{
    prelude::*,
    system_program::System,
};

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"state", signer.key().as_ref()],
        bump = vault_state.vault_state_bump,
        close = vault_state
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [vault_state.key().as_ref()],
        bump = vault_state.vault_system_account_bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        assert!(
            self.vault.lamports() <= Rent::get()?.minimum_balance(8) && self.vault.lamports() > 0
        );
        self.vault_state.close(self.vault.to_account_info())?;
        Ok(())
    }
}