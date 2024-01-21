use anchor_lang::prelude::*;

#[account(zero_copy)]
pub struct DeployData {
    data: [u8; 16],
}
