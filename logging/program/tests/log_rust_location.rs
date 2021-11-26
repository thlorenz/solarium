mod log_rust_location {
    use borsh::BorshSerialize;
    use sol_logging::{log_rust_location, process, LogSetup};
    use solana_program::pubkey::Pubkey;
    use solana_program_test::{processor, tokio, ProgramTest};
    use solana_sdk::{account::Account, signer::Signer, transaction::Transaction};

    #[tokio::test]
    async fn success_log_rust_location() {
        let program_id = Pubkey::new_unique();

        let log_setup = LogSetup::new();
        let log_pubkey = Pubkey::new_unique();
        let log_account = Account {
            lamports: 1,
            data: log_setup.try_to_vec().unwrap(),
            owner: program_id,
            ..Account::default()
        };

        let mut pt = ProgramTest::new("sol_logging", program_id, processor!(process));
        pt.add_account(log_pubkey, log_account);

        let mut ctx = pt.start_with_context().await;

        let ix = log_rust_location(program_id, ctx.payer.pubkey(), log_pubkey);

        let mut tx = Transaction::new_with_payer(&[ix], Some(&ctx.payer.pubkey()));
        tx.sign(&[&ctx.payer], ctx.last_blockhash);

        ctx.banks_client
            .process_transaction(tx)
            .await
            .expect("Log Rust Location succeeds");
    }
}
