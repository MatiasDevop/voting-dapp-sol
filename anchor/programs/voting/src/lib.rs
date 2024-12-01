#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

#[program]
pub mod voting{
    use super::*;

    pub fn initialize_poll(ctx: Context<InitializePoll>,
                            poll_id: u64,
                            description: String,
                            poll_start: u64,
                            poll_end: u64,
                        ) -> Result<()> { 
        let poll = &mut ctx.accounts.poll;
        poll.poll_id = poll_id;
        poll.description = description;
        poll.poll_start = poll_start;
        poll.poll_end = poll_end;
        poll.candidate_amount = 0;
        Ok(())
    }

    pub fn initialize_candidate(ctx: Context<InitializeCandidate>,
                                candidate_name: String,
                                _poll_id: u64) -> Result<()>{
        let candidate = &mut ctx.accounts.candidate;
        candidate.candidate_name = candidate_name;
        candidate.candidate_votes = 0;
        Ok(())                       
    }

}

//make sure have the same order
#[derive(Accounts)]
#[instruction(candidate_name: String, poll_id: u64)]
pub struct InitializeCandidate<'info>{
    #[account(mut)]
    pub signer: Signer<'info>, // cuz someone has ot pay for it

    #[account( // this is the poll
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    //Candidate
    #[account(
        init,
        payer = signer,                  
        space = 8 + Candidate::INIT_SPACE,   
        seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_bytes()],      // PDA seeds for the account
        bump,
    )]
    pub candidate: Account<'info, Candidate>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(280)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}


#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    // The signer of the transaction (mutable)
    #[account(mut)]
    pub signer: Signer<'info>,
    // The poll account to be initialized
    #[account(
        init,                           // Initialize a new account
        payer = signer,                  // The account paying for the initialization
        space = 8 + Poll::INIT_SPACE,   // Space allocated for the account
        seeds = [poll_id.to_le_bytes().as_ref()],      // PDA seeds for the account
        bump,
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}

// Defines the structure of a Poll account
#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id: u64,
    #[max_len(280)]
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64,
}