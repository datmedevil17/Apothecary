#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, Symbol, Map, Bytes};

#[contract]
pub struct PaymentContract;

#[contractimpl]
impl PaymentContract {
    pub fn initialize(env: Env) {
        let inst = env.storage().instance();
        inst.set(&Bytes::from_slice(&env, b"balances"), &Map::<Address, i128>::new(&env));
    }

    pub fn deposit(env: Env, addr: Address, amount: i128) {
        addr.require_auth();
        assert!(amount > 0, "Amount must be positive");
        
        let mut inst = env.storage().instance();
        let mut balances: Map<Address, i128> = inst.get(&Bytes::from_slice(&env, b"balances")).unwrap_or(Map::new(&env));
        
        let current_balance = balances.get(addr.clone()).unwrap_or(0);
        balances.set(addr, current_balance + amount);
        
        inst.set(&Bytes::from_slice(&env, b"balances"), &balances);
    }

    pub fn transfer(env: Env, to: Address, amount: i128) {
        assert!(amount > 0, "Amount must be positive");
        
        // Get contract address (this will be the DAO contract in your case)
        let contract_id = env.current_contract_address();
        
        let mut inst = env.storage().instance();
        let mut balances: Map<Address, i128> = inst.get(&Bytes::from_slice(&env, b"balances")).unwrap_or(Map::new(&env));
        
        let current_contract_balance = balances.get(contract_id.clone()).unwrap_or(0);
        assert!(current_contract_balance >= amount, "Insufficient funds in contract");
        
        // Update sender's balance (the contract)
        balances.set(contract_id, current_contract_balance - amount);
        
        // Update recipient's balance
        let current_recipient_balance = balances.get(to.clone()).unwrap_or(0);
        balances.set(to, current_recipient_balance + amount);
        
        inst.set(&Bytes::from_slice(&env, b"balances"), &balances);
    }

    pub fn withdraw(env: Env, addr: Address, amount: i128) {
        addr.require_auth();
        assert!(amount > 0, "Amount must be positive");
        
        let mut inst = env.storage().instance();
        let mut balances: Map<Address, i128> = inst.get(&Bytes::from_slice(&env, b"balances")).unwrap_or(Map::new(&env));
        
        let current_balance = balances.get(addr.clone()).unwrap_or(0);
        assert!(current_balance >= amount, "Insufficient funds");
        
        balances.set(addr, current_balance - amount);
        inst.set(&Bytes::from_slice(&env, b"balances"), &balances);
    }

    pub fn get_balance(env: Env, addr: Address) -> i128 {
        let inst = env.storage().instance();
        let balances: Map<Address, i128> = inst.get(&Bytes::from_slice(&env, b"balances")).unwrap_or(Map::new(&env));
        balances.get(addr).unwrap_or(0)
    }
}