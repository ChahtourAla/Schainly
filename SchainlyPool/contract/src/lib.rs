use std::collections::HashMap;
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128};//ValidAccountId
use near_sdk::{ext_contract, Promise, PromiseError,AccountId};
use near_sdk::{env, near_bindgen, Gas};
use serde::{Serialize,Deserialize};

#[ext_contract(ext_ft)]
pub trait Coin {
    fn ft_transfer(&mut self, receiver_id: String, amount: String, memo:String);
}

#[ext_contract(ext_factory)]
pub trait Factory {
    fn create_token(&mut self, args: TokenArgs);
}

#[ext_contract(ext_swap)]
pub trait Swap {
    fn ft_transfer_call(&mut self, receiver_id: String, amount:String, msg:String);
}

pub const TGAS: u64 = 1_000_000_000_000;
const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/png;base64,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&zyG2nwYLWXIDt4BrVKdv3zPE/tBB6ZN&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";

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenArgs {
    owner_id: AccountId,
    total_supply: U128,
    metadata: FungibleTokenMetadata,
}

// INVESTORS
// Vestors structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
pub struct Investor {
    pub investments: Vec<Investment>,
    pub tokens_aimed: Vec<Token>,
}

// Startup
// Startup structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
pub struct Startup {
    pub name: String,
    pub symbol: String,
}

// FUND_DEMAND
// FundDemand structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
pub struct FundDemand {
    pub description:String,
    pub amount: u128,
    pub end_time:u64,
    pub accepted:Option<bool>,
    pub refused:Option<bool>,
}

// INVESTMENT
// Investment structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
pub struct Investment {
    pub startup_name:String,
    pub amount: u128,
    pub time:u64,
}

// TOKEN
// Token structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
pub struct Token {
    pub startup_name:String,
    pub symbol: String,
    pub total_suply:u128,
    pub price:f64,
    pub amount_owned:u128,
}

