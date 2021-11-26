use core::panic;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{LogInstruction, LogSetup};

pub fn process(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = LogInstruction::try_from_slice(instruction_data)?;
    match instruction {
        LogInstruction::SetupLog => {
            msg!("Instruction: SetupLog");
            process_setup_log(accounts)
        }
        LogInstruction::LogRustLocation => {
            msg!("Instruction: LogRustLocation");
            process_log_rust_location(accounts)
        }
    }
}

fn process_setup_log(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let _payer = next_account_info(account_info_iter)?;
    let log_info = next_account_info(account_info_iter)?;

    let mut log_setup = if log_info.data_is_empty() {
        LogSetup::new()
    } else {
        let log_setup = LogSetup::from_account_info(&log_info)?;
        if log_setup.is_initialized {
            // Printing location in Rust code here before returning an error instead of using `panic!`
            // to archieve the same
            msg!("at {:?}", panic::Location::caller());
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        log_setup
    };

    msg!("{:#?}", log_setup);

    log_setup = LogSetup {
        is_initialized: true,
        times_invoked: 0,
    };

    msg!("Updating account data");

    log_setup.serialize(&mut &mut log_info.try_borrow_mut_data()?.as_mut())?;

    Ok(())
}

/// This instruction basically does nothing, but shows how to log the location in the Rust code
fn process_log_rust_location(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let _payer = next_account_info(account_info_iter)?;
    let log_info = next_account_info(account_info_iter)?;
    let log_setup = LogSetup::from_account_info(&log_info)?;

    msg!("at {:?}", panic::Location::caller());
    msg!("{:#?}", log_setup);

    Ok(())
}
