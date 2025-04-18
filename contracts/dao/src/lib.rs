#![no_std]
use soroban_sdk::{
    contract, contractimpl, Env, Address, Symbol, Vec, Map, BytesN,
};
use soroban_sdk::types::U128;

// Import the profit‐token client
use profit_token::ProfitTokenContractClient;

#[contract]
pub struct DAOContract;

#[contractimpl]
impl DAOContract {
    // Called by the factory immediately after instantiation
    pub fn initialize(
        env: Env,
        name: Symbol,
        description: Symbol,
        funding_goal: u64,
        creator: Address,
        token_contract_id: BytesN,
    ) {
        let mut inst = env.storage().instance();
        inst.set(b"name", &name);
        inst.set(b"description", &description);
        inst.set(b"funding_goal", &funding_goal);
        inst.set(b"creator", &creator);
        inst.set(b"total_raised", &0u64);
        inst.set(b"next_proposal_id", &0u64);
        inst.set(b"investors", &Vec::<Address>::new(&env));
        inst.set(b"investments", &Map::<Address, u64>::new(&env));
        inst.set(b"proposal_details", &Map::<u64, Symbol>::new(&env));
        inst.set(b"proposal_votes", &Map::<u64, i128>::new(&env));
        inst.set(b"proposal_executed", &Map::<u64, bool>::new(&env));
        inst.set(b"token_contract_id", &token_contract_id);
    }

    // Accept funds, record the investor, and mint profit‐share tokens
    pub fn invest(env: Env, investor: Address, amount: u64) {
        let mut inst = env.storage().instance();
        // 1. Update total_raised
        let mut total: u64 = inst.get(b"total_raised").unwrap();
        total += amount;
        inst.set(b"total_raised", &total);

        // 2. Record individual investment
        let mut invest_map: Map<Address, u64> =
            inst.get(b"investments").unwrap();
        let prev: u64 = invest_map.get(investor.clone()).unwrap_or(0);
        invest_map.set(investor.clone(), &(prev + amount));
        inst.set(b"investments", &invest_map);

        // 3. Track unique investors
        let mut invs: Vec<Address> = inst.get(b"investors").unwrap();
        if !invs.contains(&investor) {
            invs.push_back(investor.clone());
            inst.set(b"investors", &invs);
        }

        // 4. Mint profit‐share tokens
        let token_id: BytesN = inst.get(b"token_contract_id").unwrap();
        let token = ProfitTokenContractClient::new(&env, &token_id);
        token.mint(&investor, &U128::from(amount));
    }

    // Create a new proposal, return its ID
    pub fn create_proposal(env: Env, details: Symbol) -> u64 {
        let mut inst = env.storage().instance();
        let mut pid: u64 = inst.get(b"next_proposal_id").unwrap();
        // Store proposal text
        let mut pm: Map<u64, Symbol> =
            inst.get(b"proposal_details").unwrap();
        pm.set(pid, &details);
        inst.set(b"proposal_details", &pm);
        // Initialize vote tally
        let mut vm: Map<u64, i128> = inst.get(b"proposal_votes").unwrap();
        vm.set(pid, &0);
        inst.set(b"proposal_votes", &vm);
        // Mark as un‐executed
        let mut em: Map<u64, bool> =
            inst.get(b"proposal_executed").unwrap();
        em.set(pid, &false);
        inst.set(b"proposal_executed", &em);
        // Bump next ID
        pid += 1;
        inst.set(b"next_proposal_id", &pid);
        pid - 1
    }

    // Vote yes/no weighted by token balance
    pub fn vote(
        env: Env,
        voter: Address,
        proposal_id: u64,
        support: bool,
    ) {
        let inst = env.storage().instance();
        // Fetch weight
        let token_id: BytesN = inst.get(b"token_contract_id").unwrap();
        let token = ProfitTokenContractClient::new(&env, &token_id);
        let w: i128 = token.balance(voter.clone()).try_into().unwrap();
        // Tally
        let mut vm: Map<u64, i128> = inst.get(b"proposal_votes").unwrap();
        let mut tally: i128 = vm.get(proposal_id).unwrap();
        tally += if support { w } else { -w };
        vm.set(proposal_id, &tally);
        inst.set(b"proposal_votes", &vm);
    }

    // Execute if tally > 0 and not yet executed
    pub fn execute_proposal(env: Env, proposal_id: u64) {
        let mut inst = env.storage().instance();
        let mut em: Map<u64, bool> =
            inst.get(b"proposal_executed").unwrap();
        assert!(!em.get(proposal_id).unwrap(), "Already executed");
        let vm: Map<u64, i128> = inst.get(b"proposal_votes").unwrap();
        let tally: i128 = vm.get(proposal_id).unwrap();
        assert!(tally > 0, "Not approved");
        // Simplified payout logic: send entire pot to creator
        let creator: Address = inst.get(b"creator").unwrap();
        let pot: u64 = inst.get(b"total_raised").unwrap();
        env.invoke_contract::<()>(
            &env.host_function().with_name("Payment"),
            (&creator, &pot),
        );
        em.set(proposal_id, &true);
        inst.set(b"proposal_executed", &em);
    }

    // Expose helpers for the distribution contract
    pub fn get_investors(env: Env) -> Vec<Address> {
        env.storage().instance().get(b"investors").unwrap()
    }
    pub fn get_token_contract(env: Env) -> BytesN {
        env.storage().instance().get(b"token_contract_id").unwrap()
    }
}