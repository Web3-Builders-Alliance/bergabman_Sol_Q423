use anchor_lang::prelude::*;

#[account(zero_copy)]
pub struct DevDeployData {
    data: [u8; 16],
}
