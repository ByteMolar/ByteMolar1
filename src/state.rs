use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DentalRecord {
    pub patient_pubkey: Pubkey,
    pub dentist_pubkey: Pubkey,
    pub timestamp: i64,
    pub diagnosis: String,
    pub treatment_plan: String,
    pub is_completed: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Clinic {
    pub authority: Pubkey,
    pub name: String,
    pub total_patients: u64,
    pub active_treatments: u64,
}