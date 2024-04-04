use borsh::{BorshDeserialize, BorshSerialize};
use serde::Deserialize;

use crate::types::U8Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct SbApiFeedParams {
    pub program_id: U8Pubkey,
    pub deal_pk: U8Pubkey,
    pub url: String,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct ApiFeedData {
    pub reach: u64,
}

#[derive(Deserialize)]
pub struct ContentData {
    pub views: u64,
}
