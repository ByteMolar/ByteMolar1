use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum ByteMolarError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not Rent Exempt")]
    NotRentExempt,
    #[error("Unauthorized Access")]
    UnauthorizedAccess,
    #[error("Invalid Patient Data")]
    InvalidPatientData,
}

impl From<ByteMolarError> for ProgramError {
    fn from(e: ByteMolarError) -> Self {
        ProgramError::Custom(e as u32)
    }
}