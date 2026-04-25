use anchor_lang::prelude::*;

declare_id!("42fY4Egx8NVSkU4GivRVyQuez83z15A7DrR5goM2YNaw");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
