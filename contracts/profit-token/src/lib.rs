#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map};

#[contract]
pub struct ProfitTokenContract;

#[contractimpl]
impl ProfitTokenContract {
    // Initialize the token with the DAO contract as admin
    // admin: the DAO contract address (Address::Contract)
    pub fn initialize_ptc(env: Env, admin: Address) {
        let storage = env.storage().instance();
        // Store admin for authorization (optional)
        storage.set(b"admin", &admin);
        // Total supply starts at zero
        storage.set(b"total_supply", &0i128);
        // Balances map: Address -> i128
        storage.set(b"balances", &Map::<Address, i128>::new(&env));
        // Allowances map: (Owner, Spender) -> i128
        storage.set(b"allowances", &Map::<(Address, Address), i128>::new(&env));
    }

    // Get admin address
    pub fn get_admin(env: Env) -> Address {
        let storage = env.storage().instance();
        storage.get(b"admin").unwrap()
    }

    // Only the DAO contract (admin) should call mint
    pub fn mint(env: Env, to: Address, amount: i128) {
        let storage = env.storage().instance();

        // Check authorization - Only admin can mint
        let admin: Address = storage.get(b"admin").unwrap();
        admin.require_auth();

        // Amount must be positive
        assert!(amount > 0, "Amount must be positive");

        // update total supply
        let mut total: i128 = storage.get(b"total_supply").unwrap();
        total += amount;
        storage.set(b"total_supply", &total);
        // update balance
        let mut balances: Map<Address, i128> = storage.get(b"balances").unwrap();
        let prev: i128 = balances.get(to.clone()).unwrap_or(0);
        balances.set(to, prev + amount);
        storage.set(b"balances", &balances);
    }

    // Burn tokens
    pub fn burn(env: Env, from: Address, amount: i128) {
        let storage = env.storage().instance();

        // Require authorization from the token owner
        from.require_auth();

        // Amount must be positive
        assert!(amount > 0, "Amount must be positive");

        // update balance
        let mut balances: Map<Address, i128> = storage.get(b"balances").unwrap();
        let prev: i128 = balances.get(from.clone()).unwrap_or(0);
        assert!(prev >= amount, "Insufficient balance");

        balances.set(from, prev - amount);
        storage.set(b"balances", &balances);

        // update total supply
        let mut total: i128 = storage.get(b"total_supply").unwrap();
        total -= amount;
        storage.set(b"total_supply", &total);
    }

    // Transfer tokens to another address
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        // Authenticate the caller
        from.require_auth();

        // Amount must be positive
        assert!(amount > 0, "Amount must be positive");

        let storage = env.storage().instance();
        let mut balances: Map<Address, i128> = storage.get(b"balances").unwrap();

        // Check sender has enough balance
        let from_balance = balances.get(from.clone()).unwrap_or(0);
        assert!(from_balance >= amount, "Insufficient balance");

        // Update balances
        balances.set(from.clone(), from_balance - amount);
        let to_balance = balances.get(to.clone()).unwrap_or(0);
        balances.set(to, to_balance + amount);

        storage.set(b"balances", &balances);
    }

    // Approve spender to transfer tokens on behalf of owner
    pub fn approve(env: Env, owner: Address, spender: Address, amount: i128) {
        // Authenticate the caller
        owner.require_auth();

        // Amount must be non-negative
        assert!(amount >= 0, "Amount must be non-negative");

        let storage = env.storage().instance();
        let mut allowances: Map<(Address, Address), i128> = storage.get(b"allowances").unwrap();

        // Set allowance
        allowances.set((owner, spender), amount);
        storage.set(b"allowances", &allowances);
    }

    // Get allowance
    pub fn allowance(env: Env, owner: Address, spender: Address) -> i128 {
        let storage = env.storage().instance();
        let allowances: Map<(Address, Address), i128> = storage.get(b"allowances").unwrap();
        allowances.get((owner, spender)).unwrap_or(0)
    }

    // Transfer tokens on behalf of owner (requires approval)
    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        // Authenticate the caller
        spender.require_auth();

        // Amount must be positive
        assert!(amount > 0, "Amount must be positive");

        let storage = env.storage().instance();

        // Check allowance
        let mut allowances: Map<(Address, Address), i128> = storage.get(b"allowances").unwrap();
        let allowance = allowances.get((from.clone(), spender.clone())).unwrap_or(0);
        assert!(allowance >= amount, "Insufficient allowance");

        // Update allowance
        allowances.set((from.clone(), spender), allowance - amount);
        storage.set(b"allowances", &allowances);

        // Update balances
        let mut balances: Map<Address, i128> = storage.get(b"balances").unwrap();
        let from_balance = balances.get(from.clone()).unwrap_or(0);
        assert!(from_balance >= amount, "Insufficient balance");

        balances.set(from, from_balance - amount);
        let to_balance = balances.get(to.clone()).unwrap_or(0);
        balances.set(to, to_balance + amount);

        storage.set(b"balances", &balances);
    }

    pub fn balance(env: Env, who: Address) -> i128 {
        let balances: Map<Address, i128> = env.storage().instance().get(b"balances").unwrap();
        balances.get(who).unwrap_or(0)
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(b"total_supply").unwrap()
    }
}
