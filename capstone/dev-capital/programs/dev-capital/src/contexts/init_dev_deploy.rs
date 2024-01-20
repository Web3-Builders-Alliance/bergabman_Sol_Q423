use anchor_lang::{
    prelude::*,
    solana_program::{
        entrypoint::MAX_PERMITTED_DATA_INCREASE, program::invoke_signed, system_instruction,
        system_program,
    },
    system_program::{transfer, CreateAccount, Transfer},
    Bumps,
};

use crate::state::{DevDeploy, DevDeployData, DevDeployOffsets, DevFund, U24};

#[derive(Accounts)]
// #[instruction(seed: u64)]
pub struct InitDevDeploy<'info> {
    #[account(mut)]
    pub dev: Signer<'info>,
    #[account(
        mut,
        seeds = [b"dev_fund", dev_fund.funder.as_ref(), dev.key().as_ref()],
        bump = dev_fund.bump
    )]
    pub dev_fund: Account<'info, DevFund>,
    #[account(
        init,
        payer = dev,
        space = DevDeploy::INIT_LEN,
        seeds = [b"dev_deploy", dev_fund.key().as_ref(), dev.key().as_ref()],
        bump
    )]
    pub dev_deploy: Account<'info, DevDeploy>,
    /// CHECK validatio is done inside contract, based on initial setup
    #[account(
        init,
        payer = dev,
        space = 8 ,
        seeds = [b"dev_deploy_offsets", dev_fund.key().as_ref(), dev.key().as_ref()],
        bump
    )]
    pub dev_deploy_offsets: AccountLoader<'info, DevDeployOffsets>,
    #[account(
        init,
        payer = dev,
        space = 8,
        seeds = [b"dev_deploy_data", dev_fund.key().as_ref(), dev.key().as_ref()],
        bump
    )]
    pub dev_deploy_data: AccountLoader<'info, DevDeployData>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitDevDeploy<'info> {
    pub fn init_dev_deploy(
        &mut self,
        _program_id: &Pubkey,
        bumps: InitDevDeployBumps,
        ot_6_len: u32,
        ot_5_len: u32,
        orig_len: u32,
    ) -> Result<()> {
        // require_eq!(self.dev_deploy_offsets.executable, false);
        // require_eq!(self.dev_deploy_offsets.is_writable, true);
        // require_eq!(self.dev_deploy_offsets.lamports(), 0);
        // require_eq!(self.dev_deploy_offsets.data_is_empty(), true);
        // require_eq!(self.dev_deploy_data.executable, false);
        // require_eq!(self.dev_deploy_data.is_writable, true);
        // require_eq!(self.dev_deploy_data.lamports(), 0);
        // require_eq!(self.dev_deploy_data.data_is_empty(), true);

        self.dev_deploy.init(ot_6_len, ot_5_len, orig_len, bumps)?;

        // let dev_fund_seeds = [
        //     b"dev_fund",
        //     self.dev_fund.funder.as_ref(),
        //     self.dev.to_account_info().key.as_ref(),
        //     &[self.dev_fund.bump],
        // ];



        Ok(())
    }
}
