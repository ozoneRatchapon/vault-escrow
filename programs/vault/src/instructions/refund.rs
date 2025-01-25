use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token::{close_account, CloseAccount}, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}
};

use crate::state::EscrowState;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker
    )]
    pub maker_mint_a_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"escrow", escrow.maker.to_bytes().as_ref(),escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, EscrowState>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Refund<'info> {

    pub fn withdraw(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            to: self.maker_mint_a_ata.to_account_info(),
            mint: self.mint_a.to_account_info(),
            from: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

        Ok(())
    }
    pub fn close(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = CloseAccount {
            authority: self.escrow.to_account_info(),
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
        };

        let seed_binding = self.escrow.seed.to_le_bytes();
        let maker_binding = self.escrow.maker.to_bytes();
        let bump_binding = self.escrow.bump;

        let seeds: [&[u8]; 4] = [b"escrow", &seed_binding, &maker_binding, &[bump_binding]];
        let signer_seeds: &[&[&[u8]]] = &[&seeds];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

        close_account(cpi_ctx)?;
        
        Ok(())
    }
}
