use anchor_lang::prelude::*;

declare_id!("3nSKMvr8rJa6fih43gFHSW8TsVozKxLuy1H2VpYXyByV");  // program's public key

#[program]
pub mod voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

// #[derive(Accounts)]
// pub struct Initialize {}

#[account]
#[derive(InitSpace)]
pub struct PollAccount {
    #[max_len(32)]
    pub poll_name : String,
    #[max_len(256)]
    pub poll_description : String,
    pub poll_voting_start : u64,
    pub poll_voting_end : u64,
    pub poll_optoin_index : u64,
}


#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    #[max_len(64)]
    pub candidate_name :  String,
    pub candidate_votes : u64,
}


