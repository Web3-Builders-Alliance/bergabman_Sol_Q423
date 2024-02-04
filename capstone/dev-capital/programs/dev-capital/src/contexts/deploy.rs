use crate::state::{DeployData, DeployOffsets, DevConfig, DevFund};
use amplify_num::u24;
use anchor_lang::{prelude::*, solana_program::{compute_units::sol_remaining_compute_units, log::sol_log_compute_units}};
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

        // msg!("offsets_pda_data {:?}", { &offsets_pda_data[..20] });
        // msg!("config {:#?}", &self.dev_config);
        let offsets_6_len = self.dev_config.ot_6_len as usize * 3;
        // msg!("offsets_6_len from config {}", &offsets_6_len);
        let mut offsets_6_index = self.dev_config.ot_6_index;
        let offsets_5_len = self.dev_config.ot_5_len as usize * 3;
        // msg!("offsets_5_len from config {}", &offsets_5_len);
        let mut offsets_5_index = self.dev_config.ot_5_index;
        let mut shifting_end = self.dev_config.shifting_end;
        // msg!("shifting_end from config {}", &shifting_end);
        let offsets_6 = { &offsets_pda_data[8..offsets_6_len] };
        let offsets_5 = { &offsets_pda_data[8 + offsets_6_len..8 + offsets_6_len + offsets_5_len] };
        // msg!("offsets_pda_data {:?}", { &offsets_pda_data[..20] });
        // msg!("offsets_5 {:?}", &offsets_5[..20]);
        // msg!("offsets_6 {:?}", &offsets_6[..20]);

        // let mut shift_start = 0u32;
        // let mut shift_end = shifting_end;
        let mut tmp_buf: Box<Vec<u8>> = Box::new(Vec::with_capacity(30000));
        msg!("allocated temp buffer {}", &tmp_buf.capacity());
        sol_log_compute_units();

        // let mut base_slice_end: Option<usize> = None;
        let mut this_offset = 0u32;
        let mut to_move = 0u32;
        let mut move_chunks: Box<Vec<usize>> = Box::new(Vec::with_capacity(20));
        let mut whole_buf = 0u32;
        let mut part_buf = 0u32;
        let mut chunk_start = 0usize;
        let mut chunk_end = 0usize;

        for i in 0..offsets_5_len {
            offsets_5_index += 1;
            this_offset =
                u24::from_le_bytes(*array_ref![offsets_5, i as usize * 3, 3]).into();

            if sol_remaining_compute_units() < 30_000 || sol_remaining_compute_units() > 1_370_000{
                msg!("this_offset {}", &this_offset);
                sol_log_compute_units();
                if sol_remaining_compute_units() < 10_000 {
                    msg!("{} compute unites left", sol_remaining_compute_units());
                    msg!("current offsets5 index {} from {}", offsets_5_index, self.dev_config.ot_5_len);
                    msg!("current offsets6 index {} from {}", offsets_6_index, self.dev_config.ot_6_len);
                    break;
                }
               
            }
            
            to_move = shifting_end - this_offset;
            // msg!("to_move {}", &to_move);
            whole_buf = to_move / tmp_buf.capacity() as u32;
            // msg!("whole_buf {}", &whole_buf);
            part_buf = to_move % tmp_buf.capacity() as u32;
            // msg!("part_buf {}", &part_buf);
            for _ in 0..whole_buf {
                move_chunks.push(tmp_buf.capacity())
            }
            // move_chunks = Box::new(vec![tmp_buf.capacity(); whole_buf as usize]);
            move_chunks.push(part_buf as usize);
            // msg!("move_parts {:?}", &move_chunks);
            // sol_log_compute_units();

            // let base_slice_start = this_offset;
            // let mut sub_slice_start = 0usize;
            chunk_end = shifting_end as usize;
            // let mut chunk_start = 0usize;
            shifting_end += 5;

            for chunk in move_chunks.iter() {
                chunk_start = chunk_end
                    .checked_sub(*chunk)
                    .ok_or(ProgramError::ArithmeticOverflow)?;

                {
                    tmp_buf.extend(&data_pda_data[chunk_start..chunk_end]);
                }

                data_pda_data
                    .get_mut(chunk_start + 5..chunk_end + 5)
                    .ok_or(ProgramError::AccountBorrowFailed)?
                    .copy_from_slice(&tmp_buf);

                tmp_buf.clear();
                // chunk_end = chunk_start;
            }
            move_chunks.clear();

            data_pda_data
                .get_mut(this_offset as usize..this_offset as usize + 5)
                .ok_or(ProgramError::AccountBorrowFailed)?
                .copy_from_slice(&[0u8; 5]);

            // msg!("offset {} done", this_offset);
            // sol_log_compute_units();
        }

        Ok(())
    }
}
