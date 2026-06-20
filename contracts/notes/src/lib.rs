#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, String, Vec, token,
    vec,
};

// ============================================================
// DATA TYPES
// ============================================================

#[contracttype]
#[derive(Clone)]
pub struct Payment {
    pub resource: String,   // e.g. "/api/crypto-price"
    pub amount: i128,       // in stroops (1 USDC = 10_000_000)
    pub recipient: Address,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct AgentConfig {
    pub owner: Address,
    pub spending_limit: i128,   // max per single payment
    pub daily_limit: i128,      // max per day total
    pub is_active: bool,
}

#[contracttype]
pub enum DataKey {
    Config,
    Balance,
    PaymentCount,
    Payment(u32),
    DailySpent,
    LastResetDay,
    TokenAddr,
}

// ============================================================
// CONTRACT
// ============================================================

#[contract]
pub struct AgentPayContract;

#[contractimpl]
impl AgentPayContract {

    // ----------------------------------------------------------
    // INITIALIZE — called once by the agent owner
    // ----------------------------------------------------------
    pub fn initialize(
        env: Env,
        owner: Address,
        token_addr: Address,
        spending_limit: i128,   // max USDC per single tx (e.g. 1_000_000 = 0.1 USDC)
        daily_limit: i128,      // max USDC per day (e.g. 10_000_000 = 1 USDC)
    ) {
        owner.require_auth();

        // Prevent re-initialization
        if env.storage().instance().has(&DataKey::Config) {
            panic!("AgentPay: already initialized");
        }

        let config = AgentConfig {
            owner,
            spending_limit,
            daily_limit,
            is_active: true,
        };

        env.storage().instance().set(&DataKey::Config, &config);
        env.storage().instance().set(&DataKey::TokenAddr, &token_addr);
        env.storage().instance().set(&DataKey::Balance, &0i128);
        env.storage().instance().set(&DataKey::PaymentCount, &0u32);
        env.storage().instance().set(&DataKey::DailySpent, &0i128);
        env.storage().instance().set(&DataKey::LastResetDay, &0u64);

        // Extend instance storage TTL
        env.storage().instance().extend_ttl(100, 100);
    }

    // ----------------------------------------------------------
    // DEPOSIT — fund the agent wallet
    // ----------------------------------------------------------
    pub fn deposit(env: Env, from: Address, amount: i128) {
        from.require_auth();

        if amount <= 0 {
            panic!("AgentPay: amount must be positive");
        }

        let token_addr: Address = env.storage().instance().get(&DataKey::TokenAddr).unwrap();
        let token_client = token::Client::new(&env, &token_addr);

        // Transfer from funder to this contract
        token_client.transfer(&from, &env.current_contract_address(), &amount);

        // Update balance
        let current: i128 = env.storage().instance().get(&DataKey::Balance).unwrap_or(0);
        env.storage().instance().set(&DataKey::Balance, &(current + amount));

        env.storage().instance().extend_ttl(100, 100);
    }

