use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    json_types::U128,
    near_bindgen, Balance,
    env, require, log, ext_contract, Promise, 
};
use rand::Rng;

const TAX : f32 = 0.95;
const FACTOR: u128 = 6;

#[ext_contract(ext_ft)]
trait ExtFT{
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo:Option<String>);
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Gamble {

    gamble_min_price : Balance,

    gamble_max_price : Balance,

}

impl Default for Gamble {
    fn default() -> Self {
        Self {
            gamble_max_price : 0,
            gamble_min_price : 0,
        }
    }
}

#[near_bindgen]
impl Gamble {

    #[init]
    pub fn new() -> Self {
        Self{
            gamble_max_price : 0,
            gamble_min_price : 0,
        }
    }

    pub fn get_minimal_gamble_price(&self) -> u128 {
        self.gamble_min_price
    }

    pub fn get_maximum_gamble_price(&self) -> u128 {
        self.gamble_max_price
    }    

    pub fn get_balance(&self) -> u128 {
        env::account_balance()
    }

    #[private]
    pub fn update_price(&mut self){
        let account_balance = env::account_balance();
        self.gamble_max_price = account_balance / (5 * FACTOR);
        log!("we have {} uints in total, be sure not to exceed the max gamble price limit {} to get {}X", account_balance, self.gamble_max_price, FACTOR);
    }


    #[payable]
    pub fn sponsor(&mut self){
        let sponsor_id = env::signer_account_id();
        let deposit = env::attached_deposit();
        log!("sponsor {} has add {} to the game to increase balance, thank you ~", sponsor_id, deposit);
        self.update_price();
    }

    #[payable]
    pub fn gamble(&mut self) -> u128{
        let gambler_id = env::signer_account_id();
        let deposit = env::attached_deposit();

        require!(deposit>=self.gamble_min_price,"The gamble price must exceed gamble_min_price");
        require!(deposit<=self.gamble_max_price,"The gamble price must not exceed gamble_max_price");
        
        let num = self.rand();

        if num == FACTOR {
            let amount = (deposit as f32 ) *(FACTOR as f32) * TAX;
            let amount_u128 = amount  as u128;
            log!("Congratuations to {}, he has won the gamble, the prize is {}",gambler_id,deposit);
            Promise::new(gambler_id).transfer(amount_u128);
        }
        self.update_price();
        return num;
    }

    pub fn rand(&self) -> u128 {
        rand::thread_rng().gen_range(1..FACTOR+1)    
    }

}


#[cfg(not(target_arch="wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env,VMContext};
    use near_sdk::Gas;
    use near_sdk::AccountId;

    fn get_context(input: Vec<u8>) -> VMContext {
        VMContext {
            current_account_id: AccountId::new_unchecked("alice.testnet".to_string()),
            signer_account_id: AccountId::new_unchecked("robert.testnet".to_string()),
            signer_account_pk: vec![0u8; 33].try_into().unwrap(),
            predecessor_account_id: AccountId::new_unchecked("jane.testnet".to_string()),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: Gas(10u64.pow(18)),
            random_seed: [0u8; 32],
            view_config: None,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn rand_test() {
        let context = get_context(vec![]);
        testing_env!(context);
        let contract = Gamble{
            gamble_min_price : 0,
            gamble_max_price : 0,
        };
        for _ in 0..100{
            let val = contract.rand();
            // println!("{}",val);
            assert_eq!(val>=1,true,"The random value should not be smaller than 1");
            assert_eq!(val<=6,true,"The random value should not be bigger than 6");
        }
    }

    #[test]
    fn gamble_test() {
        let context = get_context(vec![]);
        testing_env!(context);
        let mut contract = Gamble{
            gamble_min_price : 0,
            gamble_max_price : 0,
        };
        println!("minimal : {}",contract.get_minimal_gamble_price());
        println!("maximum : {}",contract.get_maximum_gamble_price());
        println!("balance : {}",contract.get_balance());
        println!("gamble: {}",contract.gamble());


    }
}



