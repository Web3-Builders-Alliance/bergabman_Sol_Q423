use anchor_lang::prelude::*;

#[account]
pub struct DevDeployData {
    pub bump: u8,
    pub ot_6_len: u32, // OffsetTable
    pub ot_5_len: u32,
    pub data_orig_len: u32,
    pub dev_fund: Pubkey,
    pub dev: Pubkey,
    pub data: Pubkey,
}
