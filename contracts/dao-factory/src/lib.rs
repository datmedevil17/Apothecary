#![no_std]
use soroban_sdk::{
    contract, contractimpl, Env, Symbol, Address, BytesN, Vec, Map,
};
use dao::DAOContractClient;

#[contract]
pub struct DAOFactoryContract;

#[contractimpl]
impl DAOFactoryContract {
    // Create a fresh DAO; returns its numeric ID and its on‐chain Address
    pub fn create_dao(
        env: Env,
        name: Symbol,
        description: Symbol,
        funding_goal: u64,
        creator: Address,
    ) -> (u64, Address) {
        let mut inst = env.storage().instance();
        // 1. Fetch & bump DAO ID
        let mut next_id: u64 = inst.get(b"next_dao_id").unwrap_or(0);
        // 2. Deploy a new ProfitToken for this DAO
        let token_wasm = env.register_contract_wasm("profit_token");
        let token_id: BytesN = env.deploy_contract(token_wasm);
        // Initialize token, setting admin = DAO contract address (unknown until we deploy DAO below)
        // 3. Deploy the DAO contract
        let dao_wasm = env.register_contract_wasm("dao");
        let dao_contract_id: BytesN = env.deploy_contract(dao_wasm);
        let dao_addr = Address::Contract(dao_contract_id.clone());
        // 4. Initialize DAO with the token contract we just deployed
        let dao = DAOContractClient::new(&env, &dao_contract_id);
        dao.initialize(
            &name,
            &description,
            &funding_goal,
            &creator,
            &token_id,
        );
        // 5. Store mapping
        let mut map: Map<u64, Address> =
            inst.get(b"daos").unwrap_or(Map::new(&env));
        map.set(next_id, &dao_addr);
        inst.set(b"daos", &map);
        // 6. Bump next_id
        inst.set(b"next_dao_id", &(next_id + 1));
        (next_id, dao_addr)
    }
}