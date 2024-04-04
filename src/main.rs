mod errors;
mod models;
mod types;

use std::sync::Arc;

use ::borsh::{self, BorshDeserialize};
use errors::CustomError;
use reqwest;
use tokio;

use models::{ApiFeedData, ContentData, SbApiFeedParams};
use switchboard_solana::{get_ixn_discriminator, prelude::*, switchboard_function};

#[switchboard_function]
pub async fn sb_api_feed_function(
    runner: Arc<FunctionRunner>,
    params: Vec<u8>,
) -> Result<Vec<Instruction>, SbFunctionError> {
    let params: SbApiFeedParams =
        SbApiFeedParams::try_from_slice(&params).map_err(|_| CustomError::ParseError)?;

    let res = reqwest::get(params.url)
        .await
        .map_err(|_| CustomError::FetchError)?;
    let content_data: ContentData =
        serde_json::from_str(&res.text().await.unwrap()).map_err(|_| CustomError::ParseError)?;

    let api_feed_data = ApiFeedData {
        reach: content_data.views,
    };

    let mut ix_data = get_ixn_discriminator("scheduled_callback").to_vec();
    let mut serialized_feed_data =
        borsh::to_vec(&api_feed_data).map_err(|_| CustomError::ParseError)?;
    ix_data.append(&mut serialized_feed_data);

    let ix = Instruction {
        program_id: Pubkey::new_from_array(params.program_id),
        accounts: vec![
            AccountMeta::new(Pubkey::new_from_array(params.deal_pk), false),
            AccountMeta::new_readonly(runner.signer, true),
        ],
        data: ix_data,
    };

    Ok(vec![ix])
}

// #[cfg(test)]
// mod tests {
//     use switchboard_solana::Pubkey;

//     use crate::{get_data_from_url, models::SbApiFeedParams};

//     #[test]
//     fn test_get_data_from_url() {
//         let params = SbApiFeedParams {
//             deal_pk: Pubkey::new_unique().to_bytes(),
//             program_id: Pubkey::new_unique().to_bytes(),
//             url: String::from("https://jsonplaceholder.typicode.com/todos/1"),
//         };

//         let data = get_data_from_url(params.url).unwrap();
//         assert!(data.reach == 2458);
//     }
// }
