use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct EscrowState {
    // this is the escrow account that holds the token A
    pub seed: u64,           // seeds used to derive the escrow PDA ex: b"escrow"
    pub maker: Pubkey,       // whoever initialized this escrow
    pub mint_a: Pubkey,      // exchanged token A
    pub mint_b: Pubkey,      // exchanged token B
    pub receive_amount: u64, // how much token A to receive
    pub bump: u8,            // bump used to derive the escrow PDA
}
