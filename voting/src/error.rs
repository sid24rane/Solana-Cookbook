use thiserror::Error;

use solana_program::{decode_error::DecodeError, program_error::ProgramError};

#[derive(Error, Debug, Copy, Clone)]
pub enum VoteError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Not Rent Exempt
    #[error("Not Rent Exempt")]
    NotRentExempt,
    /// Expected Amount Mismatch
    #[error("Expected Amount Mismatch")]
    ExpectedAmountMismatch,
    /// Amount Overflow
    #[error("Amount Overflow")]
    AmountOverflow,
    // Unexpected Candidate
    #[error("Unexpected Candidate")]
    UnexpectedCandidate,
    // Incorrect Owner
    #[error("Incorrect Owner")]
    IncorrectOwner,
    // Account Owes Rent
    #[error("Account Not Rent Exempt")]
    AccountNotRentExempt,
    // Account does not match Check account
    #[error("Account Not Check Account")]
    AccountNotCheckAccount,
    // Already Voted
    #[error("Already Voted")]
    AlreadyVoted,
}

impl From<VoteError> for ProgramError {
    fn from(e: VoteError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for VoteError {
    fn type_of() -> &'static str {
        "Vote Error"
    }
}
