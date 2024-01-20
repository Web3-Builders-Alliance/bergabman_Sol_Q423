use anchor_lang::{prelude::*, solana_program::entrypoint::MAX_PERMITTED_DATA_INCREASE};

use crate::state::{DevDeploy, DevDeployData, DevDeployOffsets, DevFund};

#[derive(Accounts)]
pub struct AccountSizeOffsets<'info> {
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
        seeds = [b"dev_deploy", dev_fund.key().as_ref(), dev.key().as_ref()],
        bump = dev_deploy.dev_deploy_bump
    )]
    pub dev_deploy: Account<'info, DevDeploy>,
    #[account(
        mut,
        seeds = [b"dev_deploy_offsets", dev_fund.key().as_ref(), dev.key().as_ref()],
        bump = dev_deploy.dev_deploy_offsets_bump
    )]
    pub dev_deploy_offsets: AccountLoader<'info, DevDeployOffsets>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AccountSizeData<'info> {
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
        seeds = [b"dev_deploy", dev_fund.key().as_ref(), dev.key().as_ref()],
        bump = dev_deploy.dev_deploy_bump
    )]
    pub dev_deploy: Account<'info, DevDeploy>,
    #[account(
        mut,
        seeds = [b"dev_deploy_data", dev_fund.key().as_ref(), dev.key().as_ref()],
        bump = dev_deploy.dev_deploy_data_bump
    )]
    pub dev_deploy_data: AccountLoader<'info, DevDeployData>,
    pub system_program: Program<'info, System>,
}

impl<'info> AccountSizeOffsets<'info> {
    pub fn size_increase(&mut self) -> Result<()> {
        let this_acc = self.dev_deploy_offsets.to_account_info();
        // let offsets = self.dev_deploy_offsets.to_account_info();
        let new_size = this_acc.try_borrow_data()?.len() + MAX_PERMITTED_DATA_INCREASE;
        // msg!("offsets {} ", offsets.key);
        let rent = Rent::get()?;
        let new_minimum_balance = rent.minimum_balance(new_size);

        let lamports_diff = new_minimum_balance.saturating_sub(this_acc.lamports());
        **self.dev_fund.to_account_info().try_borrow_mut_lamports()? -= lamports_diff;
        **this_acc
            .try_borrow_mut_lamports()? += lamports_diff;
        // **self
        //     .dev_deploy_offsets
        //     .to_account_info()
        //     .try_borrow_mut_lamports()? += lamports_diff;

        this_acc.realloc(new_size, false)?;
        msg!("offsets new size {}", this_acc.try_borrow_data()?.len());
        Ok(())
    }
}

impl<'info> AccountSizeData<'info> {
    pub fn size_increase(&mut self) -> Result<()> {
        let this_acc = self.dev_deploy_data.to_account_info();
        
        
        // let offsets = self.dev_deploy_offsets.to_account_info();
        let new_size = this_acc.try_borrow_data()?.len() + MAX_PERMITTED_DATA_INCREASE;
        // msg!("offsets {} ", offsets.key);
        let rent = Rent::get()?;
        let new_minimum_balance = rent.minimum_balance(new_size);

        let lamports_diff = new_minimum_balance.saturating_sub(this_acc.lamports());
        **self.dev_fund.to_account_info().try_borrow_mut_lamports()? -= lamports_diff;
        **this_acc
            .try_borrow_mut_lamports()? += lamports_diff;
        // **self
        //     .dev_deploy_offsets
        //     .to_account_info()
        //     .try_borrow_mut_lamports()? += lamports_diff;

        this_acc.realloc(new_size, false)?;
        msg!("data new size {}", this_acc.try_borrow_data()?.len());
        Ok(())
    }
}