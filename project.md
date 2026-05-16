Verifiable Diplomas (Soulbound NFTs) on Stellar

A decentralized credential verification platform where universities issue tamper-proof, non-transferable academic certificates as Soulbound NFTs using Stellar + Soroban smart contracts.

Students own their credentials permanently, employers can verify authenticity instantly, and institutions eliminate diploma fraud.

1. Project Overview
Core Idea

Universities issue:

Degree certificates
Diplomas
Professional certifications
Transcripts
Course completion badges

as non-transferable NFTs (Soulbound Tokens) on Stellar.

These credentials:

Cannot be sold or transferred
Are cryptographically verifiable
Exist permanently on-chain
Can be publicly verified by employers
Can be revoked by universities if necessary
2. Why Stellar + Soroban
Why Stellar
Fast Transactions

Credential issuance and verification happen instantly.

Very Low Fees

Perfect for universities issuing thousands of certificates.

Built-in Identity-Friendly Ecosystem

Stellar already supports compliance and institutional-grade systems.

Soroban Smart Contracts

Allows:

Soulbound NFT logic
Verification rules
Revocation systems
Role-based issuance
Metadata handling
3. Main Actors
Actor	Role
University	Issues diplomas
Student	Receives credential
Employer	Verifies diploma
Accreditation Body	Approves universities
Admin	System governance
4. Core Features
A. Soulbound Diploma NFTs

Non-transferable NFT certificates.

Each NFT contains:

{
  "student_name": "John Doe",
  "degree": "BSc Computer Science",
  "institution": "ABC University",
  "graduation_year": 2026,
  "gpa": "4.5",
  "certificate_hash": "QmX...",
  "issue_date": "2026-07-01"
}
B. Credential Verification Portal

Employers can:

Enter wallet address
Scan QR code
Check diploma authenticity
Verify issuer accreditation
C. Revocation System

Universities can revoke:

Fraudulent diplomas
Administrative mistakes
Suspended credentials
D. Multi-University Support

Platform supports:

Universities
Bootcamps
Online academies
Government institutions
E. Privacy Layer

Sensitive data stored off-chain.

On-chain stores:

Hashes
References
Proofs

Off-chain stores:

Full transcript PDFs
Metadata
Student documents
5. System Architecture
High-Level Architecture
                    ┌────────────────────┐
                    │  Frontend Portal   │
                    └─────────┬──────────┘
                              │
               ┌──────────────┴──────────────┐
               │                             │
      ┌────────▼────────┐         ┌─────────▼────────┐
      │  Backend API    │         │ Verification API │
      └────────┬────────┘         └─────────┬────────┘
               │                            │
               │                            │
       ┌───────▼────────────────────────────▼──────┐
       │        Soroban Smart Contracts             │
       │                                            │
       │  - Diploma NFT Contract                    │
       │  - University Registry                     │
       │  - Revocation Contract                     │
       └────────────────────────────────────────────┘
                              │
                     ┌────────▼────────┐
                     │ Stellar Network │
                     └─────────────────┘
6. Smart Contract Architecture
Main Contracts
Contract	Purpose
diploma_contract	Soulbound NFT issuance
university_registry	Approved institutions
revocation_contract	Revoked certificates
access_control	Role permissions
metadata_registry	Credential metadata
7. Recommended Tech Stack
Blockchain
Component	Technology
Blockchain	Stellar
Smart Contracts	Soroban SDK (Rust)
Wallet	Freighter
Storage	IPFS
Indexing	Stellar RPC/Horizon
Backend
Component	Tech
API	Node.js / NestJS
Database	PostgreSQL
Queue	Redis/BullMQ
Authentication	JWT/OAuth
File Storage	IPFS/Filecoin
Frontend
Component	Tech
Framework	Next.js
Styling	TailwindCSS
Wallet Integration	Freighter API
State Management	Zustand
QR Verification	qrcode.js
8. Full Project Folder Structure
Monorepo Structure
stellar-diploma-platform/
│
├── apps/
│   ├── frontend/
│   ├── backend/
│   ├── verifier-portal/
│   └── admin-dashboard/
│
├── contracts/
│   ├── diploma_contract/
│   ├── university_registry/
│   ├── revocation_contract/
│   ├── access_control/
│   └── shared/
│
├── packages/
│   ├── sdk/
│   ├── ui/
│   ├── types/
│   └── config/
│
├── infrastructure/
│   ├── docker/
│   ├── kubernetes/
│   ├── terraform/
│   └── monitoring/
│
├── scripts/
│   ├── deploy/
│   ├── seed/
│   └── migration/
│
├── docs/
├── tests/
├── .github/
└── README.md
9. Smart Contract Structure
contracts/diploma_contract/
│
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
│
├── Cargo.toml
└── Makefile
10. Soulbound NFT Logic
Core Rules
Allowed

