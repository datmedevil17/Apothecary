use soroban_sdk::{contractclient, Address, Env};

#[contractclient(name = "PaymentContractClient")]
pub trait PaymentContract {
    fn initialize(env: Env);
    fn deposit(env: Env, addr: Address, amount: i128);
    fn transfer(env: Env, to: Address, amount: i128);
    fn withdraw(env: Env, addr: Address, amount: i128);
    fn get_balance(env: Env, addr: Address) -> i128;
}
