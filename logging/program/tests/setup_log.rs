mod setup_log {
    use borsh::BorshSerialize;
    use sol_logging::{process, setup_log, LogSetup};
    use solana_program::pubkey::Pubkey;
    use solana_program_test::{processor, tokio, ProgramTest};
    use solana_sdk::{account::Account, signer::Signer, transaction::Transaction};

    #[tokio::test]
    async fn success_setup_log() {
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

        let ix = setup_log(program_id, ctx.payer.pubkey(), log_pubkey);

        let mut tx = Transaction::new_with_payer(&[ix], Some(&ctx.payer.pubkey()));
        tx.sign(&[&ctx.payer], ctx.last_blockhash);

        ctx.banks_client
            .process_transaction(tx)
            .await
            .expect("Setup Log succeeds");

        let log_setup: LogSetup = ctx
            .banks_client
            .get_account_data_with_borsh(log_pubkey)
            .await
            .unwrap();

        eprintln!("{:#?}", log_setup);
    }
}