    // ----------------------------------------------------------
    // PAY — agent autonomously pays for a resource
    // Core function: enforces spending rules on-chain
    // ----------------------------------------------------------
    pub fn pay(
        env: Env,
        caller: Address,        // must be the agent owner
        resource: String,       // what was purchased e.g. "/api/weather"
        amount: i128,
        recipient: Address,     // API provider wallet
    ) -> bool {
        caller.require_auth();

        let config: AgentConfig = env.storage().instance().get(&DataKey::Config).unwrap();

        // Auth check: only owner can trigger payments
        if caller != config.owner {
            panic!("AgentPay: unauthorized caller");
        }

        // Agent must be active
        if !config.is_active {
            panic!("AgentPay: agent is paused");
        }

        // Rule 1: per-tx spending limit
        if amount > config.spending_limit {
            panic!("AgentPay: exceeds per-transaction spending limit");
        }

        // Rule 2: daily limit check
        let current_day: u64 = env.ledger().timestamp() / 86400;
        let last_reset_day: u64 = env.storage().instance().get(&DataKey::LastResetDay).unwrap_or(0);
        
        let daily_spent: i128 = if current_day > last_reset_day {
            // New day — reset counter
            env.storage().instance().set(&DataKey::LastResetDay, &current_day);
            env.storage().instance().set(&DataKey::DailySpent, &0i128);
            0i128
        } else {
            env.storage().instance().get(&DataKey::DailySpent).unwrap_or(0)
        };

        if daily_spent + amount > config.daily_limit {
            panic!("AgentPay: daily spending limit reached");
        }

        // Rule 3: balance check
        let balance: i128 = env.storage().instance().get(&DataKey::Balance).unwrap_or(0);
        if amount > balance {
            panic!("AgentPay: insufficient balance");
        }

        // ✅ All rules passed — execute payment
        let token_addr: Address = env.storage().instance().get(&DataKey::TokenAddr).unwrap();
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&env.current_contract_address(), &recipient, &amount);

        // Update state
        env.storage().instance().set(&DataKey::Balance, &(balance - amount));
        env.storage().instance().set(&DataKey::DailySpent, &(daily_spent + amount));

        // Record payment on-chain
        let count: u32 = env.storage().instance().get(&DataKey::PaymentCount).unwrap_or(0);
        let payment = Payment {
            resource,
            amount,
            recipient,
            timestamp: env.ledger().timestamp(),
        };
        env.storage().persistent().set(&DataKey::Payment(count), &payment);
        env.storage().instance().set(&DataKey::PaymentCount, &(count + 1));

        env.storage().instance().extend_ttl(100, 100);

        true
    }

    // ----------------------------------------------------------
    // PAUSE / RESUME — owner can kill the agent instantly
    // ----------------------------------------------------------
    pub fn set_active(env: Env, caller: Address, active: bool) {
        caller.require_auth();

        let mut config: AgentConfig = env.storage().instance().get(&DataKey::Config).unwrap();
        if caller != config.owner {
            panic!("AgentPay: unauthorized");
        }

        config.is_active = active;
        env.storage().instance().set(&DataKey::Config, &config);
    }

    // ----------------------------------------------------------
    // UPDATE LIMITS — owner adjusts spending rules
    // ----------------------------------------------------------
    pub fn update_limits(
        env: Env,
        caller: Address,
        new_spending_limit: i128,
        new_daily_limit: i128,
    ) {
        caller.require_auth();

        let mut config: AgentConfig = env.storage().instance().get(&DataKey::Config).unwrap();
        if caller != config.owner {
            panic!("AgentPay: unauthorized");
        }

        config.spending_limit = new_spending_limit;
        config.daily_limit = new_daily_limit;
        env.storage().instance().set(&DataKey::Config, &config);
    }

    // ----------------------------------------------------------
    // WITHDRAW — owner pulls remaining funds
    // ----------------------------------------------------------
    pub fn withdraw(env: Env, caller: Address, amount: i128) {
        caller.require_auth();

        let config: AgentConfig = env.storage().instance().get(&DataKey::Config).unwrap();
        if caller != config.owner {
            panic!("AgentPay: unauthorized");
        }

        let balance: i128 = env.storage().instance().get(&DataKey::Balance).unwrap_or(0);
        if amount > balance {
            panic!("AgentPay: insufficient balance");
        }

        let token_addr: Address = env.storage().instance().get(&DataKey::TokenAddr).unwrap();
        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&env.current_contract_address(), &config.owner, &amount);

