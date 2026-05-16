# Contract Interface — Structure & Flow

> Single source of truth for the backend and frontend teams.
> Every function, type, error, and flow documented here reflects what is **actually deployed** in the contracts.
> Do not integrate against anything not listed here.

---

## Status Legend

| Symbol | Meaning |
|---|---|
| ✅ | Implemented & tested |
| 🔜 | Planned (not yet available) |

---

## Contracts Overview

| Contract | Address Key | Status |
|---|---|---|
| `university_registry` | `REGISTRY_CONTRACT_ID` | ✅ |
| `diploma_contract` | `DIPLOMA_CONTRACT_ID` | ✅ |
| `access_control` | `ACCESS_CONTRACT_ID` | 🔜 |
| `revocation_contract` | `REVOCATION_CONTRACT_ID` | 🔜 |
| `metadata_registry` | `METADATA_CONTRACT_ID` | 🔜 |

---

## `university_registry`

### Data Types

```rust
enum UniversityStatus { Pending, Approved, Removed }

struct University {
    name:   Symbol,
    status: UniversityStatus,
}
```

### Functions

#### `initialize(admin: Address)`
- **Auth:** none (one-time, panics if called again)
- **Called by:** deployment script
- **Returns:** nothing

#### `register(university: Address, name: Symbol)`
- **Auth:** `university` must sign
- **Called by:** backend — university onboarding flow
- **Returns:** nothing
- **Errors:** panics `"already registered"`

#### `approve(university: Address)`
- **Auth:** admin must sign
- **Called by:** backend — admin approval endpoint
- **Returns:** nothing
- **Errors:** panics `"not registered"`

#### `remove(university: Address)`
- **Auth:** admin must sign
- **Called by:** backend — admin revoke accreditation endpoint
- **Returns:** nothing
- **Errors:** panics `"not registered"`

#### `is_approved(university: Address) → bool`
- **Auth:** none (read-only)
- **Called by:** `diploma_contract` internally on every mint; backend for pre-checks; frontend for UI gating
- **Returns:** `true` | `false`

#### `get(university: Address) → Option<University>`
- **Auth:** none (read-only)
- **Called by:** backend, frontend — display university status
- **Returns:** `{ name: Symbol, status: Pending | Approved | Removed }` or `null`

---

## `diploma_contract`

### Data Types

```rust
struct Diploma {
    token_id:     u64,
    student:      Address,
    university:   Address,
    degree:       Symbol,       // max 9 chars e.g. "BSc", "MBA", "PhD"
    issued_at:    u64,          // ledger timestamp (Unix seconds)
    revoked:      bool,
    metadata_uri: String,       // "ipfs://Qm..."
}
```

### Functions

#### `initialize(admin: Address, registry: Address)`
- **Auth:** none (one-time, panics if called again)
- **Called by:** deployment script
- **Returns:** nothing

#### `mint(university: Address, student: Address, degree: Symbol, metadata_uri: String) → u64`
- **Auth:** `university` must sign
- **Called by:** backend — diploma issuance endpoint
- **Returns:** `token_id` (auto-incrementing `u64` starting at 1)
- **Side effects:** cross-contract call to `university_registry.is_approved()` — panics if not approved
- **Errors:** panics `"university not approved"`

#### `verify(token_id: u64) → bool`
- **Auth:** none (read-only)
- **Called by:** backend verification endpoint; frontend employer portal
- **Returns:** `true` if diploma exists and `revoked == false`, otherwise `false`

#### `revoke(university: Address, token_id: u64)`
- **Auth:** `university` must sign
- **Called by:** backend — revocation endpoint
- **Returns:** nothing
- **Errors:** panics `"diploma not found"` | `"not the issuing university"`

#### `get_diploma(token_id: u64) → Option<Diploma>`
- **Auth:** none (read-only)
- **Called by:** backend, frontend — display diploma details
- **Returns:** full `Diploma` struct or `null`

