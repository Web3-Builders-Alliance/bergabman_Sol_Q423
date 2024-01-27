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
pub struct InitDevConfig<'info> {
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
        space = DevConfig::INIT_LEN,
        seeds = [b"dev_config", dev_fund.key().as_ref(), dev.key().as_ref()],
        bump
    )]
    pub dev_config: Account<'info, DevConfig>,
    #[account(
        init,
        payer = dev,
        space = 8 + 16,
        seeds = [b"deploy_offsets", dev_fund.key().as_ref(), dev.key().as_ref()],
        bump
    )]
    pub deploy_offsets: AccountLoader<'info, DeployOffsets>,
    #[account(
        init,
        payer = dev,
        space = 8 + 16,
        seeds = [b"deploy_data", dev_fund.key().as_ref(), dev.key().as_ref()],
        bump
    )]
    pub deploy_data: AccountLoader<'info, DeployData>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitDevConfig<'info> {
    pub fn init_dev_config(
        &mut self,
        _program_id: &Pubkey,
        bumps: InitDevConfigBumps,
        ot_6_len: u32,
        ot_5_len: u32,
        comp_len: u32,
        orig_len: u32,
    ) -> Result<()> {

        self.dev_config.init(ot_6_len, ot_5_len, comp_len, orig_len, bumps)?;
        self.deploy_offsets.load_init()?;
        self.deploy_data.load_init()?;

        // let dev_fund_seeds = [
        //     b"dev_fund",
        //     self.dev_fund.funder.as_ref(),
        //     self.dev.to_account_info().key.as_ref(),
        //     &[self.dev_fund.bump],
        // ];

        Ok(())
    }
}
