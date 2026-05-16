#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum UniversityStatus {
    Pending,
    Approved,
    Removed,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct University {
    pub name: Symbol,
    pub status: UniversityStatus,
}

#[contracttype]
pub enum DataKey {
    Admin,
    University(Address),
}

#[contract]
pub struct UniversityRegistry;

#[contractimpl]
impl UniversityRegistry {
    /// Initialize the registry with an admin address.
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Register a new university (pending approval).
    pub fn register(env: Env, university: Address, name: Symbol) {
        university.require_auth();
        let key = DataKey::University(university.clone());
        if env.storage().persistent().has(&key) {
            panic!("already registered");
        }
        env.storage().persistent().set(
            &key,
            &University {
                name,
                status: UniversityStatus::Pending,
            },
        );
    }

    /// Approve a registered university (admin only).
    pub fn approve(env: Env, university: Address) {
        Self::require_admin(&env);
        let key = DataKey::University(university.clone());
        let mut u: University = env.storage().persistent().get(&key).expect("not registered");
        u.status = UniversityStatus::Approved;
        env.storage().persistent().set(&key, &u);
    }

    /// Remove a university (admin only).
    pub fn remove(env: Env, university: Address) {
        Self::require_admin(&env);
        let key = DataKey::University(university.clone());
        let mut u: University = env.storage().persistent().get(&key).expect("not registered");
        u.status = UniversityStatus::Removed;
        env.storage().persistent().set(&key, &u);
    }

    /// Returns true if the university is approved.
    pub fn is_approved(env: Env, university: Address) -> bool {
        let key = DataKey::University(university);
        match env.storage().persistent().get::<DataKey, University>(&key) {
            Some(u) => u.status == UniversityStatus::Approved,
            None => false,
        }
    }

    /// Get full university record.
    pub fn get(env: Env, university: Address) -> Option<University> {
        env.storage()
            .persistent()
            .get(&DataKey::University(university))
    }

    // ── helpers ──────────────────────────────────────────────────────────────

    fn require_admin(env: &Env) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("not initialized");
        admin.require_auth();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::Env;

    fn setup() -> (Env, UniversityRegistryClient<'static>, Address, Address) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(UniversityRegistry, ());
        let client = UniversityRegistryClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let uni = Address::generate(&env);
        client.initialize(&admin);
        (env, client, admin, uni)
    }

    #[test]
    fn test_register_and_approve() {
        let (_env, client, _admin, uni) = setup();
        let name = soroban_sdk::symbol_short!("MIT");
        client.register(&uni, &name);
        assert!(!client.is_approved(&uni));
        client.approve(&uni);
        assert!(client.is_approved(&uni));
    }

    #[test]
    fn test_remove() {
        let (_env, client, _admin, uni) = setup();
        let name = soroban_sdk::symbol_short!("MIT");
        client.register(&uni, &name);
        client.approve(&uni);
        client.remove(&uni);
        assert!(!client.is_approved(&uni));
    }

    #[test]
    fn test_get() {
        let (_env, client, _admin, uni) = setup();
        let name = soroban_sdk::symbol_short!("MIT");
        client.register(&uni, &name);
        let record = client.get(&uni).unwrap();
        assert_eq!(record.status, UniversityStatus::Pending);
    }

    #[test]
    #[should_panic(expected = "already registered")]
    fn test_double_register_panics() {
        let (_env, client, _admin, uni) = setup();
        let name = soroban_sdk::symbol_short!("MIT");
        client.register(&uni, &name);
        client.register(&uni, &name);
    }

    #[test]
    fn test_unknown_university_not_approved() {
        let (env, client, _admin, _uni) = setup();
        let stranger = Address::generate(&env);
        assert!(!client.is_approved(&stranger));
    }
}
