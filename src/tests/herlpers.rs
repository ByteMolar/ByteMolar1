use solana_program::pubkey::Pubkey;
use solana_program_test::*;

pub fn program_test(program_id: Pubkey) -> ProgramTest {
    let mut program_test = ProgramTest::new(
        "bytemolar",
        program_id,
        processor!(bytemolar::processor::process_instruction),
    );
    program_test.set_compute_max_units(100_000);
    program_test
}

pub async fn create_clinic(
    banks_client: &mut BanksClient,
    program_id: &Pubkey,
    payer: &Keypair,
    recent_blockhash: &Hash,
) -> Result<(), TransportError> {
    // Implementation coming soon
    Ok(())
}

pub async fn add_dental_record(
    banks_client: &mut BanksClient,
    program_id: &Pubkey,
    payer: &Keypair,
    recent_blockhash: &Hash,
) -> Result<(), TransportError> {
    // Implementation coming soon
    Ok(())
}