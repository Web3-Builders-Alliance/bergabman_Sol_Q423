use std::ops::{Add, Div, Mul, Rem};

use anchor_lang::prelude::*;

#[account]
pub struct DevDeploy {
    pub bump: u8,
    pub ot_6_len: u32, // OffsetTable
    pub ot_5_len: u32,
    pub data_orig_len: u32,
    pub dev_fund: Pubkey,
    pub dev: Pubkey,
    pub data: Pubkey,
}

impl DevDeploy {
    // pub const LEN: usize = 8 + std::mem::size_of::<DevDeploy>();
    pub const INIT_LEN: usize = 8 + 1 + 4 + 4 + 4 + 32 + 32 + 32;
    pub fn init(
        &mut self,
        bump: u8,
        ot_6_len: u32,
        ot_5_len: u32,
        data_orig_len: u32,
    ) -> Result<()> {
        self.bump = bump;
        self.ot_6_len = ot_6_len;
        self.ot_5_len = ot_5_len;
        self.data_orig_len = data_orig_len;
        self.dev_fund = self.dev_fund.key();
        self.dev = self.dev.key();
        self.data = self.data.key();
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub struct U24(u8, u16);
impl U24 {
    pub const MAX: u32 = 16_711_425; //u8::MAX(255) * u16::MAX(65535)
    pub const LEN: u8 = 3;
}

impl From<(u8, u16)> for U24 {
    fn from(value: (u8, u16)) -> Self {
        Self(value.0, value.1)
    }
}

impl From<u32> for U24 {
    fn from(value: u32) -> Self {
        let whole_u16 = value.div(u16::MAX as u32);
        let remainder = value.rem(whole_u16 * u16::MAX as u32);
        Self(whole_u16 as u8, remainder as u16)
    }
}

impl From<U24> for u32 {
    fn from(value: U24) -> Self {
        let whole_u16 = (value.0 as u16).mul(u16::MAX) as u32;
        let result = whole_u16.add(value.1 as u32);
        result
    }
}
