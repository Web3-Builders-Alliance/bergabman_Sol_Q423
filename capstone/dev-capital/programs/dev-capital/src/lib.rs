use anchor_lang::prelude::*;

declare_id!("5MHA6ForrBUbPbom2x231cNsMCQvE4VCpQ7F6rKMt8bS");

#[program]
pub mod dev_capital {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
