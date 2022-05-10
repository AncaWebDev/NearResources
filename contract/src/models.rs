use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::serde::{Deserialize, Deserializer , Serializer, Serialize};

#[allow(unused_imports)]

use crate::utils::{
    AccountId,
    Timestamp
};


#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]

pub struct Resource {
    id: i32,
  pub creator: AccountId,
    created_at: Timestamp,
    url: String,
    budget: u128,
   pub total_votes: i64,
    description: String,
    resource_type: String,
   pub votes: Vec<String>
}


impl Resource {
    pub fn new(id:i32, url: String, resource_type: String, budget: u128, description: String) -> Self {
        
        Resource {
            id,
            creator: env::signer_account_id().to_string(),
            created_at: env::block_timestamp(),
            url,
            budget,
            total_votes : 0,
            description,
            resource_type,
            votes: vec![],
        }
    }
}