✅ Mint diploma
✅ Verify diploma
✅ Revoke diploma
✅ Read metadata

Forbidden

❌ Transfer NFT
❌ Sell NFT
❌ Approve NFT transfers

11. Contract Data Model
Diploma Struct
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
12. Important Contract Functions
A. Issue Diploma
fn mint_diploma(
    env: Env,
    student: Address,
    degree: Symbol,
    metadata_uri: String
)
B. Verify Diploma
fn verify_diploma(
    env: Env,
    token_id: u64
) -> bool
C. Revoke Diploma
fn revoke_diploma(
    env: Env,
    token_id: u64
)
D. Prevent Transfer
fn transfer(...) {
    panic!("Soulbound token: transfer disabled");
}
13. University Registry Contract

Maintains approved institutions.

Features
Add university
Remove university
Accreditation status
Public verification
14. Backend Architecture
Backend Responsibilities
Service	Purpose
Auth Service	University login
Diploma Service	Issue certificates
Verification Service	Public verification
Metadata Service	IPFS uploads
Notification Service	Email students
Backend Structure
apps/backend/
│
├── src/
│   ├── modules/
│   │   ├── auth/
│   │   ├── diploma/
│   │   ├── university/
│   │   ├── verification/
│   │   └── notifications/
│   │
│   ├── blockchain/
│   ├── database/
│   ├── config/
│   └── utils/
│
├── prisma/
└── package.json
15. Frontend Pages
Student Portal
/student
├── dashboard
├── my-certificates
├── wallet-connect
└── profile
Employer Verification Portal
/verify
├── qr-scan
├── search
├── result
└── accreditation
University Dashboard
/university
├── issue-diploma
├── batch-upload
├── revoke
├── analytics
└── settings
16. Database Schema
PostgreSQL Tables
universities
id
name
wallet_address
accredited
created_at
diplomas
id
student_wallet
token_id
degree
metadata_hash
revoked
issued_at
verification_logs
id
employer
token_id
verified_at
17. Metadata Storage
Recommended Approach
On-chain

Store:

IPFS hash
Credential hash
Token ownership
Revocation status
Off-chain

Store:

PDF diploma
Student photo
Transcript
Additional metadata
18. IPFS Structure
{
  "name": "Bachelor of Science",
  "description": "Official diploma NFT",
  "image": "ipfs://...",
  "attributes": [
    {
      "trait_type": "Institution",
      "value": "ABC University"
    },
    {
      "trait_type": "Major",
      "value": "Computer Science"
    }
  ]
}
19. Verification Flow
Employer Verification Process
Employer scans QR
        ↓
Frontend fetches token
        ↓
Backend checks Stellar
        ↓
Contract verifies:
    - token exists
    - not revoked
    - issuer approved
        ↓
Result displayed
20. QR Code System

Each diploma includes:

https://verify.project.com/token/2391

QR embedded in:

PDF diploma
Printed certificate
Student profile
21. Security Features
Critical Security Measures
Role-Based Access

Only accredited universities can mint.

Multi-Sig University Wallets

Protect issuance authority.

Immutable Issuance Logs

Every diploma permanently auditable.

Anti-Fraud Verification

Detect fake issuers.

22. Advanced Features
A. Zero-Knowledge Proof Verification

Students prove:

Degree ownership
GPA range
Graduation status

without revealing full transcript.

B. AI Fraud Detection

Detect:

Duplicate certificates
Suspicious issuance
Fake institutions
C. Cross-Chain Credentials

