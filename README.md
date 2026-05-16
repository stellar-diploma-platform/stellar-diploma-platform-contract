# stellar-diploma-platform — Contract

Soroban smart contracts for issuing, verifying, and revoking tamper-proof academic diplomas as **soulbound NFTs** on the Stellar network.

Part of a three-repo monorepo:

| Repo | Description |
|---|---|
| **contract** (this repo) | Soroban smart contracts |
| **backend** | NestJS API, IPFS integration, notifications |
| **frontend** | Next.js student/employer/university portal |

---

## Contracts

### `university_registry`

Maintains the list of accredited institutions. Only approved universities can issue diplomas.

| Function | Auth | Description |
|---|---|---|
| `initialize(admin)` | — | One-time setup, sets the admin |
| `register(university, name)` | university | Self-register (status: Pending) |
| `approve(university)` | admin | Approve a registered university |
| `remove(university)` | admin | Revoke a university's approval |
| `is_approved(university)` | — | Returns `bool` — used by diploma contract |
| `get(university)` | — | Returns full university record |

### `diploma_contract`

Issues soulbound NFTs to students. Enforces non-transferability at the contract level.

| Function | Auth | Description |
|---|---|---|
| `initialize(admin, registry)` | — | One-time setup |
| `mint(university, student, degree, metadata_uri)` | university | Issues a diploma NFT; verifies university is approved |
| `verify(token_id)` | — | Returns `true` if diploma exists and is not revoked |
| `revoke(university, token_id)` | university | Revokes a diploma; only the issuing university can revoke |
| `get_diploma(token_id)` | — | Returns full diploma record |
| `transfer(...)` | — | Always panics — soulbound enforcement |

#### Diploma data structure

```rust
pub struct Diploma {
    pub token_id:     u64,
    pub student:      Address,
    pub university:   Address,
    pub degree:       Symbol,
    pub issued_at:    u64,       // ledger timestamp
    pub revoked:      bool,
    pub metadata_uri: String,    // ipfs://...
}
```

---

## Project Structure

```
contracts/
├── diploma_contract/
│   ├── src/lib.rs
│   └── Cargo.toml
└── university_registry/
    ├── src/lib.rs
    └── Cargo.toml
Cargo.toml   ← workspace root
```

---

## Getting Started

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32v1-none

# Install Stellar CLI
cargo install --locked stellar-cli
```

### Run Tests

```bash
cargo test
```

Expected output:

```
running 7 tests   # diploma_contract
test result: ok. 7 passed

running 5 tests   # university_registry
test result: ok. 5 passed
```

### Build for Deployment

```bash
cargo build --release --target wasm32v1-none
```

WASM artifacts will be at:

```
target/wasm32v1-none/release/diploma_contract.wasm
target/wasm32v1-none/release/university_registry.wasm
```

### Deploy to Testnet

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/university_registry.wasm \
  --source <YOUR_ACCOUNT> \
  --network testnet

stellar contract deploy \
  --wasm target/wasm32v1-none/release/diploma_contract.wasm \
  --source <YOUR_ACCOUNT> \
  --network testnet
```

---

## Implementation Status

### Done (50%)
- [x] `university_registry` — register, approve, remove, query
- [x] `diploma_contract` — mint, verify, revoke, soulbound transfer block
- [x] Cross-contract call: diploma contract validates university on every mint
- [x] 12 unit tests, all passing

### Remaining (50%)
- [ ] `access_control` — role-based permissions contract
- [ ] `revocation_contract` — standalone revocation registry
- [ ] `metadata_registry` — on-chain IPFS hash registry
- [ ] Batch minting
- [ ] Contract events (`DiplomaIssued`, `DiplomaRevoked`, `UniversityApproved`)

---

## Architecture

```
diploma_contract
      │
      │ cross-contract call: is_approved()
      ▼
university_registry
```

Sensitive student data (PDFs, transcripts) is stored off-chain on IPFS. Only the IPFS hash is stored on-chain via `metadata_uri`.

---

## Tech Stack

- **Rust** — contract language
- **Soroban SDK** `v22` — smart contract framework
- **Stellar** — L1 blockchain
- **IPFS** — off-chain metadata storage

---

## License

MIT
