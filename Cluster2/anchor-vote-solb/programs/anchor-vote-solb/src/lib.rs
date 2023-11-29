use anchor_lang::prelude::*;

declare_id!("D8uPUKvwE6xftD3MGyT5wxJWt4p7jsCQLacQay45iiyB");

#[program]
pub mod anchor_vote_solb {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
