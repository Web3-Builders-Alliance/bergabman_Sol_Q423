use crate::state::{DeployData, DeployOffsets, DevConfig, DevFund};
use amplify_num::u24;
use anchor_lang::{prelude::*, solana_program::log::sol_log_compute_units};
use arrayref::array_ref;

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
    pub fn deploy_offsets(&mut self, incoming: &[u8]) -> Result<()> {
        // msg!("incoming_offset {:?}", {&incoming[..20]});
        let offsets_pda = self.deploy_offsets.to_account_info();
        let mut data = offsets_pda.try_borrow_mut_data()?;
        let msg_index = { u16::from_le_bytes(*array_ref!(&incoming, 0, 2))};
        msg!("msg index {}", msg_index);
        let msg_len = { &incoming[2..].len() };
        msg!("msg len {}", msg_len);
        let static_len = 900;
        let start_offset: usize = if msg_index.clone() == 0 {
            8.clone()
        } else {
            (8 + (msg_index as usize * static_len as usize)).clone()
        };

        data.get_mut(start_offset..start_offset + *msg_len)
            .ok_or(ProgramError::AccountBorrowFailed)?
            .copy_from_slice(&incoming[2..]);

        Ok(())
    }

    pub fn deploy_data(&mut self, incoming: &[u8]) -> Result<()> {
        // msg!("incoming_data {:?}", {&incoming[..20]});

        let data_pda = self.deploy_data.to_account_info();
        let mut data = data_pda.try_borrow_mut_data()?;
        let msg_index = { u16::from_le_bytes(*array_ref!(&incoming, 0, 2)) };
        msg!("msg index {}", msg_index);
        let msg_len = { &incoming[2..].len() };
        msg!("msg len {}", msg_len);
        let static_len = 900;
        let start_offset: usize = if msg_index.clone() == 0 {
            8.clone()
        } else {
            (8 + (msg_index as usize * static_len as usize)).clone()
        };

        data.get_mut(start_offset..start_offset + *msg_len)
            .ok_or(ProgramError::AccountBorrowFailed)?
            .copy_from_slice(&incoming[2..]);

        Ok(())
    }

    #[allow(unused_assignments)]
    pub fn decompress_data(&mut self) -> Result<()> {
        let offsets_pda = self.deploy_offsets.to_account_info();
        let data_pda = self.deploy_data.to_account_info();
        let mut offsets_pda_data = offsets_pda.try_borrow_mut_data()?;
        let mut data_pda_data = data_pda.try_borrow_mut_data()?;

        msg!("offsets_pda_data {:?}", { &offsets_pda_data[..20] });
        msg!("config {:#?}", &self.dev_config);
        let offsets_6_len = self.dev_config.ot_6_len as usize * 3;
        msg!("offsets_6_len from config {}", &offsets_6_len);
        let mut offsets_6_index = self.dev_config.ot_6_index;
        let offsets_5_len = self.dev_config.ot_5_len as usize * 3;
        msg!("offsets_5_len from config {}", &offsets_5_len);
        let mut offsets_5_index = self.dev_config.ot_5_index;
        let mut shifting_end = self.dev_config.shifting_end;
        msg!("shifting_end from config {}", &shifting_end);
        let offsets_6 = { &offsets_pda_data[8..offsets_6_len] };
        let offsets_5 = { &offsets_pda_data[8 + offsets_6_len..8 + offsets_6_len + offsets_5_len] };
        msg!("offsets_pda_data {:?}", { &offsets_pda_data[..20] });
        msg!("offsets_5 {:?}", &offsets_5[..20]);
        msg!("offsets_6 {:?}", &offsets_6[..20]);

        let mut shift_start = 0u32;
        let mut shift_end = shifting_end;
        let mut tmp_buf: Box<Vec<u8>> = Box::new(Vec::with_capacity(25000));
        msg!("allocated temp buffer {}", &tmp_buf.capacity());
        sol_log_compute_units();

        let mut base_slice_end: Option<usize> = None;

        for i in 0..offsets_5_len {
            offsets_5_index += 1;
            let this_offset: u32 =
                u24::from_le_bytes(*array_ref![offsets_5, i as usize * 3, 3]).into();
            msg!("this_offset {}", &this_offset);
            let to_move = shifting_end - this_offset;
            // msg!("to_move {}", &to_move);
            let whole_buf = to_move / tmp_buf.capacity() as u32;
            // msg!("whole_buf {}", &whole_buf);
            let part_buf = to_move % tmp_buf.capacity() as u32;
            // msg!("part_buf {}", &part_buf);
            let mut move_parts = vec![tmp_buf.capacity(); whole_buf as usize];
            move_parts.push(part_buf as usize);
            msg!("move_parts {:?}", &move_parts);
            sol_log_compute_units();

            // let base_slice_start = this_offset;
            let mut sub_slice_start = 0usize;
            let mut sub_slice_end = 0usize;

            for sub_slice_len in move_parts.iter() {
                if let Some(base_slice_end) = base_slice_end {
                    sub_slice_end = base_slice_end;
                } else {
                    sub_slice_end = shifting_end as usize;
                    shifting_end += 5;
                }
                sub_slice_start = sub_slice_end - sub_slice_len;
                base_slice_end = Some(sub_slice_start);

                {
                    tmp_buf.extend_from_slice(&data_pda_data[sub_slice_start..sub_slice_end]);
                }

                data_pda_data
                    .get_mut(sub_slice_start + 5..sub_slice_end + 5)
                    .ok_or(ProgramError::AccountBorrowFailed)?
                    .copy_from_slice(&tmp_buf);
                tmp_buf.clear();
            }
            data_pda_data
                .get_mut(this_offset as usize..this_offset as usize + 5)
                .ok_or(ProgramError::AccountBorrowFailed)?
                .copy_from_slice(&[0u8; 5]);

            msg!("offset {} done", this_offset);
            sol_log_compute_units();
        }

        Ok(())
    }
}
