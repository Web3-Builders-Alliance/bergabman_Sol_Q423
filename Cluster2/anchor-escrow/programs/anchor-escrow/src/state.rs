use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub seed: u64,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
    pub bump: u8,
}

impl Escrow {
    // pub const LEN: usize = std::mem::size_of::<Escrow>();
    pub const LEN: usize = std::mem::size_of::<u64>()
        + std::mem::size_of::<Pubkey>()
        + std::mem::size_of::<Pubkey>()
        + std::mem::size_of::<u64>()
        + std::mem::size_of::<u8>();
}
