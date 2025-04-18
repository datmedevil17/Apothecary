#![no_std]
use soroban_sdk::{
    contract, contractimpl, Env, Symbol, Address, BytesN, Vec, Map, Bytes,
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
        let next_id: u64 = inst.get(&Bytes::from_slice(&env, b"next_dao_id")).unwrap_or(0);
        
        // 2. Deploy a new ProfitToken for this DAO
        // Fix: register_contract_wasm needs two arguments - an optional Address and the WASM bytes
        let token_wasm_hash = env.register_contract_wasm(None, Bytes::from_slice(&env, b"profit_token"));
        let token_id: BytesN = env.deploy_contract(&token_wasm_hash);
        
        // 3. Deploy the DAO contract
        let dao_wasm_hash = env.register_contract_wasm(None, Bytes::from_slice(&env, b"dao"));
        let dao_contract_id: BytesN = env.deploy_contract(&dao_wasm_hash);
        let dao_addr = Address::Contract(dao_contract_id.clone());
        
        // 4. Initialize DAO with the token contract we just deployed
        let dao = DAOContractClient::new(&env, &dao_addr);
        dao.initialize(
            &name,
            &description,
            &funding_goal,
            &creator,
            &Address::Contract(token_id),  // Convert token_id to Address
        );
        
        // 5. Store mapping
        let mut map: Map<u64, Address> =
            inst.get(&Bytes::from_slice(&env, b"daos")).unwrap_or(Map::new(&env));
        map.set(next_id, dao_addr);
        inst.set(&Bytes::from_slice(&env, b"daos"), &map);
        
        // 6. Bump next_id
        inst.set(&Bytes::from_slice(&env, b"next_dao_id"), &(next_id + 1));
        
        (next_id, dao_addr)
    }
    // Fetch the on‐chain address of a DAO by its ID
    pub fn get_dao(env: Env, dao_id: u64) -> Address {
        let inst = env.storage().instance();
        let map: Map<u64, Address> =
            inst.get(&Bytes::from_slice(&env, b"daos")).unwrap_or(Map::new(&env));
        map.get(dao_id).unwrap_or(Address::default())
    }


    // Fetch the list of all DAOs
    pub fn get_all_daos(env: Env) -> Vec<Address> {
        let inst = env.storage().instance();
        let map: Map<u64, Address> =
            inst.get(&Bytes::from_slice(&env, b"daos")).unwrap_or(Map::new(&env));
        let mut daos: Vec<Address> = Vec::new(&env);
        for i in 0..map.len() {
            if let Some(dao) = map.get(i) {
                daos.push(dao);
            }
        }
        daos
    }
}