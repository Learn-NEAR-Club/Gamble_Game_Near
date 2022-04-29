use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    json_types::U128,
    near_bindgen, PanicOnDefault, AccountId, Balance,
    env, require, log, ext_contract, Gas,
};

const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;
const TGAS: u64 = 1_000_000_000_000;

#[ext_contract(ext_ft)]
trait ExtFT{
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo:Option<String>);
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Gamble {
    owner : AccountId,

    factor : U128,

    gamble_min_price : Balance,

    gamble_max_price : Balance,
}


#[near_bindgen]
impl Gamble {

    #[init]
    pub fn new(
            owner_id : AccountId,
            factor : U128
        ) -> Self {
        Self{
            owner_id,
            factor,
            gamble_min_price.0,
            gamble_max_price.0,
        }
        
    }

    #[private]
    pub fn updatePrice(&mut self){
        let account_balance = env::account_balance();
        self.gamble_max_price = account_balance / (2 * self.factor);
        log!("we have {} uints in total, be sure not to exceed the max gamble price limit {} to get {}X", account_balance, self.gamble_max_price, self.factor);
    }

    #[payable]
    pub fn start(&mut self){
        let sponsor_id = env::signer_account_id();
        require!(sponsor_id == owner, "The gamble game should be started by the contract owner");

        let deposit = env::attached_deposit();
        log!("owner {} has add {} to the game to increase blance", sponsor_id, deposit);
        updatePrice();
    }
}
