use anchor_lang::prelude::*;

#[account(zero_copy)]
pub struct DeployOffsets {
    data: [u8; 16],
}
