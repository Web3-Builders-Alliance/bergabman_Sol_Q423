use anchor_lang::prelude::*;
use arrayref::array_ref;
use crate::state::{DevConfig, DeployData, DeployOffsets, DevFund};
use amplify_num::u24;

#[derive(Accounts)]
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
}

impl<'info> Deploy<'info> {
    pub fn deploy_offsets(
        &mut self,
        incoming: &[u8],
    ) -> Result<()> {
        let offsets_pda = self.deploy_offsets.to_account_info();
        let mut data = offsets_pda.try_borrow_mut_data()?;
        let msg_index = {u16::from_le_bytes(*array_ref!(&incoming,0,2))}; // message index u16
        let msg_len = {&incoming[2..].len()};
        let static_len = 900;
        let start_offset = msg_index as usize * static_len;

        data
            .get_mut(start_offset..start_offset+msg_len)
            .ok_or(ProgramError::AccountBorrowFailed)?
            .copy_from_slice(&incoming[2..]);

        Ok(())
    }

    pub fn deploy_data(
        &mut self,
        incoming: &[u8],
    ) -> Result<()> {
        let data_pda = self.deploy_data.to_account_info();
        let mut data = data_pda.try_borrow_mut_data()?;
        let msg_index = {u16::from_le_bytes(*array_ref!(&incoming,0,2))};
        let msg_len = {&incoming[2..].len()};
        let static_len = 900;
        let start_offset = msg_index as usize * static_len;

        data
            .get_mut(start_offset..start_offset+msg_len)
            .ok_or(ProgramError::AccountBorrowFailed)?
            .copy_from_slice(&incoming[2..]);

        Ok(())
    }

    #[allow(unused_variables, unused_mut)]
    pub fn decompress_data(&mut self) -> Result<()> {

        let offsets_pda = self.deploy_offsets.to_account_info();
        let data_pda = self.deploy_data.to_account_info();
        let mut offsets_pda_data = offsets_pda.try_borrow_mut_data()?;
        let mut data_pda_data = data_pda.try_borrow_mut_data()?;

        let offsets_6_len = self.dev_config.ot_6_len;
        let mut offsets_6_index = self.dev_config.ot_6_index;
        let offsets_5_len = self.dev_config.ot_5_len;
        let mut offsets_5_index = self.dev_config.ot_5_index;
        let mut shifting_end = self.dev_config.shifting_end;
        let offsets_6 = {&offsets_pda_data[3..offsets_6_len as usize]};
        let offsets_5 = {&offsets_pda_data[3+3+offsets_6_len as usize..offsets_5_len as usize]};

        
        let mut shift_start = 0u32;
        let mut shift_end = shifting_end;
        let mut tmp_buf: Box<Vec<u8>> = Box::new(Vec::with_capacity(32000));
        

        for i in 0..offsets_5_len/3 {
            offsets_5_index += 1;
            shifting_end += 5;
            let this_offset: u32 = u24::from_le_bytes(*array_ref![offsets_5, i as usize, 3]).into();


            if shift_end.checked_sub(this_offset).ok_or(ProgramError::ArithmeticOverflow)? > tmp_buf.len() as u32 {
                shift_start = shift_end - tmp_buf.len().into();
                // (shift_end-shift_start).euc
            } else {
                shift_start = shift_end.checked_sub(this_offset).ok_or(ProgramError::ArithmeticOverflow)?;
            }
            shift_end = shift_start;

            todo!();




            // if let Some(full) = shift_end.checked_rem(i as u32) {
            //     shift_start = shift_end-tmp_buf.capacity() as u32;

            // } else {
            //     shift_start = 
            // }
            // if shift_end.checked_rem(i).is_some() {
            //     shift_start = shift_end.checked_rem(tmp_buf.capacity() as u32).unwrap();
            // } else {
            //     shift_start = shift_end.checked_rem(i);
            // }
            // let current_offset = u24::from_le_bytes(*array_ref!(&offsets_5, i as usize*3, 3));
            // let current_offset = u24::from_le_bytes(&offsets_5[i as usize..i as usize+3]);




        }
        
        Ok(())
    }
}
