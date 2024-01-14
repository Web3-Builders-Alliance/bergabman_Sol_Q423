use anchor_lang::prelude::*;

#[account]
pub struct DevFund {
    pub funder: Pubkey,
    pub dev: Pubkey,
    pub bump: u8,
}

impl DevFund {
    // pub const LEN: usize = 8+std::mem::size_of::<DevFund>();
    pub const INIT_LEN: usize = 8 + 32 + 32 + 1;

    pub fn init(&mut self, funder: Pubkey, dev: Pubkey, bump: u8) -> Result<()> {
        self.funder = funder;
        self.dev = dev;
        self.bump = bump;

        Ok(())
    }
}
