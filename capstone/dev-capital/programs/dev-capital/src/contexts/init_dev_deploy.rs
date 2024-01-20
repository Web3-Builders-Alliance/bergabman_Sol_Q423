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

        /////// increase size
        // let offsets = ctx.accounts.dev_deploy_offsets.load_mut()?;
        let offsets = self.dev_deploy_offsets.to_account_info();

        // let dev_fund_seeds = [
        //     b"dev_fund",
        //     self.dev_fund.funder.as_ref(),
        //     self.dev.to_account_info().key.as_ref(),
        //     &[self.dev_fund.bump],
        // ];

        // let new_size = offsets.try_borrow_data()?.len() + MAX_PERMITTED_DATA_INCREASE;
        // msg!("offsets {} ", offsets.key);
        // let rent = Rent::get()?;
        // let new_minimum_balance = rent.minimum_balance(new_size);

        // let lamports_diff = new_minimum_balance.saturating_sub(offsets.lamports());
        // **self.dev_fund.to_account_info().try_borrow_mut_lamports()? -= lamports_diff;
        // **self.dev_deploy_offsets.to_account_info().try_borrow_mut_lamports()? += lamports_diff;
        // // for _ in 0..=10 {
        // //     // invoke_signed(
        // //     //     &system_instruction::transfer(&self.dev_fund.key(), &self.dev_deploy_offsets.key(), lamports_diff),
        // //     //     &[
        // //     //         self.dev_fund.to_account_info(),
        // //     //         self.dev_deploy_offsets.to_account_info(),
        // //     //         self.system_program.to_account_info(),
        // //     //     ],
        // //     //     &[&dev_fund_seeds]
        // //     // )?;
        // // }
        // offsets.realloc(new_size, false)?;
        // msg!("account increased {}", offsets.try_borrow_data()?.len());

        // let rent_exempt = rent.minimum_balance(((ot_6_len + ot_5_len) * U24::LEN as u32) as usize);

        // let cpi_accounts = Transfer {
        //     from: self.dev_fund.to_account_info(),
        //     to: self.dev_deploy_offsets.to_account_info(),
        // };
        // let seeds_lock = &[dev_fund_seeds][..];
        // let binding = [dev_fund_seeds];
        // let binding = [&dev_fund_seeds];
        // let ctx = CpiContext::new_with_signer(self.system_program.to_account_info(), cpi_accounts, &binding);
        // transfer(ctx, rent_exempt)?;
        // for i in 0..10 {

        //     offsets.realloc(32, false)?;
        //     // msg!("data len {} {}", i, &offsets.data.borrow().len());
        // }
        // let data = offsets.try_borrow_mut_data()?;
        // msg!("data len {}", &data.len());

        // let dev_fund_seeds = &[
        //     b"dev_fund",
        //     self.dev_fund.funder.as_ref(),
        //     self.dev.to_account_info().key.as_ref(),
        //     &[self.dev_fund.bump],
        // ];

        // let dev_fund_signer_seeds = &[&dev_fund_seeds[..]];

        // let cpi_accounts_offsets = CreateAccount {
        //     from: self.dev_fund.to_account_info(),
        //     to: self.dev_deploy_offsets.to_account_info(),
        // };

        // let cpi_context_offsets = CpiContext::new(
        //     self.system_program.to_account_info(),
        //     cpi_accounts_offsets,
        //     // dev_fund_signer_seeds,
        // );

        // let rent_exempt_fee =
        //     Rent::get()?.minimum_balance(((ot_6_len + ot_5_len) * U24::LEN as u32) as usize);

        // anchor_lang::system_program::create_account(
        //     cpi_context_offsets.with_signer(dev_fund_signer_seeds),
        //     rent_exempt_fee,
        //     orig_len as u64,
        //     program_id,
        // )?;

        Ok(())
    }
}
