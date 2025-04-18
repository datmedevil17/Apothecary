#![no_std]
use dao::DAOContractClient;
use soroban_sdk::{contract, contractimpl, Address, Bytes, BytesN, Env, Map, Symbol, Vec};

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
        let inst = env.storage().instance();
        // 1. Fetch & bump DAO ID
        let next_id: u64 = inst
            .get(&Bytes::from_slice(&env, b"next_dao_id"))
            .unwrap_or(0);

        // 2. Deploy a new ProfitToken for this DAO
        // First upload the WASM
        let token_wasm_hash: BytesN<32> = env
            .deployer()
            .upload_contract_wasm(Bytes::from_slice(&env, b"profit_token"));

        // Deploy the token contract using with_current_contract and deploy_v2
        let salt_token = BytesN::from_array(&env, &[0; 32]); // Use a zero salt
        let token_address = env
            .deployer()
            .with_current_contract(salt_token)
            .deploy_v2(token_wasm_hash, ()); // Empty tuple for no constructor args

        // 3. Deploy the DAO contract
        let dao_wasm_hash: BytesN<32> = env
            .deployer()
            .upload_contract_wasm(Bytes::from_slice(&env, b"dao"));

        // Deploy the DAO contract with a different salt
        let salt_dao = BytesN::from_array(&env, &[1; 32]); // Use a different salt
        let dao_address = env
            .deployer()
            .with_current_contract(salt_dao)
            .deploy_v2(dao_wasm_hash, ()); // Empty tuple for no constructor args

        // 4. Initialize DAO with the token contract
        // We need to update our DAO contract to accept Address instead of BytesN<32>
        let dao = DAOContractClient::new(&env, &dao_address.clone());
        dao.initialize_dao(
            &name,
            &description,
            &funding_goal,
            &creator,
            &token_address, // Pass the Address directly
        );

        // 5. Store mapping
        let mut map: Map<u64, Address> = inst
            .get(&Bytes::from_slice(&env, b"daos"))
            .unwrap_or(Map::new(&env));
        map.set(next_id, dao_address.clone());
        inst.set(&Bytes::from_slice(&env, b"daos"), &map);

        // 6. Bump next_id
        inst.set(&Bytes::from_slice(&env, b"next_dao_id"), &(next_id + 1));

        (next_id, dao_address)
    }

    // Fetch the on‐chain address of a DAO by its ID
    pub fn get_dao(env: Env, dao_id: u64) -> Option<Address> {
        let inst = env.storage().instance();
        let map: Map<u64, Address> = inst
            .get(&Bytes::from_slice(&env, b"daos"))
            .unwrap_or(Map::new(&env));

        map.get(dao_id)
    }

    // Get the total number of DAOs
    pub fn get_dao_count(env: Env) -> u64 {
        let inst = env.storage().instance();
        inst.get(&Bytes::from_slice(&env, b"next_dao_id"))
            .unwrap_or(0)
    }

    // Get DAOs with pagination
    pub fn get_daos_paginated(env: Env, start: u64, limit: u64) -> Vec<(u64, Address)> {
        let inst = env.storage().instance();
        let map: Map<u64, Address> = inst
            .get(&Bytes::from_slice(&env, b"daos"))
            .unwrap_or(Map::new(&env));

        let total = inst
            .get(&Bytes::from_slice(&env, b"next_dao_id"))
            .unwrap_or(0);
        let mut result: Vec<(u64, Address)> = Vec::new(&env);

        let end = if start + limit > total {
            total
        } else {
            start + limit
        };

        for i in start..end {
            if let Some(dao) = map.get(i) {
                result.push_back((i, dao));
            }
        }

        result
    }

    // Fetch all DAOs created by a specific address
    pub fn get_daos_by_creator(env: Env, creator: Address) -> Vec<(u64, Address)> {
        let inst = env.storage().instance();
        let map: Map<u64, Address> = inst
            .get(&Bytes::from_slice(&env, b"daos"))
            .unwrap_or(Map::new(&env));

        let total = inst
            .get(&Bytes::from_slice(&env, b"next_dao_id"))
            .unwrap_or(0);
        let mut result: Vec<(u64, Address)> = Vec::new(&env);

        for i in 0..total {
            if let Some(dao_address) = map.get(i) {
                // Check if this DAO was created by the specified creator
                let dao = DAOContractClient::new(&env, &dao_address);
                if dao.get_creator() == creator {
                    result.push_back((i, dao_address));
                }
            }
        }

        result
    }

    // Fetch the list of all DAOs
    pub fn get_all_daos(env: Env) -> Vec<Address> {
        let inst = env.storage().instance();
        let map: Map<u64, Address> = inst
            .get(&Bytes::from_slice(&env, b"daos"))
            .unwrap_or(Map::new(&env));

        let mut daos: Vec<Address> = Vec::new(&env);
        for i in 0..map.len() {
            // Convert u32 to u64
            if let Some(dao) = map.get(i.into()) {
                // Use push_back not push
                daos.push_back(dao);
            }
        }
        daos
    }
}
