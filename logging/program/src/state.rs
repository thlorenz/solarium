use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, borsh::try_from_slice_unchecked, msg, program_error::ProgramError,
};

use crate::error::LogError;

const MAX_LOG_SETUP_LEN: usize = 1 + 4;

pub fn try_from_slice_checked<T: BorshDeserialize>(
    data: &[u8],
    data_size: usize,
) -> Result<T, ProgramError> {
    if data.len() != data_size {
        msg!("Data len mismatch: {} != {}", data.len(), data_size);
        return Err(LogError::DataTypeMismatch.into());
    }

    let result: T = try_from_slice_unchecked(data)?;

    Ok(result)
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct LogSetup {
    /* 1 */ pub is_initialized: bool,
    /* 4 */ pub times_invoked: u32,
}

impl LogSetup {
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            times_invoked: 0,
        }
    }
    pub fn from_account_info(a: &AccountInfo) -> Result<LogSetup, ProgramError> {
        let md: LogSetup = try_from_slice_checked(&a.data.borrow(), MAX_LOG_SETUP_LEN)?;
        Ok(md)
    }
}
