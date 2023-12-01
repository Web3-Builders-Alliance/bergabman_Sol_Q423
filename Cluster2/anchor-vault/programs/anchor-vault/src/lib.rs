use anchor_lang::prelude::*;

declare_id!("5CGXgvPRVtW81njC68V46DKzoXFoysteMezTS67JL4iL");

#[program]
pub mod anchor_vault {
    use anchor_lang::system_program::{Transfer, transfer};

    use super::*;

    pub fn deposit(ctx: Context<Vault>, lamports: u64) -> Result<()> {

        let accounts = Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            accounts
        );

        transfer(cpi_ctx, lamports)?;

        Ok(())
    }

    pub fn close(ctx: Context<Vault>/*, _lamports: u64*/) -> Result<()> {

        // let signer_seeds: &[&[_]] = &[
        //     b"vault", 
        //     &ctx.accounts.signer.to_account_info().key.to_bytes(), 
        //     &[ctx.bumps.vault]
        // ];

        // let signer_seeds: &[&[u8]] = &[b"vault", ctx.accounts.signer.to_account_info().key.as_ref(), &[ctx.bumps.vault]];

        let seeds = &[
            b"vault",
            ctx.accounts.signer.to_account_info().key.as_ref(),
            &[ctx.bumps.vault]
        ];

        let signer_seeds = &[&seeds[..]];

        let accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.signer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            accounts,
            signer_seeds
        );

        transfer(cpi_ctx, ctx.accounts.vault.lamports())?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Vault<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump
    )]
    vault: SystemAccount<'info>,
    system_program: Program<'info, System>
}
