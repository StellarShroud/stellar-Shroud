# StellarShroud

> **Confidential Payments Layer for Stellar Anchors & Stablecoins**

StellarShroud is a privacy-preserving payment protocol for **Anchor-issued stablecoins and regulated digital assets on the Stellar network**.

It enables users to transfer supported assets without publicly exposing sensitive payment information while preserving an **opt-in compliance and auditor disclosure mechanism** for regulated anchors, institutions, and payment providers.

The goal is not to make payments completely opaque.

Instead, StellarShroud introduces **selective privacy**:

> **Private by default. Auditable when authorized.**

---

## Table of Contents

* [Overview](#overview)
* [Problem](#problem)
* [Vision](#vision)
* [Core Idea](#core-idea)
* [How StellarShroud Works](#how-stellarshroud-works)
* [Key Features](#key-features)
* [Architecture](#architecture)
* [Privacy Model](#privacy-model)
* [Auditor Disclosure](#auditor-disclosure)
* [Soroban Smart Contracts](#soroban-smart-contracts)
* [Cryptographic Design](#cryptographic-design)
* [Anchor Integration](#anchor-integration)
* [Transaction Flow](#transaction-flow)
* [Project Structure](#project-structure)
* [Technology Stack](#technology-stack)
* [Development Roadmap](#development-roadmap)
* [MVP](#mvp)
* [Security Considerations](#security-considerations)
* [Compliance Philosophy](#compliance-philosophy)
* [Testing Strategy](#testing-strategy)
* [Future Extensions](#future-extensions)
* [Success Criteria](#success-criteria)
* [Why StellarShroud](#why-stellarshroud)
* [Conclusion](#conclusion)

---

# Overview

StellarShroud is a **confidential payment primitive built for the Stellar ecosystem**.

Traditional Stellar transactions provide transparent settlement. This transparency is valuable for network verification, but it can become problematic for financial applications where transaction amounts, balances, counterparties, and payment relationships are commercially sensitive.

For example:

* A business paying employees may not want salaries publicly visible.
* A remittance provider may not want customer payment relationships exposed.
* An institution settling an RWA transaction may not want transaction amounts publicly observable.
* A merchant may not want competitors tracking its payment flows.
* An anchor may require authorized auditors to inspect transactions without making that information publicly available.

StellarShroud addresses this problem by introducing a privacy layer around supported Stellar assets.

The protocol combines:

* Zero-knowledge proofs
* Cryptographic commitments
* Nullifiers
* Merkle trees
* Shielded balances
* Selective disclosure
* Auditor authorization
* Soroban smart contracts

---

# Problem

Stellar provides fast and efficient asset settlement, but public blockchain transactions can expose information that financial institutions and users may consider sensitive.

A normal payment can reveal:

```text
Sender
   ↓
Recipient
   ↓
Asset
   ↓
Amount
   ↓
Ledger history
```

For certain financial applications, this creates several problems.

### 1. Financial Privacy

Public transaction histories can reveal:

* balances
* payment amounts
* counterparties
* transaction frequency
* business relationships
* treasury movements

---

### 2. Commercial Confidentiality

Businesses may not want competitors to observe:

```text
Company A → Supplier B
             $250,000
```

because transaction patterns can reveal supplier relationships, operating costs, or business activity.

---

### 3. Institutional Settlement

Institutions increasingly require privacy around:

* RWA settlement
* treasury management
* cross-border payments
* payroll
* institutional transfers

while still requiring a mechanism for compliance and auditing.

---

### 4. Privacy vs Compliance

Completely anonymous systems create regulatory challenges.

Completely transparent systems sacrifice financial privacy.

StellarShroud aims to provide a middle ground:

```text
                 ┌─────────────────┐
                 │  StellarShroud  │
                 └────────┬────────┘
                          │
             ┌────────────┴────────────┐
             │                         │
       User Privacy              Compliance
             │                         │
      Hidden payments        Authorized disclosure
```

---

# Vision

The long-term vision of StellarShroud is to become a **confidential transaction infrastructure layer for Stellar-based financial applications**.

Applications should be able to build:

* private stablecoin payments
* confidential payroll
* private remittances
* institutional settlements
* private treasury management
* confidential RWA settlement
* privacy-preserving merchant payments

without having to implement complex cryptography themselves.

---

# Core Idea

StellarShroud introduces a shielded transaction pool around supported Stellar assets.

Instead of directly exposing:

```text
Alice → Bob
1000 USDC
```

the protocol records a cryptographic commitment representing the transaction state.

Conceptually:

```text
Public Stellar Ledger
        │
        ▼
┌──────────────────────┐
│   StellarShroud      │
│                      │
│ Commitments          │
│ Nullifiers           │
│ Merkle Tree          │
│ ZK Proofs            │
└──────────┬───────────┘
           │
           ▼
    Shielded Assets
```

The network verifies that:

* the sender owns the funds
* the funds have not already been spent
* the transaction is valid
* the balance equation is correct
* the transaction satisfies protocol rules

without requiring the sensitive transaction details to be publicly revealed.

---

# How StellarShroud Works

A simplified shielded payment looks like:

```text
Alice
 │
 │ 1. Deposit stablecoin
 ▼
StellarShroud
 │
 │ 2. Create commitment
 ▼
Shielded Pool
 │
 │ 3. Generate ZK proof
 ▼
Soroban Contract
 │
 │ 4. Verify proof
 ▼
Shielded State
 │
 │
 └──────────────► Bob
                   │
                   │ 5. Spend/redeem
                   ▼
             Stellar Asset
```

---

# Key Features

## 1. Shielded Transfers

Users can transfer supported assets without publicly exposing:

* sender
* recipient
* amount

where the privacy model supports hiding those values.

---

## 2. Zero-Knowledge Proofs

Users generate proofs demonstrating that a transaction is valid without revealing the underlying private information.

A proof can establish:

> "I own a valid unspent note and I am authorized to spend it."

without revealing the note itself.

---

## 3. Cryptographic Commitments

Sensitive transaction information is represented using commitments.

Conceptually:

```text
Commitment = Hash(asset || amount || recipient || randomness)
```

The actual construction will use a cryptographic primitive suitable for the selected proof system rather than relying on a plain hash in production.

---

## 4. Nullifiers

Each shielded note generates a unique nullifier.

When the note is spent:

```text
Note
 │
 ▼
Nullifier
 │
 ▼
Spent
```

The nullifier prevents double spending without revealing which private note was consumed.

---

## 5. Merkle Tree

Shielded notes are organized into a Merkle tree.

```text
                    Root
                  /      \
                H1        H2
               /  \      /  \
             H3   H4   H5   H6
             │    │    │    │
            N1   N2   N3   N4
```

Users prove membership of their note using a Merkle path.

The Soroban contract only needs to maintain and verify the relevant commitment state.

---

# Privacy Model

StellarShroud follows a **selective privacy** model.

The protocol should not assume that every participant needs complete anonymity.

Instead:

### Public

Potentially visible:

* protocol state
* commitment root
* nullifiers
* proof verification results
* transaction metadata required by Stellar/Soroban

### Private

Potentially hidden:

* payment amount
* sender identity
* recipient identity
* note contents
* private balance information

### Authorized Disclosure

Potentially accessible to an authorized auditor:

* transaction details
* source of funds
* destination
* amount
* compliance metadata

subject to the protocol's disclosure policy.

---

# Auditor Disclosure

A core differentiator of StellarShroud is its **opt-in auditor disclosure mechanism**.

The objective is:

> Allow authorized entities to inspect specific transactions without making those transactions publicly transparent.

For example:

```text
User
 │
 │ Shielded Payment
 ▼
StellarShroud
 │
 │
 ├──────────────► Public observers
 │                  Cannot see private data
 │
 └──────────────► Authorized Auditor
                    Can decrypt disclosed data
```

---

## Auditor Keys

An anchor or regulated institution can register an auditor public key.

For example:

```text
Auditor Public Key
        │
        ▼
StellarShroud Registry
        │
        ▼
Authorized Disclosure
```

Sensitive transaction metadata can be encrypted specifically for the auditor.

The auditor uses its private key to decrypt the information.

---

## Important Design Principle

The auditor should **not** have unrestricted control over user funds.

Auditor capabilities should be limited to:

```text
READ / DISCLOSE
```

rather than:

```text
TRANSFER
SPEND
FREEZE
SEIZE
```

unless a separate application-level compliance system explicitly introduces such functionality.

This separation minimizes protocol-level trust.

---

# Soroban Smart Contracts

The core protocol will be implemented using **Soroban smart contracts written in Rust**.

The contracts are responsible for maintaining the cryptographic state and validating shielded transactions.

Potential contracts include:

```text
stellar-shroud/
│
├── shroud_pool
├── commitment_tree
├── nullifier_registry
├── asset_registry
└── auditor_registry
```

---

# Contract Responsibilities

## Shroud Pool

Responsible for:

* deposits
* withdrawals
* shielded transfers
* asset accounting
* commitment insertion

---

## Commitment Tree

Responsible for:

* storing commitment roots
* tracking tree state
* verifying membership paths

---

## Nullifier Registry

Responsible for:

* tracking spent notes
* preventing double spending

Conceptually:

```rust
if nullifier_exists(nullifier) {
    return Err(Error::AlreadySpent);
}
```

---

## Asset Registry

Tracks supported assets.

Example:

```text
Asset
 ├── Stellar Asset ID
 ├── Anchor
 ├── Asset Code
 └── Status
```

Only approved assets should be able to enter the shielded pool.

---

## Auditor Registry

Maintains authorized auditor configuration.

Potential information:

```text
Auditor ID
Public Key
Anchor
Status
Created At
```

---

# Cryptographic Design

The initial cryptographic architecture will use:

### Commitments

Commitments hide sensitive transaction information while allowing users to prove knowledge of the committed values.

Conceptually:

```text
C = Commit(value, randomness)
```

---

### Nullifiers

Nullifiers prevent the same shielded note from being spent twice.

```text
N = Nullifier(secret, note_id)
```

The exact construction will be finalized during the cryptographic design phase.

---

### Merkle Trees

Merkle trees allow users to prove that their commitment exists inside the current shielded state.

```text
Commitment
    │
    ▼
Merkle Leaf
    │
    ▼
Merkle Root
```

---

### Zero-Knowledge Proofs

The proof system must demonstrate statements such as:

```text
I know a valid note
AND
the note belongs to the current Merkle tree
AND
the note has not been spent
AND
the transaction values are valid
AND
the resulting commitments are correctly formed
```

without exposing the underlying secrets.

---

# Proof Circuit

A simplified circuit could verify:

```text
Private Inputs
──────────────

secret
amount
asset
recipient
randomness
Merkle path


Public Inputs
─────────────

Merkle root
nullifier
output commitment
asset identifier


Constraints
───────────

1. Note exists
2. Merkle path is valid
3. Nullifier is correctly derived
4. Note is unspent
5. Amount is valid
6. Output commitment is valid
7. Asset is supported
```

---

# Anchor Integration

StellarShroud is specifically designed around **anchor-issued stablecoins and regulated assets**.

An anchor can integrate StellarShroud by registering its supported asset.

Example:

```text
Anchor
  │
  │ issues
  ▼
USDC-like Stellar Asset
  │
  ▼
Stellar Network
  │
  ▼
StellarShroud
  │
  ▼
Shielded Transfers
```

---

# Anchor Workflow

A simplified anchor workflow:

```text
User
 │
 │ Deposit stablecoin
 ▼
Anchor / Stellar
 │
 ▼
StellarShroud
 │
 │ Shield
 ▼
Private Balance
```

The user can then transfer the shielded asset.

To exit:

```text
Shielded Balance
       │
       ▼
Generate Withdrawal Proof
       │
       ▼
Soroban Contract
       │
       ▼
Stellar Asset
       │
       ▼
Recipient
```

---

# Transaction Flow

## Deposit

```text
1. User holds Stellar asset.

2. User deposits asset into StellarShroud.

3. User generates a shielded note.

4. Commitment is inserted into the Merkle tree.

5. User receives ownership information privately.
```

---

## Private Transfer

```text
1. Alice selects a note.

2. Alice generates a ZK proof.

3. Alice computes a nullifier.

4. Alice creates Bob's output commitment.

5. Transaction is submitted.

6. Soroban verifies the proof.

7. Nullifier is marked as spent.

8. New commitment is inserted.
```

---

## Withdrawal

```text
1. Bob generates a withdrawal proof.

2. Contract verifies ownership.

3. Contract verifies the nullifier.

4. Shielded funds are released.

5. Stellar asset is transferred to Bob.
```

---

# Project Structure

The initial repository should be organized as:

```text
stellarShroud/
│
├── contracts/
│   │
│   ├── shroud_pool/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── storage.rs
│   │   │   ├── errors.rs
│   │   │   ├── events.rs
│   │   │   └── types.rs
│   │   └── Cargo.toml
│   │
│   ├── commitment_tree/
│   │   ├── src/
│   │   └── Cargo.toml
│   │
│   ├── nullifier_registry/
│   │   ├── src/
│   │   └── Cargo.toml
│   │
│   ├── asset_registry/
│   │   ├── src/
│   │   └── Cargo.toml
│   │
│   └── auditor_registry/
│       ├── src/
│       └── Cargo.toml
│
├── crypto/
│   │
│   ├── commitments/
│   ├── merkle/
│   ├── nullifiers/
│   ├── circuits/
│   └── proofs/
│
├── sdk/
│   └── rust/
│
├── frontend/
│   ├── src/
│   └── public/
│
├── tests/
│   ├── integration/
│   ├── contract/
│   └── crypto/
│
├── docs/
│   ├── architecture.md
│   ├── cryptography.md
│   ├── threat-model.md
│   └── auditor-disclosure.md
│
├── Cargo.toml
├── README.md
├── PROJECT.md
└── LICENSE
```

---

# Technology Stack

## Blockchain

* Stellar
* Soroban
* Stellar Assets

## Smart Contracts

* Rust
* Soroban SDK

## Cryptography

* Zero-knowledge proofs
* Cryptographic commitments
* Merkle trees
* Nullifiers
* Public/private key cryptography

## Backend

Rust

Potential components:

* transaction builder
* proof generation service
* anchor integration
* auditor service

## Frontend

Potential stack:

* React
* Next.js
* TypeScript
* Stellar wallet integration

---

# Development Roadmap

## Phase 0 — Research

**Goal:** Validate the architecture.

Tasks:

* [ ] Study Soroban storage and authorization
* [ ] Study Stellar asset authorization
* [ ] Study Stellar anchor architecture
* [ ] Select ZK proving system
* [ ] Define privacy model
* [ ] Define threat model
* [ ] Design commitment scheme
* [ ] Design nullifier scheme
* [ ] Design auditor disclosure model

Deliverable:

```text
Cryptographic & System Design Specification
```

---

# Phase 1 — Soroban Foundation

Build the basic protocol without ZK.

Tasks:

* [ ] Initialize Rust workspace
* [ ] Create Soroban contracts
* [ ] Implement asset registry
* [ ] Implement shielded pool
* [ ] Implement commitment storage
* [ ] Implement nullifier registry
* [ ] Implement deposit
* [ ] Implement withdrawal
* [ ] Implement events
* [ ] Write contract tests

Deliverable:

```text
Working shielded-pool prototype
```

---

# Phase 2 — Cryptographic Layer

Implement:

* [ ] Commitment scheme
* [ ] Nullifier generation
* [ ] Merkle tree
* [ ] Merkle membership proofs
* [ ] ZK circuit
* [ ] Proof generation
* [ ] Proof verification
* [ ] Negative test cases

Deliverable:

```text
Working ZK payment primitive
```

---

# Phase 3 — ZK + Soroban Integration

Connect the cryptographic system to Soroban.

Tasks:

* [ ] Submit proof to contract
* [ ] Verify proof on-chain
* [ ] Verify Merkle root
* [ ] Verify nullifier
* [ ] Insert output commitment
* [ ] Prevent double spending
* [ ] Test invalid proofs
* [ ] Test replay attacks

Deliverable:

```text
End-to-end private transfer
```

---

# Phase 4 — Auditor Disclosure

Implement selective disclosure.

Tasks:

* [ ] Auditor key generation
* [ ] Auditor registration
* [ ] Transaction encryption
* [ ] Disclosure metadata
* [ ] Auditor authorization
* [ ] Auditor decryption
* [ ] Disclosure events
* [ ] Revocation mechanism
* [ ] Audit logs

Deliverable:

```text
Private + auditable payment system
```

---

# Phase 5 — Anchor Integration

Build an anchor-facing integration layer.

Tasks:

* [ ] Asset registration
* [ ] Anchor configuration
* [ ] Supported asset verification
* [ ] Deposit integration
* [ ] Withdrawal integration
* [ ] Compliance metadata
* [ ] Auditor integration
* [ ] Test stablecoin flow

Deliverable:

```text
Anchor-compatible confidential payment protocol
```

---

# Phase 6 — SDK

Create a developer-friendly SDK.

Example:

```rust
let payment = ShroudPayment::new()
    .asset(asset)
    .amount(amount)
    .recipient(recipient)
    .build()?;

let proof = payment.generate_proof()?;

client.submit(proof).await?;
```

The goal is to hide cryptographic complexity from application developers.

---

# Phase 7 — Demo Application

Build a complete demonstration.

The demo should contain:

### User Wallet

* balance
* deposit
* shield
* send
* receive
* withdraw
* transaction history

### Anchor Dashboard

* supported assets
* shielded volume
* transactions
* compliance configuration

### Auditor Dashboard

* authorized transactions
* disclosure requests
* decrypted transaction information
* audit history

---

# MVP

The first working MVP should intentionally remain small.

### MVP Requirements

```text
Stellar Testnet
      │
      ▼
Supported Test Asset
      │
      ▼
Shield
      │
      ▼
Private Note
      │
      ▼
ZK Proof
      │
      ▼
Soroban Verification
      │
      ▼
Private Transfer
      │
      ▼
Withdraw
```

The MVP should demonstrate:

1. Deposit
2. Shield
3. Private transfer
4. ZK proof verification
5. Nullifier protection
6. Withdrawal
7. Auditor disclosure

---

# Security Considerations

Security is a primary requirement because StellarShroud handles financial assets.

The protocol must consider:

## Double Spending

Prevent reuse of shielded notes through nullifiers.

---

## Replay Attacks

Proofs must be bound to the appropriate:

* chain
* contract
* asset
* transaction state

---

## Merkle Root Manipulation

The contract must verify that the supplied root corresponds to an accepted shielded state.

---

## Proof Forgery

Invalid proofs must always fail verification.

---

## Auditor Key Compromise

If an auditor key is compromised, previously disclosed information could become exposed.

The system should therefore consider:

* key rotation
* key revocation
* encryption versioning
* scoped disclosure
* multi-auditor models

---

## Metadata Leakage

Even if transaction values are hidden, timing and transaction-level metadata can leak information.

The design should document this limitation explicitly.

---

# Compliance Philosophy

StellarShroud is **not intended to provide unrestricted anonymity**.

Its philosophy is:

```text
                 STELLARSHROUD
                       │
          ┌────────────┴────────────┐
          │                         │
       Privacy                 Compliance
          │                         │
   Hidden financial data     Authorized inspection
          │                         │
          └────────────┬────────────┘
                       │
                Selective Privacy
```

The protocol should give applications the ability to determine:

* who can inspect information
* what information can be disclosed
* when disclosure occurs
* how disclosure is authorized

without requiring every blockchain observer to see sensitive financial information.

---

# Threat Model

StellarShroud should assume that an attacker may:

* observe all public ledger transactions
* inspect contract state
* attempt to submit invalid proofs
* attempt double spending
* replay transactions
* compromise user devices
* compromise auditor keys
* attempt Merkle manipulation
* analyze transaction timing
* attempt denial-of-service attacks

The protocol must clearly distinguish between:

### Cryptographic Security

What the ZK system guarantees.

### Smart Contract Security

What Soroban guarantees through contract logic.

### Application Security

What the wallet/backend must protect.

### Operational Security

What anchors and auditors must protect.

---

# Testing Strategy

Testing should happen at multiple layers.

## Unit Tests

Test:

* commitments
* nullifiers
* Merkle trees
* proof inputs
* state transitions

---

## Contract Tests

Test:

* deposit
* withdrawal
* transfers
* invalid proofs
* duplicate nullifiers
* unauthorized auditors
* unsupported assets

---

## Integration Tests

Test:

```text
Wallet
   ↓
SDK
   ↓
Proof System
   ↓
Soroban
   ↓
Stellar Asset
```

---

## Security Tests

Include:

* replay attacks
* double-spending attempts
* malformed proofs
* invalid Merkle paths
* unauthorized disclosure
* compromised auditor scenarios

---

# Future Extensions

## Multi-Asset Shielded Pool

Support multiple Stellar assets.

```text
USDC
EURC
RWA Token
Other Anchor Assets
       │
       ▼
StellarShroud
```

---

## Private Payroll

Organizations can pay employees without exposing salary amounts publicly.

---

## Confidential RWA Settlement

Institutions can settle tokenized assets while protecting sensitive transaction information.

---

## Private Remittances

Cross-border payment providers can use StellarShroud for confidential settlement.

---

## Institutional Treasury

Institutions can manage treasury movements privately while maintaining controlled auditability.

---

## Multi-Auditor Disclosure

Allow different auditors to receive different levels of access.

Example:

```text
Regulator
   │
   ├── Transaction details
   │
Auditor
   │
   ├── Payment details
   │
Anchor
   │
   └── Compliance metadata
```

---

## Threshold Auditing

Instead of trusting a single auditor key:

```text
Auditor A
Auditor B
Auditor C
   │
   ▼
2-of-3 authorization
   │
   ▼
Disclosure
```

This could significantly reduce centralized trust.

---

# Developer Experience

StellarShroud should expose a simple API so developers do not need to understand the underlying cryptography.

For example:

```rust
let wallet = ShroudWallet::new(...)?;

wallet.deposit(asset, amount)?;

let payment = wallet
    .send(recipient)
    .amount(amount)
    .build()?;

let proof = wallet.generate_proof(payment)?;

wallet.submit(proof)?;
```

The SDK handles:

```text
Wallet
  │
  ├── Note management
  ├── Commitment generation
  ├── Merkle paths
  ├── Nullifiers
  ├── Proof generation
  └── Transaction construction
```

---

# Success Criteria

StellarShroud will be considered successful when the MVP can demonstrate:

### Technical

* [ ] Shielded Stellar asset deposits
* [ ] Private transfers
* [ ] ZK proof generation
* [ ] On-chain proof verification
* [ ] Double-spend prevention
* [ ] Shielded withdrawals
* [ ] Auditor disclosure

### Developer

* [ ] Rust SDK
* [ ] Clear documentation
* [ ] Example integration
* [ ] Test suite
* [ ] Local development environment

### Ecosystem

* [ ] Stellar Testnet deployment
* [ ] Example anchor integration
* [ ] Demonstration application
* [ ] Open-source repository
* [ ] Technical architecture documentation

---

# Why StellarShroud

StellarShroud is designed around a specific gap:

> **Financial applications need both privacy and accountability.**

Existing approaches often emphasize one side:

```text
Transparent Blockchain
        │
        └── Strong visibility
            Weak financial privacy


Anonymous System
        │
        └── Strong privacy
            Weak compliance


StellarShroud
        │
        └── Selective privacy
            + controlled disclosure
```

This makes StellarShroud particularly relevant to:

* Stellar anchors
* stablecoins
* cross-border payments
* tokenized real-world assets
* institutional settlement
* financial infrastructure
* payment providers

---

# Differentiation

StellarShroud should remain clearly differentiated from generic privacy protocols and other Stellar privacy projects.

Its primary focus is:

> **Confidential payments for Stellar-issued and anchor-issued financial assets with optional compliance disclosure.**

The combination of:

```text
Stellar Assets
       +
Soroban
       +
Zero Knowledge
       +
Shielded Payments
       +
Anchor Integration
       +
Selective Auditor Disclosure
```

forms the core identity of the project.

---

# Open Track Positioning

StellarShroud is well suited to an open-track ecosystem funding proposal because it combines:

### Originality

A privacy layer specifically designed around Stellar's payment and anchor ecosystem.

### Technical Depth

The project requires meaningful work across:

* Rust
* Soroban
* cryptography
* ZK circuits
* smart contracts
* SDK development
* frontend integration

### Ecosystem Relevance

The protocol targets:

* financial inclusion
* stablecoin payments
* cross-border settlement
* tokenization
* institutional infrastructure
* developer tooling

### Reusability

The final system should function as infrastructure rather than a single-purpose application.

Other Stellar applications could integrate the SDK and build their own confidential financial products.

---

# Project Milestones

| Milestone | Deliverable                   |
| --------- | ----------------------------- |
| M1        | Architecture + threat model   |
| M2        | Soroban shielded pool         |
| M3        | Commitment + nullifier system |
| M4        | Merkle tree                   |
| M5        | ZK circuit                    |
| M6        | On-chain proof verification   |
| M7        | Private transfer              |
| M8        | Auditor disclosure            |
| M9        | Anchor integration            |
| M10       | Rust SDK                      |
| M11       | Demo application              |
| M12       | Testnet deployment            |

---

# Final Architecture

The intended final architecture is:

```text
                         ┌───────────────────┐
                         │ Stellar Anchor    │
                         │ / Issuer          │
                         └─────────┬─────────┘
                                   │
                                   ▼
                         ┌───────────────────┐
                         │ Stellar Asset     │
                         └─────────┬─────────┘
                                   │
                                   ▼
┌──────────────┐        ┌───────────────────┐
│              │        │                   │
│    Wallet    │───────►│   StellarShroud   │
│              │        │                   │
└──────────────┘        │   Soroban Layer   │
                        │                   │
                        └─────────┬─────────┘
                                  │
              ┌───────────────────┼───────────────────┐
              │                   │                   │
              ▼                   ▼                   ▼
       ┌────────────┐      ┌────────────┐      ┌────────────┐
       │ Commitment │      │ Nullifier  │      │   Merkle   │
       │   State    │      │  Registry  │      │    Tree    │
       └────────────┘      └────────────┘      └────────────┘
                                  │
                                  ▼
                         ┌───────────────────┐
                         │  ZK Verification  │
                         └─────────┬─────────┘
                                   │
                                   ▼
                         ┌───────────────────┐
                         │ Auditor Registry  │
                         │ & Disclosure      │
                         └───────────────────┘
```

---

# Long-Term Vision

StellarShroud should evolve from a single privacy protocol into a reusable **confidential financial infrastructure layer for Stellar**.

The long-term goal is:

```text
                    Stellar
                       │
          ┌────────────┴────────────┐
          │                         │
      Transparent              Confidential
      Payments                  Payments
          │                         │
          │                  ┌──────┴──────┐
          │                  │ Stellar     │
          │                  │ Shroud      │
          │                  └──────┬──────┘
          │                         │
          │          ┌──────────────┼──────────────┐
          │          │              │              │
          ▼          ▼              ▼              ▼
       Payments   Stablecoins     RWA          Payroll
                                  Settlement
```

The objective is not to hide blockchain activity for its own sake.

The objective is to make **privacy a native feature of Stellar financial infrastructure while preserving controlled accountability where it is required.**

---

# License

The project should use an open-source license appropriate for the final repository and ecosystem requirements.

A permissive license such as MIT or Apache-2.0 can be considered for the SDK and application components, while cryptographic and contract components should be reviewed carefully before final licensing.

---

# Project Summary

**StellarShroud** is a privacy-preserving payment layer for Stellar anchor-issued stablecoins and financial assets.

It combines:

**Zero-Knowledge Proofs + Shielded Payments + Soroban + Stellar Assets + Selective Disclosure**

to enable a new category of Stellar applications:

> **Private financial transactions that remain auditable when properly authorized.**

The core principle is simple:

## **Private by Default. Auditable by Authorization. Built for Stellar.**
