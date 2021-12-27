use solana_client::rpc_client::RpcClient;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    system_program,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signature, Signer},
    transaction::Transaction,
};
use std::convert::TryFrom;

pub fn airdrop(
    client: &RpcClient,
    to_pubkey: &Pubkey,
    lamports: u64,
) -> anyhow::Result<Signature> {
    let (blockhash, _fee_calculator) = client.get_recent_blockhash()?;
    let signature = client
        .request_airdrop_with_blockhash(to_pubkey, lamports, &blockhash)?;
    client.confirm_transaction_with_spinner(
        &signature,
        &blockhash,
        CommitmentConfig::finalized(),
    )?;

    Ok(signature)
}

pub fn main() -> anyhow::Result<()> {
    let program_id = vault::id();
    let payer = Keypair::new();
    let rpc_client = RpcClient::new("http://localhost:8899".to_string());
    eprintln!("Airdropping to payer: {:?}", payer.pubkey());
    airdrop(&rpc_client, &payer.pubkey(), LAMPORTS_PER_SOL)?;

    // Derive the PDA from the payer account, a string representing the unique
    // purpose of the account ("vault"), and the address of our on-chain program.
    let (vault_pubkey, vault_bump_seed) = Pubkey::find_program_address(
        &[b"vault", payer.pubkey().as_ref()],
        &program_id,
    );

    eprintln!("vault_pubkey {:?}", vault_pubkey);
    // Get the amount of lamports needed to pay for the vault's rent
    let vault_account_size = usize::try_from(vault::VAULT_ACCOUNT_SIZE)?;
    let lamports = rpc_client
        .get_minimum_balance_for_rent_exemption(vault_account_size)?;

    // The on-chain program's instruction data, imported from that program's crate.
    let instr_data = vault::InstructionData {
        vault_bump_seed,
        lamports,
    };

    // The accounts required by both our on-chain program and the system program's
    // `create_account` instruction, including the vault's address.
    let accounts = vec![
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(vault_pubkey, false),
        AccountMeta::new(system_program::ID, false),
    ];

    // Create the instruction by serializing our instruction data via borsh
    let instruction =
        Instruction::new_with_borsh(program_id, &instr_data, accounts);

    let (blockhash, _) = rpc_client.get_recent_blockhash()?;

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    rpc_client.send_and_confirm_transaction(&transaction)?;
    Ok(())
}
