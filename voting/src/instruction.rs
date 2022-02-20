use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::VoteError;

pub enum VoteInstruction {
    /// Votes for the candidate
    ///
    /// Accounts expected:
    ///
    /// 0. `[writable]` Account that store all vote counts
    /// 1. `[]` Check Account to chekc if voter account has already voted
    /// 2. `[signer]` Actual voters account
    /// 3. `[]` The rent sysvar
    Vote {
        /// The amount party A expects to receive of token Y
        candidate: u32,
    }
}


// Not sure if I need these implementations.
impl VoteInstruction {
    /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(VoteError::InvalidInstruction)?;

        Ok(match tag {
            0 => Self::Vote {
                candidate: Self::unpack_amount(rest)?,
            },
            _ => return Err(VoteError::InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u32, ProgramError> {
        let candidate = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u32::from_le_bytes)
            .ok_or(VoteError::InvalidInstruction)?;
        Ok(candidate)
    }
}