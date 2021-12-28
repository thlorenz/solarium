use std::ops::{AddAssign, SubAssign};

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::{Pubkey, PUBKEY_BYTES},
    system_instruction,
};

// The custom instruction processed by our program. It includes the
// PDA's bump seed, which is derived by the client program. This
// definition is also imported into the off-chain client program.
// The computed address of the PDA will be passed to this program via
// the `accounts` vector of the `Instruction` type.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InstructionData {
    // 0: init
    // 1: close
    pub instruction: u8,
    pub vault_bump_seed: u8, // only use for vault_init
    pub lamports: u64,       // how much to add to / withdraw from vault
}

// The size in bytes of a vault account. The client program needs
// this information to calculate the quantity of lamports necessary
// to pay for the account's rent.
pub static VAULT_ACCOUNT_SIZE: u64 = PUBKEY_BYTES as u64;

entrypoint!(process_instruction);
solana_program::declare_id!("6ds1BgdmEDDX74bNbpyw8Sm12vFasGL4wqKcbv1wuwDp");

// The entrypoint of the on-chain program, as provided to the
// `entrypoint!` macro.
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let mut instruction_data = instruction_data;
    let instr = InstructionData::deserialize(&mut instruction_data)?;
    match instr.instruction {
        0 => process_vault_init(program_id, accounts, &instr),
        1 => process_vault_withdraw(program_id, accounts, &instr),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

// -----------------
// Init Vault
// -----------------
fn process_vault_init(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instr: &InstructionData,
) -> ProgramResult {
    msg!("Instruction: Vault Init");

    let account_info_iter = &mut accounts.iter();
    let payer = next_account_info(account_info_iter)?;
    // The vault PDA, derived from the payer's address
    let vault = next_account_info(account_info_iter)?;

    let vault_bump_seed = instr.vault_bump_seed;
    let lamports = instr.lamports;
    let vault_size = VAULT_ACCOUNT_SIZE;

    // Invoke the system program to create an account while virtually
    // signing with the vault PDA, which is owned by this caller program.
    invoke_signed(
        &system_instruction::create_account(
            &payer.key,
            &vault.key,
            lamports,
            vault_size,
            &program_id,
        ),
        &[payer.clone(), vault.clone()],
        // A slice of seed slices, each seed slice being the set
        // of seeds used to generate one of the PDAs required by the
        // callee program, the final seed being a single-element slice
        // containing the `u8` bump seed.
        &[&[b"vault", payer.key.as_ref(), &[vault_bump_seed]]],
    )?;
    msg!("vault_bump_seed: {}", vault_bump_seed);

    // &[&[b"vault", payer.key.as_ref(), &[vault_bump_seed]]],
    payer
        .key
        .serialize(&mut &mut vault.try_borrow_mut_data()?.as_mut())?;

    Ok(())
}

// -----------------
// Withdraw From Vault
// -----------------
fn process_vault_withdraw(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instr: &InstructionData,
) -> ProgramResult {
    msg!("Instruction: Vault Withdraw");
    let account_info_iter = &mut accounts.iter();
    let payer = next_account_info(account_info_iter)?;
    let vault = next_account_info(account_info_iter)?;

    let pubkey_in_vault: Pubkey = vault.deserialize_data().unwrap();
    if !payer.key.eq(&pubkey_in_vault) {
        msg!("Only the payer that added to the vault can withdraw from it");
        return Err(ProgramError::IllegalOwner);
    }

    let lamports = instr.lamports;
    if lamports > vault.lamports() {
        msg!("Vault only holds {} lamports", vault.lamports());
        return Err(ProgramError::InsufficientFunds);
    }

    msg!("Will withdraw {} lamports", lamports);
    if !payer.is_writable {
        msg!("payer needs to be writable");
        return Err(ProgramError::InvalidAccountData);
    }
    if !vault.owner.eq(program_id) {
        msg!("vault account needs to owned by vault");
        return Err(ProgramError::IllegalOwner);
    }

    let vault_before = vault.lamports();
    let payer_before = payer.lamports();
    vault.lamports.borrow_mut().sub_assign(lamports);
    payer.lamports.borrow_mut().add_assign(lamports);
    let vault_after = vault.lamports();
    let payer_after = payer.lamports();

    msg!("vault: {} -> {}", vault_before, vault_after,);
    msg!("payer: {} -> {}", payer_before, payer_after,);

    Ok(())
}
