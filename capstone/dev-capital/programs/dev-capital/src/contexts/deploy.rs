use anchor_lang::{
    prelude::*,
    solana_program::{
        entrypoint::MAX_PERMITTED_DATA_INCREASE, program::invoke_signed, system_instruction,
        system_program,
    },
    system_program::{transfer, CreateAccount, Transfer},
    Bumps,
};

use crate::state::{DevConfig, DeployData, DeployOffsets, DevFund, U24};

#[derive(Accounts)]
// #[instruction(seed: u64)]
pub struct Deploy<'info> {
    #[account(mut)]
    pub dev: Signer<'info>,
    #[account(
        mut,
        seeds = [b"dev_fund", dev_fund.funder.as_ref(), dev.key().as_ref()],
        bump = dev_fund.bump
    )]
    pub dev_fund: Account<'info, DevFund>,
    #[account(
        mut,
        seeds = [b"dev_config", dev_fund.key().as_ref(), dev.key().as_ref()],
        bump = dev_config.dev_config_bump,
    )]
    pub dev_config: Account<'info, DevConfig>,
    #[account(
        mut,
        seeds = [b"deploy_offsets", dev_fund.key().as_ref(), dev.key().as_ref()],
        bump = dev_config.deploy_offsets_bump,
    )]
    pub deploy_offsets: AccountLoader<'info, DeployOffsets>,
    #[account(
        mut,
        seeds = [b"deploy_data", dev_fund.key().as_ref(), dev.key().as_ref()],
        bump = dev_config.deploy_data_bump,
    )]
    pub deploy_data: AccountLoader<'info, DeployData>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deploy<'info> {
    pub fn deploy_offsets(
        &mut self,
        data: &[u8],
    ) -> Result<()> {

        // self.dev_deploy.init(ot_6_len, ot_5_len, orig_len, bumps)?;

        // let dev_fund_seeds = [
        //     b"dev_fund",
        //     self.dev_fund.funder.as_ref(),
        //     self.dev.to_account_info().key.as_ref(),
        //     &[self.dev_fund.bump],
        // ];

        Ok(())
    }
}
