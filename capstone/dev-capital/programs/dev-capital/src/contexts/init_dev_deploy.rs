use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::state::{DevDeploy, DevDeployData, DevFund};

#[derive(Accounts)]
// #[instruction(seed: u64)]
pub struct InitDevDeploy<'info> {
    #[account(mut)]
    pub dev: Signer<'info>,
    #[account(
        seeds = [b"dev_fund", dev_fund.funder.as_ref(), dev.key().as_ref()],
        bump = dev_fund.bump
    )]
    pub dev_fund: Account<'info, DevFund>,
    #[account(
        mut,
        seeds = [b"dev_deploy", dev_fund.key().as_ref(), dev.key().as_ref()],
        bump
    )]
    pub dev_deploy: Account<'info, DevDeploy>,
    #[account(
        init,
        payer = dev,
        seeds = [b"dev_deploy_data", dev_fund.key().as_ref(), dev.key().as_ref()],
        space = 1024,
        bump
    )]
    pub dev_deploy_data: Account<'info, DevDeployData>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitDevDeploy<'info> {
    pub fn init_dev_deploy(
        &mut self,
        bump: u8,
        ot_6_len: u32,
        ot_5_len: u32,
        orig_len: u32,
    ) -> Result<()> {
        self.dev_deploy.init(bump, ot_6_len, ot_5_len, orig_len)?;
        let cpi_accounts = Transfer {
            from: self.dev_fund.to_account_info(),
            to: self.dev_deploy_data.to_account_info(),
        };

        Ok(())
    }
}
