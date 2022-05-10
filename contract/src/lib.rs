mod models;
mod utils;

use std::convert::TryInto;

use crate::{
    utils::{
        AccountId,
    },
    models::{
        Resource
    }
};

use near_sdk::{borsh::{self, BorshDeserialize, BorshSerialize}};
#[allow(unused_imports)]
use near_sdk::{env, PromiseIndex, near_bindgen};
near_sdk::setup_alloc!();


#[near_bindgen]
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]


pub struct Contract{
    owner: AccountId,
    resources: Vec<Resource>,
}

#[near_bindgen]
impl Contract{
    #[init]
    pub fn new(owner: AccountId) -> Self {
        let resources: Vec<Resource> = Vec::new();
        Contract{owner, resources}
    }

    pub fn add_resource(&mut self, url: String, resource_type: String, budget: u128, description: String){

        let id = self.resources.len() as i32;

        self.resources.push(Resource::new(
            id,
            url,
            resource_type,
            budget,
            description
        ));

        env::log("Added a new link! Thanks for your contribution!".as_bytes());
    }

    pub fn list_resources(&self) -> Vec<Resource> {
        let resources = &self.resources;

        resources.to_vec()
    }

    pub fn count_resources(&mut self) -> usize {
        self.resources.len()
    }

    pub fn vote(&mut self, id:usize){
        let resource: &mut Resource = self.resources.get_mut(id).unwrap();

        let voting_account = env::predecessor_account_id();
        resource.votes.push(voting_account.to_string());

        resource.total_votes = resource.total_votes + 1;
        env::log("Vote counted ! Thank you for voting!".as_bytes());
    }

    pub fn get_number_of_votes(&mut self, id:usize) -> u64 {
        let resource: &mut Resource = self.resources.get_mut(id).unwrap();
        resource.total_votes.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId};

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn add_link() {

        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());

        testing_env!(context.build());

        let mut contract = Contract::new(alice.to_string());

        contract.add_resource("https://learn.figment.io/protocols/near".to_string(), "Tutorials".to_string(), 5, "Beginner tutorials - writing smart contracts for NFTs and more".to_string());

        let result = contract.count_resources();

        assert_eq!(result, 1);
    }


    #[test] 
    fn add_voter() {

        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());

        testing_env!(context.build());

        let mut contract = Contract::new(alice.to_string());

        contract.add_resource("https://learn.figment.io/protocols/near".to_string(), "Tutorials".to_string(), 5, "Beginner tutorials - writing smart contracts for NFTs and more".to_string());


        contract.vote(0);

        let result = contract.get_number_of_votes(0);

        assert_eq!(result, 1);

    }
}





//   contract.add_resource("https://learn.figment.io/protocols/near".to_string(): String, "Tutorials".to_string(): String, 5: u128, "Beginner tutorials - writing smart contracts for NFTs and more".to_string(): String);