        env.storage().instance().set(&DataKey::Balance, &(balance - amount));
    }

    // ----------------------------------------------------------
    // READ FUNCTIONS
    // ----------------------------------------------------------

    pub fn get_balance(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Balance).unwrap_or(0)
    }

    pub fn get_config(env: Env) -> AgentConfig {
        env.storage().instance().get(&DataKey::Config).unwrap()
    }

    pub fn get_payment_count(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::PaymentCount).unwrap_or(0)
    }

    pub fn get_payment(env: Env, index: u32) -> Payment {
        env.storage().persistent().get(&DataKey::Payment(index)).unwrap()
    }

    pub fn get_daily_spent(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::DailySpent).unwrap_or(0)
    }
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger};
    use soroban_sdk::{token::Client as TokenClient, token::StellarAssetClient, Env, Address};

    fn create_token(env: &Env, admin: &Address) -> Address {
        let token_contract = env.register_stellar_asset_contract_v2(admin.clone());
        token_contract.address().clone()
    }

    #[test]
    fn test_full_agent_payment_flow() {
        let env = Env::default();
        env.mock_all_auths();

        let owner = Address::generate(&env);
        let api_provider = Address::generate(&env);

        // Setup token
        let token_addr = create_token(&env, &owner);
        let token_admin = StellarAssetClient::new(&env, &token_addr);
        let token = TokenClient::new(&env, &token_addr);

        // Mint 100 USDC to owner (100 * 10_000_000 stroops)
        token_admin.mint(&owner, &1_000_000_000);

        // Deploy AgentPay contract
        let contract_addr = env.register(AgentPayContract, ());
        let client = AgentPayContractClient::new(&env, &contract_addr);

        // Initialize: 0.01 USDC per tx limit, 0.1 USDC daily limit
        client.initialize(
            &owner,
            &token_addr,
            &100_000,       // 0.01 USDC per tx
            &1_000_000,     // 0.1 USDC per day
        );

        // Deposit 1 USDC into agent wallet
        client.deposit(&owner, &10_000_000);
        assert_eq!(client.get_balance(), 10_000_000);

        // Agent pays for /api/crypto-price — 0.001 USDC
        let result = client.pay(
            &owner,
            &String::from_str(&env, "/api/crypto-price"),
            &10_000,
            &api_provider,
        );
        assert!(result);

        // Balance reduced
        assert_eq!(client.get_balance(), 9_990_000);

        // Payment recorded
        assert_eq!(client.get_payment_count(), 1);
        let payment = client.get_payment(&0);
        assert_eq!(payment.amount, 10_000);

        // Daily spend tracked
        assert_eq!(client.get_daily_spent(), 10_000);

        println!("✅ Full agent payment flow passed");
    }

    #[test]
    #[should_panic(expected = "exceeds per-transaction spending limit")]
    fn test_spending_limit_enforced() {
        let env = Env::default();
        env.mock_all_auths();

        let owner = Address::generate(&env);
        let recipient = Address::generate(&env);

        let token_addr = create_token(&env, &owner);
        let token_admin = StellarAssetClient::new(&env, &token_addr);
        token_admin.mint(&owner, &1_000_000_000);

        let contract_addr = env.register(AgentPayContract, ());
        let client = AgentPayContractClient::new(&env, &contract_addr);

        client.initialize(&owner, &token_addr, &100_000, &1_000_000);
        client.deposit(&owner, &10_000_000);

        // Try to pay MORE than spending limit — should panic
        client.pay(
            &owner,
            &String::from_str(&env, "/api/expensive"),
            &999_999_999, // way over limit
            &recipient,
        );
    }

    #[test]
    fn test_pause_agent() {
        let env = Env::default();
        env.mock_all_auths();

        let owner = Address::generate(&env);
        let recipient = Address::generate(&env);

        let token_addr = create_token(&env, &owner);
        let token_admin = StellarAssetClient::new(&env, &token_addr);
        token_admin.mint(&owner, &1_000_000_000);

        let contract_addr = env.register(AgentPayContract, ());
        let client = AgentPayContractClient::new(&env, &contract_addr);

        client.initialize(&owner, &token_addr, &100_000, &1_000_000);
        client.deposit(&owner, &10_000_000);

        // Pause agent
        client.set_active(&owner, &false);

        let config = client.get_config();
        assert!(!config.is_active);

        println!("✅ Pause agent test passed");
    }
}