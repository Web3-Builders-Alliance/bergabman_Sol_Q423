use anchor_lang::prelude::*;

#[account(zero_copy)]
pub struct DevDeployOffsets {
    data: [u8; 16],
}
