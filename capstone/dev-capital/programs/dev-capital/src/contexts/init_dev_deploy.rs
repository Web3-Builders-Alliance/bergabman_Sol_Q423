use anchor_lang::{
    prelude::*,
    solana_program::{system_instruction, system_program},
    system_program::{transfer, CreateAccount, Transfer},
};

use crate::state::{DevDeploy, DevFund, U24};

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
        init,
        payer = dev,
        space = DevDeploy::INIT_LEN,
        seeds = [b"dev_deploy", dev_fund.key().as_ref(), dev.key().as_ref()],
        bump
    )]
    pub dev_deploy: Account<'info, DevDeploy>,
    /// CHECK validatio is done inside contract, based on initial setup
    #[account(mut)]
    pub dev_deploy_offsets: UncheckedAccount<'info>,
    /// CHECK validatio is done inside contract, based on initial setup
    #[account(mut)]
    pub dev_deploy_data: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitDevDeploy<'info> {
    pub fn init_dev_deploy(
        &mut self,
        program_id: &Pubkey,
        bump: u8,
        ot_6_len: u32,
        ot_5_len: u32,
        orig_len: u32,
    ) -> Result<()> {
        require_eq!(self.dev_deploy_offsets.executable, false);
        require_eq!(self.dev_deploy_offsets.is_writable, true);
        require_eq!(self.dev_deploy_offsets.lamports(), 0);
        require_eq!(self.dev_deploy_offsets.data_is_empty(), true);
        require_eq!(self.dev_deploy_data.executable, false);
        require_eq!(self.dev_deploy_data.is_writable, true);
        require_eq!(self.dev_deploy_data.lamports(), 0);
        require_eq!(self.dev_deploy_data.data_is_empty(), true);

        self.dev_deploy.init(ot_6_len, ot_5_len, orig_len, bump)?;

        let dev_fund_seeds = &[
            b"dev_fund",
            self.dev_fund.funder.as_ref(),
            self.dev.to_account_info().key.as_ref(),
            &[self.dev_fund.bump],
        ];

        let dev_fund_signer_seeds = &[&dev_fund_seeds[..]];

        let cpi_accounts_offsets = CreateAccount {
            from: self.dev_fund.to_account_info(),
            to: self.dev_deploy_offsets.to_account_info(),
        };

        let cpi_context_offsets = CpiContext::new(
            self.system_program.to_account_info(),
            cpi_accounts_offsets,
            // dev_fund_signer_seeds,
        );

        let rent_exempt_fee =
            Rent::get()?.minimum_balance(((ot_6_len + ot_5_len) * U24::LEN as u32) as usize);

        anchor_lang::system_program::create_account(
            cpi_context_offsets.with_signer(dev_fund_signer_seeds),
            rent_exempt_fee,
            orig_len as u64,
            program_id,
        )?;

        Ok(())
    }
}
