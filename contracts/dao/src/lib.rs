#![no_std]
use soroban_sdk::{
    contract, contractimpl, symbol_short, vec, Address, Bytes, Env, IntoVal, Map, Symbol, Vec,
};

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
        token_contract_id: Address,
    ) {
        let inst = env.storage().instance();
        inst.set(&Bytes::from_slice(&env, b"name"), &name);
        inst.set(&Bytes::from_slice(&env, b"description"), &description);
        inst.set(&Bytes::from_slice(&env, b"funding_goal"), &funding_goal);
        inst.set(&Bytes::from_slice(&env, b"creator"), &creator);
        inst.set(&Bytes::from_slice(&env, b"total_raised"), &0u64);
        inst.set(&Bytes::from_slice(&env, b"next_proposal_id"), &0u64);
        inst.set(
            &Bytes::from_slice(&env, b"investors"),
            &Vec::<Address>::new(&env),
        );
        inst.set(
            &Bytes::from_slice(&env, b"investments"),
            &Map::<Address, u64>::new(&env),
        );
        inst.set(
            &Bytes::from_slice(&env, b"proposal_details"),
            &Map::<u64, Symbol>::new(&env),
        );
        inst.set(
            &Bytes::from_slice(&env, b"proposal_votes"),
            &Map::<u64, i128>::new(&env),
        );
        inst.set(
            &Bytes::from_slice(&env, b"proposal_executed"),
            &Map::<u64, bool>::new(&env),
        );
        inst.set(
            &Bytes::from_slice(&env, b"token_contract_id"),
            &token_contract_id,
        );
        inst.set(
            &Bytes::from_slice(&env, b"distribution_history"),
            &Vec::<(u64, u64)>::new(&env),
        );
    }

    // Accept funds, record the investor, and mint profit‐share tokens
    pub fn invest(env: Env, investor: Address, amount: u64) {
        // Authenticate the investor
        investor.require_auth();

        // Check amount is positive
        assert!(amount > 0, "Investment amount must be positive");

        let inst = env.storage().instance();
        // 1. Update total_raised
        let mut total: u64 = inst.get(&Bytes::from_slice(&env, b"total_raised")).unwrap();
        total += amount;
        inst.set(&Bytes::from_slice(&env, b"total_raised"), &total);

        // 2. Record individual investment
        let mut invest_map: Map<Address, u64> =
            inst.get(&Bytes::from_slice(&env, b"investments")).unwrap();
        let prev: u64 = invest_map.get(investor.clone()).unwrap_or(0);
        invest_map.set(investor.clone(), prev + amount);
        inst.set(&Bytes::from_slice(&env, b"investments"), &invest_map);

        // 3. Track unique investors
        let mut invs: Vec<Address> = inst.get(&Bytes::from_slice(&env, b"investors")).unwrap();
        if !invs.contains(&investor) {
            invs.push_back(investor.clone());
            inst.set(&Bytes::from_slice(&env, b"investors"), &invs);
        }

        // 4. Mint profit‐share tokens
        let token_address: Address = inst
            .get(&Bytes::from_slice(&env, b"token_contract_id"))
            .unwrap();
        let token = ProfitTokenContractClient::new(&env, &token_address);
        token.mint(&investor, &(amount as i128));
    }

    // Create a new proposal, return its ID
    pub fn create_proposal(env: Env, details: Symbol) -> u64 {
        let inst = env.storage().instance();
        let mut pid: u64 = inst
            .get(&Bytes::from_slice(&env, b"next_proposal_id"))
            .unwrap();
        // Store proposal text
        let mut pm: Map<u64, Symbol> = inst
            .get(&Bytes::from_slice(&env, b"proposal_details"))
            .unwrap();
        pm.set(pid, details);
        inst.set(&Bytes::from_slice(&env, b"proposal_details"), &pm);
        // Initialize vote tally
        let mut vm: Map<u64, i128> = inst
            .get(&Bytes::from_slice(&env, b"proposal_votes"))
            .unwrap();
        vm.set(pid, 0i128);
        inst.set(&Bytes::from_slice(&env, b"proposal_votes"), &vm);
        // Mark as un‐executed
        let mut em: Map<u64, bool> = inst
            .get(&Bytes::from_slice(&env, b"proposal_executed"))
            .unwrap();
        em.set(pid, false);
        inst.set(&Bytes::from_slice(&env, b"proposal_executed"), &em);
        // Bump next ID
        pid += 1;
        inst.set(&Bytes::from_slice(&env, b"next_proposal_id"), &pid);
        pid - 1
    }

    // Vote yes/no weighted by token balance
    pub fn vote(env: Env, voter: Address, proposal_id: u64, support: bool) {
        // Authenticate the voter
        voter.require_auth();

        // Check proposal exists
        let inst = env.storage().instance();
        let pm: Map<u64, Symbol> = inst
            .get(&Bytes::from_slice(&env, b"proposal_details"))
            .unwrap();
        assert!(pm.contains_key(proposal_id), "Proposal does not exist");

        // Check proposal not executed
        let em: Map<u64, bool> = inst
            .get(&Bytes::from_slice(&env, b"proposal_executed"))
            .unwrap();
        assert!(!em.get(proposal_id).unwrap(), "Proposal already executed");

        // Fetch weight
        let token_address: Address = inst
            .get(&Bytes::from_slice(&env, b"token_contract_id"))
            .unwrap();

        let token = ProfitTokenContractClient::new(&env, &token_address);
        let w: i128 = token.balance(&voter);

        // Ensure voter has tokens
        assert!(w > 0, "Voter has no voting power");

        // Tally
        let mut vm: Map<u64, i128> = inst
            .get(&Bytes::from_slice(&env, b"proposal_votes"))
            .unwrap();
        let mut tally: i128 = vm.get(proposal_id).unwrap();
        tally += if support { w } else { -w };
        vm.set(proposal_id, tally);
        inst.set(&Bytes::from_slice(&env, b"proposal_votes"), &vm);
    }

    // Execute if tally > 0 and not yet executed
    pub fn execute_proposal(env: Env, proposal_id: u64) {
        let inst = env.storage().instance();

        // Check proposal exists
        let pm: Map<u64, Symbol> = inst
            .get(&Bytes::from_slice(&env, b"proposal_details"))
            .unwrap();
        assert!(pm.contains_key(proposal_id), "Proposal does not exist");

        let mut em: Map<u64, bool> = inst
            .get(&Bytes::from_slice(&env, b"proposal_executed"))
            .unwrap();
        assert!(!em.get(proposal_id).unwrap(), "Already executed");
        let vm: Map<u64, i128> = inst
            .get(&Bytes::from_slice(&env, b"proposal_votes"))
            .unwrap();
        let tally: i128 = vm.get(proposal_id).unwrap();
        assert!(tally > 0, "Not approved");

        // Simplified payout logic: send entire pot to creator
        let creator: Address = inst.get(&Bytes::from_slice(&env, b"creator")).unwrap();
        let pot: u64 = inst.get(&Bytes::from_slice(&env, b"total_raised")).unwrap();

        // Assume the payment contract already exists
        let payment_address = Address::from_str(&env, "Payment");

        // Properly convert arguments to Val
        let args = vec![&env, creator.into_val(&env), pot.into_val(&env)];

        // Invoke the transfer function and specify the return type as ()
        env.invoke_contract::<()>(&payment_address, &symbol_short!("transfer"), args);

        // Mark proposal as executed
        em.set(proposal_id, true);
        inst.set(&Bytes::from_slice(&env, b"proposal_executed"), &em);
    }

    // Record a profit distribution event
    pub fn record_distribution(env: Env, timestamp: u64, amount: u64) {
        // Ensure only the profit distribution contract can call this
        // This logic would need to be expanded based on your authorization model

        let inst = env.storage().instance();
        let mut history: Vec<(u64, u64)> = inst
            .get(&Bytes::from_slice(&env, b"distribution_history"))
            .unwrap();

        history.push_back((timestamp, amount));
        inst.set(&Bytes::from_slice(&env, b"distribution_history"), &history);
    }

    // Get distribution history
    pub fn get_distribution_history(env: Env) -> Vec<(u64, u64)> {
        env.storage()
            .instance()
            .get(&Bytes::from_slice(&env, b"distribution_history"))
            .unwrap()
    }

    // Get all proposals
    pub fn get_proposals_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&Bytes::from_slice(&env, b"next_proposal_id"))
            .unwrap()
    }

    // Get proposal details
    pub fn get_proposal_details(env: Env, proposal_id: u64) -> Symbol {
        let inst = env.storage().instance();
        let pm: Map<u64, Symbol> = inst
            .get(&Bytes::from_slice(&env, b"proposal_details"))
            .unwrap();

        assert!(pm.contains_key(proposal_id), "Proposal does not exist");
        pm.get(proposal_id).unwrap()
    }

    // Get proposal votes
    pub fn get_proposal_votes(env: Env, proposal_id: u64) -> i128 {
        let inst = env.storage().instance();
        let vm: Map<u64, i128> = inst
            .get(&Bytes::from_slice(&env, b"proposal_votes"))
            .unwrap();

        assert!(vm.contains_key(proposal_id), "Proposal does not exist");
        vm.get(proposal_id).unwrap()
    }

    // Get proposal execution status
    pub fn get_proposal_executed(env: Env, proposal_id: u64) -> bool {
        let inst = env.storage().instance();
        let em: Map<u64, bool> = inst
            .get(&Bytes::from_slice(&env, b"proposal_executed"))
            .unwrap();

        assert!(em.contains_key(proposal_id), "Proposal does not exist");
        em.get(proposal_id).unwrap()
    }

    // Get voting power for an address
    pub fn get_voting_power(env: Env, voter: Address) -> i128 {
        let inst = env.storage().instance();
        let token_address: Address = inst
            .get(&Bytes::from_slice(&env, b"token_contract_id"))
            .unwrap();

        let token = ProfitTokenContractClient::new(&env, &token_address);
        token.balance(&voter)
    }

    // Get total raised funds
    pub fn get_total_raised(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&Bytes::from_slice(&env, b"total_raised"))
            .unwrap()
    }

    // Check if funding goal has been reached
    pub fn is_funding_goal_reached(env: Env) -> bool {
        let inst = env.storage().instance();
        let total: u64 = inst.get(&Bytes::from_slice(&env, b"total_raised")).unwrap();
        let goal: u64 = inst.get(&Bytes::from_slice(&env, b"funding_goal")).unwrap();

        total >= goal
    }

    // Expose helpers for the distribution contract
    pub fn get_investors(env: Env) -> Vec<Address> {
        env.storage()
            .instance()
            .get(&Bytes::from_slice(&env, b"investors"))
            .unwrap()
    }

    pub fn get_token_contract(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&Bytes::from_slice(&env, b"token_contract_id"))
            .unwrap()
    }

    pub fn get_investments(env: Env) -> Map<Address, u64> {
        env.storage()
            .instance()
            .get(&Bytes::from_slice(&env, b"investments"))
            .unwrap()
    }

    // Expose getters for the factory contract
    pub fn get_name(env: Env) -> Symbol {
        env.storage()
            .instance()
            .get(&Bytes::from_slice(&env, b"name"))
            .unwrap()
    }

    pub fn get_description(env: Env) -> Symbol {
        env.storage()
            .instance()
            .get(&Bytes::from_slice(&env, b"description"))
            .unwrap()
    }

    pub fn get_funding_goal(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&Bytes::from_slice(&env, b"funding_goal"))
            .unwrap()
    }

    pub fn get_creator(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&Bytes::from_slice(&env, b"creator"))
            .unwrap()
    }
}