// TOKEN
// Token structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
pub struct Buyer {
    pub startup_name:String,
    pub token_price: f64,
    pub amount:u128,
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Schainly {
    investors : HashMap<String,Investor>,
    startups : HashMap<String,Startup>,
    fund_demand : HashMap<String,FundDemand>,
    investments : HashMap<String,Investment>,
    buyers : HashMap<String,Buyer>,
}

// Define the default, which automatically initializes the contract
impl Default for Schainly {
    fn default() -> Self {
        panic!("Contract is not initialized yet")
    }
}

// Make sure that the caller of the function is the owner
// fn assert_self() {
//     assert_eq!(
//         env::current_account_id(),
//         env::predecessor_account_id(),
//         "Can only be called by owner"
//     );
// }

// Implement the contract structure
// To be implemented in the front end
#[near_bindgen]
impl Schainly {
    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
            investors : HashMap::new(),
            startups : HashMap::new(),
            fund_demand : HashMap::new(),
            investments : HashMap::new(),
            buyers : HashMap::new(),
        }
    }

    pub fn check_startup(&self,account:String) -> bool {
        self.startups.contains_key(&account)
    }

    pub fn check_investor(&self,account:String) -> bool {
        self.investors.contains_key(&account)
    }

    pub fn add_investor(&mut self,account:String) {
        let data = Investor{
            investments: Vec::new(),
            tokens_aimed: Vec::new(),
        };
        self.investors.insert(account, data);
    }

    pub fn add_startup(&mut self,account:String, name:String, symbol:String) {
        let data = Startup{
            name: name,
            symbol: symbol,
        };
        self.startups.insert(account, data);
    }

    pub fn add_fund_demand (&mut self,account:String,description:String,amount:u128) {
        assert!(self.check_startup(account.clone()),"You are not one of the startups");
        let data = FundDemand{
            description:description,
            amount: amount,
            end_time:env::block_timestamp() + 180000000,
            accepted:None,
            refused:None,
        };
        self.fund_demand.insert(account, data);
    }

    pub fn add_investment (&mut self,account:String,startup:String,amount:u128) {
        assert!(self.check_investor(account.clone()),"You are not one of the investors");
        let data = Investment {
            startup_name:startup,
            amount: amount,
            time:env::block_timestamp(),
        };
        self.investments.insert(account, data);
    }

    pub fn add_buyer (&mut self,account:String,startup:String,amount:u128){
        let mut existance = false;
        let mut token_price = 0.0 ;
        for i in self.investors.get(&account).unwrap().tokens_aimed.clone() {
            if i.startup_name == startup{
                existance = true;
                token_price = i.price;
                break;
            }
        }
        assert!(existance,"you dont have this token");
        let buyer = Buyer {
            startup_name: startup,
            token_price: token_price,
            amount:amount,
        };
        self.buyers.insert(account,buyer);

    }
    pub fn execute_investment (&mut self,startup:String) -> Promise{
        assert!(self.fund_demand.contains_key(&startup),"This startup doesn't have any demand");
        assert!(env::block_timestamp() > self.fund_demand.get(&startup).unwrap().end_time,"Demand time didn't finish yet !!");
        let mut investors = Vec::new();
        let mut total_invested_amount = 0;
        for i in self.investments.keys(){
            if self.investments.get(i).unwrap().startup_name == startup {
                investors.push(i.clone());
                total_invested_amount += self.investments.get(i).unwrap().amount;
            }
        }
        assert!(total_invested_amount > self.fund_demand.get(&startup).unwrap().amount,"demanded amount doesn't reach !!");
        let args = TokenArgs {
            owner_id: env::current_account_id(),
            total_supply: U128::from(self.fund_demand.get(&startup).unwrap().amount),
            metadata: FungibleTokenMetadata{
                spec: "ft-1.0.0".to_string(), 
                name: self.startups.get(&startup).unwrap().name.clone(), 
                symbol: self.startups.get(&startup).unwrap().symbol.clone(), 
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 8 
            },
        };
        let account: AccountId = "testschainly.testnet".to_string().try_into().unwrap(); 
        let p = ext_factory::ext(account)
        .with_static_gas(Gas(100 * TGAS))
        .create_token(args);
    

        return p.then( // Create a promise to callback staking_callback
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(10 * TGAS))
            .deploying_callback(investors,startup,total_invested_amount)
        )
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn deploying_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError>,investors:Vec<String>,startup:String,total_invested_amount:u128){
        // Check if the promise succeeded
        if call_result.is_err() {
        panic!("the deplying doesn't work successfuly");
        }

        for i in investors {
            let token = Token {
                startup_name:startup.clone(),
                symbol: self.startups.get(&startup).unwrap().symbol.clone(),
                total_suply:self.fund_demand.get(&startup).unwrap().amount,
                price:(total_invested_amount as f64) / (self.fund_demand.get(&startup).unwrap().amount as f64),
                amount_owned:((self.fund_demand.get(&startup).unwrap().amount) * ((self.investments.get(&i).unwrap().amount * 100)/total_invested_amount))/100 ,
            };
            let mut token_vect = self.investors.get(&i).unwrap().tokens_aimed.clone();
            token_vect.push(token);
            let mut investor = self.investors.get(&i).unwrap().clone();
            investor.tokens_aimed = token_vect;
            self.investors.insert(i, investor);
        }
    }


    // getters
    pub fn get_investors(self) -> HashMap<String,Investor> {
        self.investors
    }

    pub fn get_startups(self) -> HashMap<String,Startup> {
        self.startups
    }

    pub fn get_fund_demand(self) -> HashMap<String,FundDemand> {
        self.fund_demand
    }

    pub fn get_investments(self) -> HashMap<String,Investment>{
        self.investments
    }

    pub fn swap(&mut self, account:String) {
        let stable_token_account = "wrap.testnet".to_string().try_into().unwrap();
        ext_swap::ext(stable_token_account)
        .with_static_gas(Gas(5 * TGAS))
        .with_attached_deposit(1)
        .ft_transfer_call(account, "1".to_string(),"{\"force\":0,\"actions\":[{\"pool_id\":34,\"token_in\":\"wrap.testnet\",\"token_out\":\"usdt.fakes.testnet\",\"amount_in\":\"1000000000000000000000000\",\"min_amount_out\":\"0\"},{\"pool_id\":356,\"token_in\":\"usdt.fakes.testnet\",\"token_out\":\"usdn.testnet\",\"min_amount_out\":\"803352116740389500000\"}]}".to_string());
    }
    pub fn send_usdt (&mut self,account:String,amount:u128){
        let stable_token_account = "usdn.testnet".to_string().try_into().unwrap();
        ext_ft::ext(stable_token_account)
        .with_static_gas(Gas(5 * TGAS))
        .ft_transfer(account, amount.to_string(),"".to_string());
    }
}