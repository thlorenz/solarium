#![allow(unused)]
use solana_program::{
    decode_error::DecodeError,
    program_error::{PrintProgramError, ProgramError},
};

use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum LogError {
    #[error("Failed to unpack instruction data")]
    InstructionUnpackError,

    #[error("Data type mismatch")]
    DataTypeMismatch,
}

// -----------------
// Trait Impls
// -----------------
impl PrintProgramError for LogError {
    fn print<E>(&self) {
        //        msg!(&self.to_string());
    }
}

impl From<LogError> for ProgramError {
    fn from(e: LogError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for LogError {
    fn type_of() -> &'static str {
        "Log Error"
    }
}