Mirror credentials on:

Ethereum
Polygon
Solana
D. Resume Integration

Generate:

Verified CV
LinkedIn integrations
On-chain reputation
23. Batch Diploma Minting

Universities upload CSV:

student_wallet,degree,major
GABCD...,BSc,Computer Science
GXYZ...,MBA,Business

Backend automatically:

Uploads metadata
Mints NFTs
Emails students
24. Soroban Development Setup
Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
Add WASM Target
rustup target add wasm32v1-none
Install Stellar CLI
cargo install stellar-cli
Verify
stellar --version
25. Create Contract
stellar contract init diploma_contract
26. Local Development
Start Local Sandbox
stellar network sandbox start
Deploy Contract
stellar contract deploy \
  --wasm target/wasm32v1-none/release/diploma_contract.wasm \
  --source alice
27. Testing Strategy
Smart Contract Tests
tests/
├── mint.rs
├── revoke.rs
├── verification.rs
├── soulbound.rs
└── access_control.rs
Frontend Tests

Use:

Vitest
Playwright
Cypress
Backend Tests

Use:

Jest
Supertest
28. DevOps Architecture
Recommended Deployment
Component	Platform
Frontend	Vercel
Backend	Railway/Fly.io
Database	Supabase/Postgres
IPFS	Pinata
Monitoring	Grafana
29. CI/CD Pipeline
GitHub Actions
    ↓
Run Tests
    ↓
Build Contracts
    ↓
Deploy to Testnet
    ↓
Run Integration Tests
    ↓
Deploy Production
30. Token Economics (Optional)

You can introduce:

Token Utility	Purpose
Verification Fees	Paid by employers
University Staking	Prevent spam issuers
Governance	Community voting
31. Revenue Model
Monetization
Source	Example
University subscriptions	Annual licensing
Verification API	Employer access
Premium analytics	Accreditation insights
White-label solutions	Private university deployments
32. Compliance Considerations
Important Legal Areas
GDPR/Data Privacy

Avoid storing personal student data directly on-chain.

Accreditation Validation

Require proof before allowing institutions.

Educational Regulations

Different countries have different requirements.

33. MVP Roadmap
Phase 1

✅ University registry
✅ Diploma minting
✅ Verification portal
✅ QR verification

Phase 2

✅ Batch issuance
✅ Revocation system
✅ Employer APIs
✅ Analytics dashboard

Phase 3

✅ ZK proofs
✅ Cross-chain credentials
✅ Mobile app
✅ AI fraud detection

34. Example User Flow
University
Login
 → Upload graduates CSV
 → Approve transaction
 → Diplomas minted
 → Students notified
Student
Connect wallet
 → View diploma NFT
 → Download PDF
 → Share verification link
Employer
Scan QR
 → Verify credential
 → Check accreditation
 → Download proof
35. Best Soroban Design Practices
Keep Contracts Modular

Avoid one giant contract.

Minimize On-Chain Data

Store hashes only.

Emit Events
DiplomaIssued
DiplomaRevoked
UniversityApproved
Use TTL Extensions Carefully

Prevent storage expiration.

36. Future Expansion Ideas
Government ID credentials
Medical licenses
Professional certifications
NFT student IDs
Scholarship records
Academic reputation scores
Decentralized academic transcripts
Global education passport
37. Final Recommended Architecture
Frontend (Next.js)
        ↓
Backend API (NestJS)
        ↓
Soroban SDK Client
        ↓
Soroban Smart Contracts
        ↓
Stellar Network
        ↓
IPFS/Filecoin Storage
38. Recommended Development Order
Step-by-Step Build Plan
Week 1
Setup Soroban
Create university registry
Week 2
Build Soulbound NFT contract
Week 3
Build verification APIs
Week 4
Create frontend dashboard
Week 5
Add QR verification
Week 6
Add batch minting
Week 7
Security audits
Week 8
Testnet deployment
39. Ideal Hackathon MVP

For a hackathon, build:

✅ University registration
✅ Soulbound diploma minting
✅ QR verification
✅ Employer verification page
✅ Revocation support

This is enough for:

Demo
Investors
Pilot universities
Real-world adoption
