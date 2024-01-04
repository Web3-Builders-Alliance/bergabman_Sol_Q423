use crate::{errors::AmmError, Config};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{Mint, TokenAccount},
};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = payer,
        seeds = [b"lp", config.key.as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = auth,
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_x,
        associated_token::authority = auth
    )]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_y,
        associated_token::authority = auth
    )]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,
    #[account(seeds = [b"auth", config.key().as_ref()], bump)]
    pub auth: UncheckedAccount<'info>,
    #[account(
        init,
        payer = payer,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump,
        space = Config::INIT_SPACE,
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Initialize<'info> {
    pub fn init(
        &mut self,
        bumps: &InitializeBumps,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        require!(fee <= 10000, AmmError::InvalidFee);

        let (auth_bump, config_bump, lp_bump) = (bumps.auth, bumps.config, bumps.mint_lp);

        self.config.init(
            seed,
            authority,
            self.mint_x.key(),
            self.mint_y.key(),
            fee,
            auth_bump,
            lp_bump,
            config_bump,
        );
        Ok(())
    }
}
