use anchor_lang::prelude::*;

declare_id!("5MHA6ForrBUbPbom2x231cNsMCQvE4VCpQ7F6rKMt8bS");

#[program]
pub mod dev_capital {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    pub fn deploy(ctx: Context<Initialize>) -> Result<()> {
        // develper deploys program to the contract
        // program gets decompressed by contract and saved in pda
        Ok(())
    }
    pub fn support_deploy(ctx: Context<Initialize>) -> Result<()> {
        // funding for a developer by other(s) for program deployment
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
