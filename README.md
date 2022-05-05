# Gamble_Game_Near


contract name : **Gamble**

contract info : **enable users to transfer near with the range to throw the dice, when 6 is hit, the user would get 6 times the prize as the reward**

<hr>
contract property: 

   -  **gamble_min_price** (Minimum price that should be transfered to the contract, revert otherwise)
   - **gamble_max_price** (Maximum price that should be transfered to the contract, revert otherwise)

<hr>
contract function:

   - **pub fn new() -> Self** : The new function should be called to initialize the contract, and set the gamble_max_price and the gamble_min_price

   - **pub fn get_minimal_gamble_price(&self) -> u128** : Get the Minimum amount of near to be transfered(Used for dapp, but usually won't as it's 0 all the time)

   - **pub fn get_maximum_gamble_price(&self) -> u128** :  Get the Minimum amount of near to be transfered(Used for dapp)
     
   - **pub fn get_balance(&self) -> u128** :  Get contract balance

   - **pub fn update_price(&mut self)** : Update price everytime the account balance changes

   - **pub fn sponsor(&mut self)** : The user could sponsor the contract(maybe only the owner will...)

   - **pub fn gamble(&mut self) -> u8** : The user could transfer near to get a chance to gamble, return the dice throwed by the user (Randomly generated)

   - **pub fn rand_dice(&self) -> u8** : Generate random number from 1 to 6

<hr>

deploy : 

```
    cargo build --target wasm32-unknown-unknown --release

    near deploy --wasmFile target/wasm32-unknown-unknown/release/gamble_game_near.wasm --accountId gamble_game1.young_cn.testnet
```

interact with contract:

```
# Initialize gamble contract
near call gamble_game1.young_cn.testnet new  --accountId gamble_game1.young_cn.testnet

# Get balance
near view gamble_game1.young_cn.testnet get_balance

# Get minimal gamble price
near view gamble_game1.young_cn.testnet get_minimal_gamble_price

# Sponsor the game
near call gamble_game1.young_cn.testnet sponsor --deposit 1  --accountId young_cn.testnet

# Play the gamble game
near call gamble_game1.young_cn.testnet gamble --deposit 1  --accountId young_cn.testnet
```

