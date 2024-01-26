use std::{borrow::BorrowMut, cell::{Ref, RefMut}, ops::DerefMut};

use anchor_lang::{
    prelude::*,
    solana_program::{
        entrypoint::MAX_PERMITTED_DATA_INCREASE, program::invoke_signed, system_instruction,
        system_program,
    },
    system_program::{transfer, CreateAccount, Transfer},
    Bumps,
};
use arrayref::array_ref;
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
    // pub system_program: Program<'info, System>,
}

impl<'info> Deploy<'info> {
    pub fn deploy_offsets(
        &mut self,
        incoming: &[u8],
    ) -> Result<()> {
        let offsets_pda = {self.deploy_offsets.to_account_info()};
        let mut data = offsets_pda.try_borrow_mut_data()?;
        let msg_index = {u16::from_le_bytes(array_ref!(&incoming,0,2).clone())}; // message index u16
        let msg_len = incoming.len() - 2;
        let start_offset = msg_index as usize * msg_len;

        data
            .get_mut(start_offset..start_offset+msg_len)
            .ok_or(ProgramError::AccountBorrowFailed)?
            .copy_from_slice(&incoming[2..]);

        // let this_data = RefMut::map(data, |data| {
        //     bytemuck::from_bytes_mut(&mut data.deref_mut()[10..])
        // });

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
