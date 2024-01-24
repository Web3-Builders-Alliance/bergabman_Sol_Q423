use anchor_lang::prelude::*;

pub mod contexts;
// use contexts::*;
pub mod state;
use crate::contexts::*;
use crate::state::dev_fund::DevFund;

declare_id!("5MHA6ForrBUbPbom2x231cNsMCQvE4VCpQ7F6rKMt8bS");

#[program]
pub mod dev_capital {

    use super::*;

    pub fn init_dev_fund(ctx: Context<InitDevFund>, lamports: u64) -> Result<()> {
        // initializes funding for specific dev pubkey, with amount in lamports
        // tx for deployment will only be accepted if signed by dev key set in init step

        ctx.accounts.init_dev_fund(ctx.bumps.dev_fund, lamports)?;
        Ok(())
    }
    pub fn init_dev_config(
        ctx: Context<InitDevConfig>,
        ot_6_len: u32,
        ot_5_len: u32,
        orig_len: u32,
    ) -> Result<()> {
        // initializes deploy pda and data account
        // load dynamic sized accounts data
        // let offsets = ctx.accounts.dev_deploy_offsets.acc;

        ctx.accounts
            .init_dev_config(ctx.program_id, ctx.bumps, ot_6_len, ot_5_len, orig_len)?;

        Ok(())
    }

    pub fn account_size_offsets(ctx: Context<AccountSizeOffsets>) -> Result<()> {
        ctx.accounts.size_increase()?;
        Ok(())
    }

    pub fn account_size_data(ctx: Context<AccountSizeData>) -> Result<()> {
        ctx.accounts.size_increase()?;
        Ok(())
    }

    pub fn deploy_offsets(ctx: Context<Deploy>, data: Vec<u8>) -> Result<()> {
        ctx.accounts.deploy_offsets(&data)?;
        Ok(())
    }

    // pub fn deploy_data(ctx: Context<Deploy>, data: Vec<u8>) -> Result<()> {
    //     ctx.accounts.deploy_data(&data)?;
    //     Ok(())
    // }

    // pub fn dev_deploy(ctx: Context<DevDeploy>) -> Result<()> {
    //     Ok(())
    // }
}
