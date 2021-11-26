#![allow(unused)]
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum LogInstruction {
    /// Initializes a Logger
    ///
    /// Expected Accounts:
    ///
    /// 0. `[signer]` payer initializing the log
    /// 1. `[writable] The Log Account holding necessary info
    SetupLog,

    /// Logs Rust location
    ///
    /// Expected Accounts:
    ///
    /// 0. `[signer]` payer initializing the log
    /// 1. `[writable] The Log Account holding necessary info
    LogRustLocation,
}

pub fn setup_log(program_id: Pubkey, payer: Pubkey, log: Pubkey) -> Instruction {
    let accounts = vec![AccountMeta::new(payer, true), AccountMeta::new(log, false)];

    let data = LogInstruction::SetupLog.try_to_vec().unwrap();
    Instruction {
        program_id,
        accounts,
        data,
    }
}

pub fn log_rust_location(program_id: Pubkey, payer: Pubkey, log: Pubkey) -> Instruction {
    let accounts = vec![AccountMeta::new(payer, true), AccountMeta::new(log, false)];

    let data = LogInstruction::LogRustLocation.try_to_vec().unwrap();
    Instruction {
        program_id,
        accounts,
        data,
    }
}
