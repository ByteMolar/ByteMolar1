use solana_program::{
    pubkey::Pubkey,
    system_instruction,
};
use solana_program_test::*;
use solana_sdk::{
    signature::Signer,
    transaction::Transaction,
};

mod helpers;

#[tokio::test]
async fn test_create_clinic() {
    let program_id = Pubkey::new_unique();
    let (mut banks_client, payer, recent_blockhash) = 
        helpers::program_test(program_id).start().await;

    // Create clinic account
    let clinic_account = solana_sdk::signature::Keypair::new();
    let rent = banks_client.get_rent().await.unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[system_instruction::create_account(
            &payer.pubkey(),
            &clinic_account.pubkey(),
            rent.minimum_balance(1000),
            1000,
            &program_id,
        )],
        Some(&payer.pubkey()),
        &[&payer, &clinic_account],
        recent_blockhash,
    );

    banks_client.process_transaction(tx).await.unwrap();
}

#[tokio::test]
async fn test_add_dental_record() {
    let program_id = Pubkey::new_unique();
    let (mut banks_client, payer, recent_blockhash) = 
        helpers::program_test(program_id).start().await;

    // Test implementat