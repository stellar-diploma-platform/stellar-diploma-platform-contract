#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Diploma {
    pub token_id: u64,
    pub student: Address,
    pub university: Address,
    pub degree: Symbol,
    pub issued_at: u64,
    pub revoked: bool,
    pub metadata_uri: String,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Registry,
    NextId,
    Diploma(u64),
}

#[contract]
pub struct DiplomaContract;

#[contractimpl]
impl DiplomaContract {
    /// Initialize with an admin and the university registry contract address.
    pub fn initialize(env: Env, admin: Address, registry: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Registry, &registry);
        env.storage().instance().set(&DataKey::NextId, &1u64);
    }

    /// Mint a soulbound diploma NFT. Only approved universities can call this.
    pub fn mint(
        env: Env,
        university: Address,
        student: Address,
        degree: Symbol,
        metadata_uri: String,
    ) -> u64 {
        university.require_auth();
        Self::require_approved_university(&env, &university);

        let token_id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::NextId)
            .unwrap_or(1);

        let diploma = Diploma {
            token_id,
            student,
            university,
            degree,
            issued_at: env.ledger().timestamp(),
            revoked: false,
            metadata_uri,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Diploma(token_id), &diploma);
        env.storage()
            .instance()
            .set(&DataKey::NextId, &(token_id + 1));

        token_id
    }

    /// Verify a diploma — returns true if it exists and is not revoked.
    pub fn verify(env: Env, token_id: u64) -> bool {
        match env
            .storage()
            .persistent()
            .get::<DataKey, Diploma>(&DataKey::Diploma(token_id))
        {
            Some(d) => !d.revoked,
            None => false,
        }
    }

    /// Revoke a diploma. Only the issuing university can revoke.
    pub fn revoke(env: Env, university: Address, token_id: u64) {
        university.require_auth();
        let key = DataKey::Diploma(token_id);
        let mut diploma: Diploma = env
            .storage()
            .persistent()
            .get(&key)
            .expect("diploma not found");
        if diploma.university != university {
            panic!("not the issuing university");
        }
        diploma.revoked = true;
        env.storage().persistent().set(&key, &diploma);
    }

    /// Fetch a diploma record.
    pub fn get_diploma(env: Env, token_id: u64) -> Option<Diploma> {
        env.storage()
            .persistent()
            .get(&DataKey::Diploma(token_id))
    }

    // ── soulbound: transfer is permanently disabled ───────────────────────────

    pub fn transfer(_env: Env, _from: Address, _to: Address, _token_id: u64) {
        panic!("soulbound: transfer disabled");
    }

    // ── helpers ───────────────────────────────────────────────────────────────

    fn require_approved_university(env: &Env, university: &Address) {
        let registry: Address = env
            .storage()
            .instance()
            .get(&DataKey::Registry)
            .expect("not initialized");

        // Cross-contract call to university_registry::is_approved
        let approved: bool = env.invoke_contract(
            &registry,
            &Symbol::new(env, "is_approved"),
            soroban_sdk::vec![env, university.to_val()],
        );
        if !approved {
            panic!("university not approved");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{Env, String};

    // ── minimal registry stub ─────────────────────────────────────────────────

    /// A tiny in-test registry that always returns approved=true for any address.
    #[contract]
    pub struct StubRegistry;

    #[contractimpl]
    impl StubRegistry {
        pub fn is_approved(_env: Env, _university: Address) -> bool {
            true
        }
    }

    fn setup() -> (Env, DiplomaContractClient<'static>, Address, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let registry_id = env.register(StubRegistry, ());
        let contract_id = env.register(DiplomaContract, ());
        let client = DiplomaContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let uni = Address::generate(&env);

        client.initialize(&admin, &registry_id);
        (env, client, admin, uni)
    }

    fn uri(env: &Env) -> String {
        String::from_str(env, "ipfs://QmTest")
    }

    #[test]
    fn test_mint_and_verify() {
        let (env, client, _admin, uni) = setup();
        let student = Address::generate(&env);
        let degree = soroban_sdk::symbol_short!("BSc");

        let id = client.mint(&uni, &student, &degree, &uri(&env));
        assert_eq!(id, 1);
        assert!(client.verify(&id));
    }

    #[test]
    fn test_revoke() {
        let (env, client, _admin, uni) = setup();
        let student = Address::generate(&env);
        let degree = soroban_sdk::symbol_short!("BSc");

        let id = client.mint(&uni, &student, &degree, &uri(&env));
        assert!(client.verify(&id));
        client.revoke(&uni, &id);
        assert!(!client.verify(&id));
    }

    #[test]
    fn test_get_diploma() {
        let (env, client, _admin, uni) = setup();
        let student = Address::generate(&env);
        let degree = soroban_sdk::symbol_short!("MBA");

        let id = client.mint(&uni, &student, &degree, &uri(&env));
        let diploma = client.get_diploma(&id).unwrap();
        assert_eq!(diploma.token_id, id);
        assert_eq!(diploma.student, student);
        assert!(!diploma.revoked);
    }

    #[test]
    fn test_verify_nonexistent_returns_false() {
        let (_env, client, _admin, _uni) = setup();
        assert!(!client.verify(&999));
    }

    #[test]
    fn test_token_ids_increment() {
        let (env, client, _admin, uni) = setup();
        let student = Address::generate(&env);
        let degree = soroban_sdk::symbol_short!("BSc");

        let id1 = client.mint(&uni, &student, &degree, &uri(&env));
        let id2 = client.mint(&uni, &student, &degree, &uri(&env));
        assert_eq!(id2, id1 + 1);
    }

    #[test]
    #[should_panic(expected = "soulbound: transfer disabled")]
    fn test_transfer_panics() {
        let (env, client, _admin, uni) = setup();
        let student = Address::generate(&env);
        let other = Address::generate(&env);
        let degree = soroban_sdk::symbol_short!("BSc");
        let id = client.mint(&uni, &student, &degree, &uri(&env));
        client.transfer(&student, &other, &id);
    }

    #[test]
    #[should_panic(expected = "not the issuing university")]
    fn test_revoke_wrong_university_panics() {
        let (env, client, _admin, uni) = setup();
        let student = Address::generate(&env);
        let other_uni = Address::generate(&env);
        let degree = soroban_sdk::symbol_short!("BSc");
        let id = client.mint(&uni, &student, &degree, &uri(&env));
        client.revoke(&other_uni, &id);
    }
}
