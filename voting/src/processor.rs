use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{
    error::VoteError,
    state::{Vote, VoteCount, VoterCheck},
};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {

        // NOTE: Not using the instruction part for now.


        // get candidate to vote for from instruction_data (unchecked because data is not null)
        let candidate = Vote::unpack_unchecked(&instruction_data)?.candidate;

        // Iterating accounts is safer then indexing
        let accounts_iter = &mut accounts.iter();

        // Get the account that holds the vote count
        let count_account = next_account_info(accounts_iter)?;

        // The account must be owned by the program in order to modify its data
        if count_account.owner != program_id {
            msg!(
                "Vote count account ({}) not owned by program, actual: {}, expected: {}",
                count_account.key,
                count_account.owner,
                program_id
            );
            return Err(VoteError::IncorrectOwner.into());
        }

        // Get the account that checks for dups
        let check_account = next_account_info(accounts_iter)?;

        // The check account must be owned by the program in order to modify its data
        if check_account.owner != program_id {
            msg!("Check account not owned by program");
            return Err(VoteError::IncorrectOwner.into());
        }

        // The account must be rent exempt, i.e. live forever
        let sysvar_account = next_account_info(accounts_iter)?;
        let rent = &Rent::from_account_info(sysvar_account)?;
        if !solana_program::sysvar::rent::check_id(sysvar_account.key) {
            msg!("Rent system account is not rent system account");
            return Err(ProgramError::InvalidAccountData);
        }
        if !rent.is_exempt(check_account.lamports(), check_account.data_len()) {
            msg!("Check account is not rent exempt");
            return Err(VoteError::AccountNotRentExempt.into());
        }

        // the voter
        let voter_account = next_account_info(accounts_iter)?;

        if !voter_account.is_signer {
            msg!("Voter account is not signer");
            return Err(ProgramError::MissingRequiredSignature);
        }

        let expected_check_account_pubkey =
            Pubkey::create_with_seed(voter_account.key, "checkvote", program_id)?;

        if expected_check_account_pubkey != *check_account.key {
            msg!("Voter fraud! not the correct check_account");
            return Err(VoteError::AccountNotCheckAccount.into());
        }

        let mut check_data = check_account.try_borrow_mut_data()?;

        // this unpack reads and deserialises the account data and also checks the data is the correct length

        let mut vote_check =
            VoterCheck::unpack_unchecked(&check_data).expect("Failed to read VoterCheck");

        if vote_check.voted_for != 0 {
            msg!("Voter fraud! You already voted");
            return Err(VoteError::AlreadyVoted.into());
        }

        // Increment vote count of candidate, and record voter's choice

        let mut count_data = count_account.try_borrow_mut_data()?;

        let mut vote_count =
            VoteCount::unpack_unchecked(&count_data).expect("Failed to read VoteCount");

        match candidate {
            1 => {
                vote_count.candidate1 += 1;
                vote_check.voted_for = 1;
                msg!("Voting for candidate1!");
            }
            2 => {
                vote_count.candidate2 += 1;
                vote_check.voted_for = 2;
                msg!("Voting for candidate2!");
            }
            _ => {
                msg!("Unknown candidate");
                return Err(ProgramError::InvalidInstructionData);
            }
        }

        VoteCount::pack(vote_count, &mut count_data).expect("Failed to write VoteCount");
        VoterCheck::pack(vote_check, &mut check_data).expect("Failed to write VoterCheck");

        Ok(())
    }
}