#### `transfer(from, to, token_id)` — **SOULBOUND**
- **Always panics** `"soulbound: transfer disabled"`
- Never call this. It exists only to block wallet-level transfer attempts.

---

## Cross-Contract Dependency

```
diploma_contract.mint()
        │
        │ invoke_contract("is_approved", [university])
        ▼
university_registry
        │
        └─ returns bool
```

The `diploma_contract` holds the `university_registry` contract address set at `initialize()`.
Backend must deploy `university_registry` first, then pass its ID when deploying `diploma_contract`.

---

## Deployment Order

```
1. Deploy university_registry
        └─ call initialize(admin)

2. Deploy diploma_contract
        └─ call initialize(admin, <university_registry_contract_id>)

3. Register + approve universities via university_registry

4. Begin minting diplomas via diploma_contract
```

---

## Integration Flows

### University Registration

```
University submits form (frontend)
        │
        ▼
Backend signs + submits: university_registry.register(uni_address, name)
        │
        ▼
Admin reviews (frontend/admin dashboard)
        │
        ▼
Backend signs + submits: university_registry.approve(uni_address)
        │
        ▼
University can now mint diplomas
```

### Diploma Issuance

```
University uploads graduate list (frontend)
        │
        ▼
Backend uploads metadata to IPFS → gets ipfs://Qm... URI
        │
        ▼
Backend signs + submits: diploma_contract.mint(uni, student, degree, uri)
        │
        ├─ contract calls university_registry.is_approved(uni) internally
        │
        ▼
Returns token_id → backend stores in DB
        │
        ▼
Student notified with token_id + verification link
```

### Diploma Verification (Employer)

```
Employer scans QR / enters token_id (frontend)
        │
        ▼
Frontend or backend calls: diploma_contract.verify(token_id)
        │
        ├─ true  → fetch full record: diploma_contract.get_diploma(token_id)
        └─ false → "Invalid or revoked diploma"
```

### Diploma Revocation

```
University submits revocation request (frontend)
        │
        ▼
Backend signs + submits: diploma_contract.revoke(uni_address, token_id)
        │
        ▼
diploma_contract.verify(token_id) now returns false
```

---

## Backend Integration Notes

- All **write** functions require a signed Stellar transaction from the authorized address.
- All **read** functions (`is_approved`, `verify`, `get`, `get_diploma`) can be called via `simulateTransaction` — no fee, no signing required.
- `degree` is a Soroban `Symbol` — **max 9 characters**. Encode longer degree names in the IPFS metadata, not on-chain.
- `issued_at` is the **ledger timestamp** (Unix seconds), not wall clock. Use it for display only.
- `token_id` starts at `1` and increments by 1 per mint. Store it in your DB as the primary on-chain reference.
- Store the `metadata_uri` (`ipfs://Qm...`) in your DB alongside `token_id` for fast lookups without on-chain reads.

## Frontend Integration Notes

- Use `verify(token_id)` for the fast pass/fail check on the employer portal.
- Use `get_diploma(token_id)` only when you need to display full details (student address, university, degree, issued date).
- Gate the "Issue Diploma" UI behind `is_approved(university_address) === true`.
- Gate the "Revoke" button to only show for the university that issued the diploma (`diploma.university === connected_wallet`).
- QR codes should encode the verification URL, not the raw token_id: `https://verify.yourdomain.com/diploma/<token_id>`

---

## Planned Contracts (Do Not Integrate Yet)

| Contract | What it will add |
|---|---|
| `access_control` | Fine-grained role management (issuer, revoker, auditor) |
| `revocation_contract` | Standalone revocation registry, decoupled from diploma contract |
| `metadata_registry` | On-chain IPFS hash registry for tamper-proof metadata anchoring |

Events (`DiplomaIssued`, `DiplomaRevoked`, `UniversityApproved`) are also planned — backend should poll via Horizon until events are emitted.
