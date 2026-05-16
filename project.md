this # Verifiable Diplomas — Soulbound NFTs on Stellar

> A decentralized credential verification platform where universities issue tamper-proof, non-transferable academic certificates as Soulbound NFTs using **Stellar + Soroban** smart contracts.

Students own their credentials permanently, employers verify authenticity instantly, and institutions eliminate diploma fraud.

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Why Stellar + Soroban](#2-why-stellar--soroban)
3. [Main Actors](#3-main-actors)
4. [Core Features](#4-core-features)
5. [System Architecture](#5-system-architecture)
6. [Smart Contract Architecture](#6-smart-contract-architecture)
7. [Tech Stack](#7-tech-stack)
8. [Project Structure](#8-project-structure)
9. [Soulbound NFT Logic](#9-soulbound-nft-logic)
10. [Contract Data Model](#10-contract-data-model)
11. [Key Contract Functions](#11-key-contract-functions)
12. [Backend Architecture](#12-backend-architecture)
13. [Frontend Pages](#13-frontend-pages)
14. [Database Schema](#14-database-schema)
15. [Metadata & Storage](#15-metadata--storage)
16. [Verification Flow](#16-verification-flow)
17. [Security Features](#17-security-features)
18. [Advanced Features](#18-advanced-features)
19. [Development Setup](#19-development-setup)
20. [Testing Strategy](#20-testing-strategy)
21. [DevOps & CI/CD](#21-devops--cicd)
22. [MVP Roadmap](#22-mvp-roadmap)
23. [Example User Flows](#23-example-user-flows)
24. [Best Practices](#24-best-practices)
25. [Future Expansion](#25-future-expansion)

---

## 1. Project Overview

Universities issue the following as **non-transferable NFTs (Soulbound Tokens)** on Stellar:

- Degree certificates
- Diplomas & transcripts
- Professional certifications
- Course completion badges

These credentials:

- ❌ Cannot be sold or transferred
- ✅ Are cryptographically verifiable
- ✅ Exist permanently on-chain
- ✅ Can be publicly verified by employers
- ✅ Can be revoked by universities if necessary

---

## 2. Why Stellar + Soroban

| Feature | Benefit |
|---|---|
| **Fast Transactions** | Credential issuance and verification happen instantly |
| **Very Low Fees** | Ideal for universities issuing thousands of certificates |
| **Institutional Ecosystem** | Stellar supports compliance and institutional-grade systems |
| **Soroban Smart Contracts** | Enables soulbound logic, revocation, role-based issuance, and metadata handling |

---

## 3. Main Actors

| Actor | Role |
|---|---|
| **University** | Issues diplomas |
| **Student** | Receives and owns credentials |
| **Employer** | Verifies diploma authenticity |
| **Accreditation Body** | Approves universities |
| **Admin** | System governance |

---

## 4. Core Features

### A. Soulbound Diploma NFTs

Non-transferable NFT certificates. Each NFT contains:

```json
{
  "student_name": "John Doe",
  "degree": "BSc Computer Science",
  "institution": "ABC University",
  "graduation_year": 2026,
  "gpa": "4.5",
  "certificate_hash": "QmX...",
  "issue_date": "2026-07-01"
}
```

### B. Credential Verification Portal

Employers can enter a wallet address, scan a QR code, check diploma authenticity, and verify issuer accreditation.

### C. Revocation System

Universities can revoke fraudulent diplomas, administrative mistakes, or suspended credentials.

### D. Multi-Institution Support

Supports universities, bootcamps, online academies, and government institutions.

### E. Privacy Layer

| Storage | Data |
|---|---|
| **On-chain** | Hashes, references, proofs |
| **Off-chain** | Full transcript PDFs, metadata, student documents |

---

## 5. System Architecture

```
                    ┌────────────────────┐
                    │  Frontend Portal   │
                    └─────────┬──────────┘
                              │
               ┌──────────────┴──────────────┐
               │                             │
      ┌────────▼────────┐         ┌──────────▼───────┐
      │  Backend API    │         │ Verification API  │
      └────────┬────────┘         └──────────┬────────┘
               │                             │
       ┌───────▼─────────────────────────────▼──────┐
       │             Soroban Smart Contracts         │
       │                                             │
       │  · Diploma NFT Contract                     │
       │  · University Registry                      │
       │  · Revocation Contract                      │
       └─────────────────────────────────────────────┘
                              │
                     ┌────────▼────────┐
                     │ Stellar Network │
                     └─────────────────┘
```

---

## 6. Smart Contract Architecture

| Contract | Purpose |
|---|---|
| `diploma_contract` | Soulbound NFT issuance |
| `university_registry` | Approved institutions |
| `revocation_contract` | Revoked certificates |
| `access_control` | Role permissions |
| `metadata_registry` | Credential metadata |

### Contract File Structure

```
contracts/diploma_contract/
├── src/
│   ├── lib.rs
│   ├── storage.rs
│   ├── events.rs
│   ├── errors.rs
│   ├── mint.rs
│   ├── revoke.rs
│   ├── verify.rs
│   ├── metadata.rs
│   └── soulbound.rs
├── Cargo.toml
└── Makefile
```

---

## 7. Tech Stack

### Blockchain

| Component | Technology |
|---|---|
| Blockchain | Stellar |
| Smart Contracts | Soroban SDK (Rust) |
| Wallet | Freighter |
| Storage | IPFS |
| Indexing | Stellar RPC / Horizon |

### Backend

| Component | Technology |
|---|---|
| API | Node.js / NestJS |
| Database | PostgreSQL |
| Queue | Redis / BullMQ |
| Authentication | JWT / OAuth |
| File Storage | IPFS / Filecoin |

### Frontend

| Component | Technology |
|---|---|
| Framework | Next.js |
| Styling | TailwindCSS |
| Wallet Integration | Freighter API |
| State Management | Zustand |
| QR Verification | qrcode.js |

---

## 8. Project Structure

```
stellar-diploma-platform/
├── apps/
│   ├── frontend/
│   ├── backend/
│   ├── verifier-portal/
│   └── admin-dashboard/
├── contracts/
│   ├── diploma_contract/
│   ├── university_registry/
│   ├── revocation_contract/
│   ├── access_control/
│   └── shared/
├── packages/
│   ├── sdk/
│   ├── ui/
│   ├── types/
│   └── config/
├── infrastructure/
│   ├── docker/
│   ├── kubernetes/
│   ├── terraform/
│   └── monitoring/
├── scripts/
│   ├── deploy/
│   ├── seed/
│   └── migration/
├── docs/
├── tests/
├── .github/
└── README.md
```

---

## 9. Soulbound NFT Logic

| Action | Allowed |
|---|---|
| Mint diploma | ✅ |
| Verify diploma | ✅ |
| Revoke diploma | ✅ |
| Read metadata | ✅ |
| Transfer NFT | ❌ |
| Sell NFT | ❌ |
| Approve NFT transfers | ❌ |

---

## 10. Contract Data Model

```rust
pub struct Diploma {
    pub token_id: u64,
    pub student: Address,
    pub university: Address,
    pub degree: Symbol,
    pub major: Symbol,
    pub issued_at: u64,
    pub revoked: bool,
    pub metadata_uri: String,
}
```

---

## 11. Key Contract Functions

```rust
// Issue a diploma
fn mint_diploma(env: Env, student: Address, degree: Symbol, metadata_uri: String)

// Verify a diploma
fn verify_diploma(env: Env, token_id: u64) -> bool

// Revoke a diploma
fn revoke_diploma(env: Env, token_id: u64)

// Block transfers (soulbound enforcement)
fn transfer(...) {
    panic!("Soulbound token: transfer disabled");
}
```

---

## 12. Backend Architecture

| Service | Purpose |
|---|---|
| Auth Service | University login |
| Diploma Service | Issue certificates |
| Verification Service | Public verification |
| Metadata Service | IPFS uploads |
| Notification Service | Email students |

```
apps/backend/
├── src/
│   ├── modules/
│   │   ├── auth/
│   │   ├── diploma/
│   │   ├── university/
│   │   ├── verification/
│   │   └── notifications/
│   ├── blockchain/
│   ├── database/
│   ├── config/
│   └── utils/
├── prisma/
└── package.json
```

---

## 13. Frontend Pages

```
/student
├── dashboard
├── my-certificates
├── wallet-connect
└── profile

/verify
├── qr-scan
├── search
├── result
└── accreditation

/university
├── issue-diploma
├── batch-upload
├── revoke
├── analytics
└── settings
```

---

## 14. Database Schema

**`universities`** — `id`, `name`, `wallet_address`, `accredited`, `created_at`

**`diplomas`** — `id`, `student_wallet`, `token_id`, `degree`, `metadata_hash`, `revoked`, `issued_at`

**`verification_logs`** — `id`, `employer`, `token_id`, `verified_at`

---

## 15. Metadata & Storage

### On-chain
- IPFS hash, credential hash, token ownership, revocation status

### Off-chain (IPFS)

```json
{
  "name": "Bachelor of Science",
  "description": "Official diploma NFT",
  "image": "ipfs://...",
  "attributes": [
    { "trait_type": "Institution", "value": "ABC University" },
    { "trait_type": "Major", "value": "Computer Science" }
  ]
}
```

### Batch Minting via CSV

```csv
student_wallet,degree,major
GABCD...,BSc,Computer Science
GXYZ...,MBA,Business
```

---

## 16. Verification Flow

```
Employer scans QR
       ↓
Frontend fetches token
       ↓
Backend checks Stellar
       ↓
Contract verifies:
  · token exists
  · not revoked
  · issuer approved
       ↓
Result displayed
```

Each diploma includes a unique verification URL embedded in the PDF, printed certificate, and student profile:

```
https://verify.project.com/token/2391
```

---

## 17. Security Features

| Measure | Description |
|---|---|
| **Role-Based Access** | Only accredited universities can mint |
| **Multi-Sig Wallets** | Protect university issuance authority |
| **Immutable Logs** | Every diploma permanently auditable |
| **Anti-Fraud Verification** | Detect fake issuers |

---

## 18. Advanced Features

| Feature | Description |
|---|---|
| **Zero-Knowledge Proofs** | Prove degree ownership or GPA range without revealing full transcript |
| **AI Fraud Detection** | Detect duplicate certificates, suspicious issuance, fake institutions |
| **Cross-Chain Credentials** | Mirror credentials on Ethereum, Polygon, Solana |
| **Resume Integration** | Generate verified CVs and LinkedIn integrations |

---

## 19. Development Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32v1-none

# Install Stellar CLI
cargo install stellar-cli

# Verify installation
stellar --version

# Initialize contract
stellar contract init diploma_contract

# Start local sandbox
stellar network sandbox start

# Deploy contract
stellar contract deploy \
  --wasm target/wasm32v1-none/release/diploma_contract.wasm \
  --source alice
```

---

## 20. Testing Strategy

```
tests/
├── mint.rs
├── revoke.rs
├── verification.rs
├── soulbound.rs
└── access_control.rs
```

| Layer | Tools |
|---|---|
| Smart Contracts | Rust built-in tests |
| Backend | Jest, Supertest |
| Frontend | Vitest, Playwright, Cypress |

---

## 21. DevOps & CI/CD

| Component | Platform |
|---|---|
| Frontend | Vercel |
| Backend | Railway / Fly.io |
| Database | Supabase / PostgreSQL |
| IPFS | Pinata |
| Monitoring | Grafana |

```
GitHub Actions → Run Tests → Build Contracts
       → Deploy to Testnet → Integration Tests → Deploy Production
```

---

## 22. MVP Roadmap

### Phase 1 — Foundation
- ✅ University registry
- ✅ Diploma minting
- ✅ Verification portal
- ✅ QR verification

### Phase 2 — Scale
- ✅ Batch issuance
- ✅ Revocation system
- ✅ Employer APIs
- ✅ Analytics dashboard

### Phase 3 — Advanced
- ✅ ZK proofs
- ✅ Cross-chain credentials
- ✅ Mobile app
- ✅ AI fraud detection

### Hackathon MVP (Minimum Viable Demo)
- ✅ University registration
- ✅ Soulbound diploma minting
- ✅ QR verification
- ✅ Employer verification page
- ✅ Revocation support

### 8-Week Build Plan

| Week | Focus |
|---|---|
| 1 | Soroban setup + university registry |
| 2 | Soulbound NFT contract |
| 3 | Verification APIs |
| 4 | Frontend dashboard |
| 5 | QR verification |
| 6 | Batch minting |
| 7 | Security audits |
| 8 | Testnet deployment |

---

## 23. Example User Flows

**University:** Login → Upload graduates CSV → Approve transaction → Diplomas minted → Students notified

**Student:** Connect wallet → View diploma NFT → Download PDF → Share verification link

**Employer:** Scan QR → Verify credential → Check accreditation → Download proof

---

## 24. Best Practices

- **Keep contracts modular** — avoid one giant contract
- **Minimize on-chain data** — store hashes only
- **Emit events** — `DiplomaIssued`, `DiplomaRevoked`, `UniversityApproved`
- **Use TTL extensions carefully** — prevent storage expiration
- **GDPR compliance** — avoid storing personal data directly on-chain
- **Require accreditation proof** before allowing institutions to mint

---

## 25. Future Expansion

- Government ID credentials
- Medical licenses & professional certifications
- NFT student IDs & scholarship records
- Academic reputation scores
- Decentralized academic transcripts
- Global education passport
