use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not rent exempt")]
    NotRentExempt,
    #[error("Expected Amount Mismatch")]
    ExpectedAmountMismatch,
    #[error("Amout Overflow")]
    AmountOverflow,
}

impl From<EscrowError> for ProgramError {
    fn from(err: EscrowError) -> ProgramError {
        ProgramError::Custom(err as u32)
    }
}
