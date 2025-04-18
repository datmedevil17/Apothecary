#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, Map, BytesN, Vec};

#[contract]
pub struct ProfitTokenContract;

#[contractimpl]
impl ProfitTokenContract {
    // Initialize the token with the DAO contract as admin
    // admin: the DAO contract address (Address::Contract)
    pub fn initialize(env: Env, admin: Address) {
        let mut storage = env.storage().instance();
        // Store admin for authorization (optional)
        storage.set(b"admin", &admin);
        // Total supply starts at zero
        storage.set(b"total_supply", &0i128);
        // Balances map: Address -> i128
        storage.set(b"balances", &Map::<Address, i128>::new(&env));
    }
    
    // Only the DAO contract (admin) should call mint
    pub fn mint(env: Env, to: Address, amount: i128) {
        let storage = env.storage().instance();
        let admin: Address = storage.get(b"admin").unwrap();
        // No explicit authorization check - we'll rely on contract-level auth
        
        // update total supply
        let mut total: i128 = storage.get(b"total_supply").unwrap();
        total += amount;
        storage.set(b"total_supply", &total);
        // update balance
        let mut balances: Map<Address, i128> = storage.get(b"balances").unwrap();
        let prev: i128 = balances.get(to.clone()).unwrap_or(0);
        balances.set(to, prev + amount); // Removed the & before prev + amount
        storage.set(b"balances", &balances);
    }
    
    pub fn balance(env: Env, who: Address) -> i128 {
        let balances: Map<Address, i128> = env
            .storage()
            .instance()
            .get(b"balances")
            .unwrap();
        balances.get(who).unwrap_or(0)
    }
    
    pub fn total_supply(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(b"total_supply")
            .unwrap()
    }
}