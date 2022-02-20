use byteorder::{ByteOrder, LittleEndian};

use solana_program::{
    program_error::ProgramError,
    program_pack::{Pack, Sealed},
    msg
};

use crate::error::VoteError;


pub struct Vote {
    pub candidate: u8,
}

impl Sealed for Vote {}

impl Pack for Vote {
    const LEN: usize = 1;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let candidate = src[0];

        if candidate != 1 && candidate != 2 {
            msg!("Vote must be for candidate 1 or 2");
            return Err(VoteError::UnexpectedCandidate.into());
        }
        Ok(Vote { candidate })
    }

    fn pack_into_slice(&self, _dst: &mut [u8]) {}
}

// Vote Check structure, which is one 4 byte u32 number
// contains zero if they havn't voted, or the candidatIsInitializede number if they have

pub struct VoterCheck {
    pub voted_for: u32,
}

impl Sealed for VoterCheck {}

impl Pack for VoterCheck {
    const LEN: usize = 4;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        Ok(VoterCheck {
            voted_for: LittleEndian::read_u32(&src[0..4]),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        LittleEndian::write_u32(&mut dst[0..4], self.voted_for);
    }
}

// Vote Count structure, which is two 4 byte u32 numbers
// first number is candidate 1's vote count, second number is candidate 2's vote count

pub struct VoteCount {
    pub candidate1: u32,
    pub candidate2: u32,
}

impl Sealed for VoteCount {}

impl Pack for VoteCount {
    const LEN: usize = 8;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        Ok(VoteCount {
            candidate1: LittleEndian::read_u32(&src[0..4]),
            candidate2: LittleEndian::read_u32(&src[4..8]),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        LittleEndian::write_u32(&mut dst[0..4], self.candidate1);
        LittleEndian::write_u32(&mut dst[4..8], self.candidate2);
    }
}