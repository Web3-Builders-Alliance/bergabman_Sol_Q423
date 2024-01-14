use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

use crate::state::dev_fund::DevFund;

#[derive(Accounts)]
pub struct InitDevFund<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,
    pub dev: SystemAccount<'info>,
    #[account(
        init,
        payer = funder,
        space = DevFund::INIT_LEN,
        seeds = [b"dev_fund", funder.key().as_ref(), dev.key.as_ref()],
        bump
    )]
    pub dev_fund: Account<'info, DevFund>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitDevFund<'info> {
    pub fn init_dev_fund(&mut self, bump: u8, lamports: u64) -> Result<()> {
        self.dev_fund
            .init(self.funder.key(), self.dev.key(), bump)?;

        
        let cpi_accounts = Transfer {
            from: self.funder.to_account_info(),
            to: self.dev_fund.to_account_info(),
        };

        let ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);

        transfer(ctx, lamports)?;
        Ok(())
    }
}
