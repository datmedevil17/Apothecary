#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Address, Env, Vec};
// Replace these imports with the actual client paths in your workspace
use dao::DAOContractClient;
use profit_token::ProfitTokenContractClient;
use soroban_sdk::IntoVal;
#[contract]
pub struct ProfitDistributionContract;

#[contractimpl]
impl ProfitDistributionContract {
    // Distribute `profit_amount` of native XLM to investors
    // dao_addr: the on‐chain address of a DAOContract instance
    pub fn distribute(env: Env, dao_addr: Address, profit_amount: u64) {
        // 1. Fetch investor list
        let dao = DAOContractClient::new(&env, &dao_addr);
        let investors: Vec<Address> = dao.get_investors();

        // 2. Locate the profit‐token contract for this DAO
        let token_id: Address = dao.get_token_contract();

        // 3. Compute total shares
        let token = ProfitTokenContractClient::new(&env, &token_id);
        let total_shares: i128 = token.total_supply();

        // 4. For each investor, compute and send payout
        for inv in investors.iter() {
            let bal: i128 = token.balance(&inv);
            // payout = profit_amount * bal / total_shares
            let payout = (profit_amount as i128 * bal) / total_shares;

            // send XLM using proper invoke_contract format
            // Note: Replace with the actual payment contract address
            let payment_address = Address::from_str(&env, "Payment");

            // Create vector of arguments for invoke_contract
            let args = vec![&env, inv.clone().into_val(&env), payout.into_val(&env)];

            // Call the transfer function with proper arguments
            env.invoke_contract::<()>(&payment_address, &symbol_short!("transfer"), args);
        }
    }
}
