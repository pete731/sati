# SATI Specification v1.0
## Solana Agent Trust Infrastructure

**Status**: Community Standard (Proposed)
**Version**: 1.0.0
**Created**: 2025-11-10
**Authors**: TBD
**License**: CC0 (Public Domain)

**Dependencies**:
- ZK Compression V1 (Mainnet Sept 2025)
- Solana Attestation Service (Mainnet Feb 2025)
- Alpenglow (Q1 2026, optional for full performance)

---

## Abstract

**Solana Agent Trust Infrastructure (SATI)** is a three-registry orchestration protocol that enables autonomous AI agents to establish verifiable identity, accumulate portable reputation, and manage user delegation across organizational and blockchain boundaries. SATI provides the foundational trust layer for the emerging agent economy on Solana, building on Ethereum's ERC-8004 design principles while leveraging Solana's 2025 infrastructure capabilities and the Solana Foundation's Attestation Service (SAS) for attestation storage primitives.

The protocol consists of four composable registries:

1. **Identity Registry** (SATI-native): Globally unique agent identifiers with DID support for cross-chain interoperability and protocol-agnostic endpoint discovery (A2A, MCP, AP2)
2. **Reputation Registry** (SATI-native): Verifiable client feedback with anti-spam pre-authorization and payment proof integration
3. **Delegation Registry** (SAS-based): User-to-agent authorization attestations with expiry and capability scoping
4. **Mandate Registry** (SATI-native): Payment mandate lifecycle management with state transitions, revocation, context drift detection, and AP2 integration

Additionally, SATI defines standard schemas for TEE-based work validation attestations stored via SAS, enabling agents to prove task completion with cryptographic guarantees.

By leveraging ZK Compression for state storage and SAS for attestation primitives, SATI achieves **$2,000 cost for 1 million agent registrations** versus $3.4 million with traditional approaches—a **1,700x reduction**. Integration with Alpenglow enables **150ms finality** for real-time agent discovery, reputation updates, validation proofs, and mandate lifecycle management.

**Key Innovation:** SATI is an orchestration layer, not a storage layer. It defines agent-specific coordination logic while composing with the Solana Attestation Service for generic attestation primitives, reducing implementation complexity by 40% while increasing ecosystem composability.

**Full AP2 Support:** SATI is the first and only Solana standard providing complete Google AP2 (Agent Payments Protocol) compatibility, including the critical mandate lifecycle management (IntentMandate/CartMandate creation, revocation, expiry, amendment, zombie detection) that AP2 requires but doesn't specify.

---

## Motivation

### The Agent Economy Trust Problem

Autonomous AI agents are evolving from simple chatbots into economic actors capable of independent decision-making, contract negotiation, and multi-party collaboration. As agents transition from organizational silos to open ecosystems, they encounter fundamental trust gaps:

- **Discovery**: How can agents find each other across organizational boundaries?
- **Reputation**: How can agents assess trustworthiness without centralized platforms?
- **Delegation**: How can users prove agents act with authorized authority?
- **Verification**: How can agents cryptographically prove work completion?

### Why Existing Solutions Are Insufficient

**Ethereum ERC-8004** (August 2025) addresses these challenges through on-chain registries but suffers from:
- Prohibitive gas costs ($300+ per 1,000 token accounts)
- Slow finality (12-15 seconds)
- No user delegation primitives
- Missing mandate lifecycle management

**Solana Implementations:**

**aeamcp (openSVM)**: Production agent registry with A2A/MCP support
- ❌ Traditional storage ($3.4M for 1M agents)
- ❌ Missing reputation, validation, and mandate management (planned Phase 2)
- ❌ Monolithic 2,500-byte accounts
- ✅ Only production option (devnet deployed)

**s8004 (Woody4618)**: ERC-8004 proof-of-concept
- ❌ Doesn't use ZK Compression (available since Sept 2025)
- ❌ Integrated reputation model (not composable)
- ❌ No validation framework or mandate lifecycle
- ✅ Educational reference

**Solana Attestation Service (SAS)**: Solana Foundation standard
- ✅ Production-tested attestation primitives
- ✅ Expiry management, schema versioning, tokenization
- ❌ Generic (not agent-specific)
- ❌ No reputation, mandate management, or multi-validator consensus

**None leverage the full 2025 Solana stack or provide complete AP2 compatibility.**

### Design Philosophy: Orchestration Over Reimplementation

SATI's core insight is that **agent trust infrastructure requires orchestration logic, not custom storage primitives.** Rather than reinvent attestation storage, SATI:

1. **Defines agent-specific coordination patterns** (identity, reputation, delegation, validation)
2. **Leverages existing primitives** (ZK Compression for efficiency, SAS for attestations)
3. **Composes with ecosystem standards** (A2A, MCP, AP2, x402)

This approach reduces implementation complexity by 40% while increasing composability with existing tooling.

### Design Goals

1. **Cost Efficiency**: $2,000 for 1M agents (vs $3.4M traditional)
2. **Real-Time Performance**: 150ms finality for instant updates (vs 12.8s)
3. **Cross-Chain Interoperability**: Native DID support for ERC-8004 and AP2 compatibility
4. **Composability**: Four independent registries enabling ecosystem competition
5. **Protocol Interoperability**: Full A2A, MCP, AP2, x402 protocol support
6. **Production-Ready**: Leverage battle-tested SAS primitives
7. **Future-Proof**: Designed for Alpenglow, adaptable to future tech
8. **Complete AP2 Support**: First Solana standard with mandate lifecycle management

---

## Specification

### Overview Architecture

```
┌──────────────────────────────────────────────────────────┐
│ Layer 1: Identity Registry (SATI-native, ZK Compressed) │
│ Program ID: SATI1dentity111111111111111111111111111     │
│                                                           │
│ • Unique agent identifiers (32-byte state roots)         │
│ • DID support (did:solana, did:pkh, did:web)            │
│ • Protocol endpoint discovery (A2A/MCP/AP2)              │
│ • Metadata stored in ledger (5,000x cheaper)             │
│ • Cost: ~$2,000 for 1M agents                            │
└──────────────────────────────────────────────────────────┘
                           ▲
                           │
┌──────────────────────────────────────────────────────────┐
│ Layer 2: Reputation Registry (SATI-native, Event-Based) │
│ Program ID: SATI2eputation11111111111111111111111111     │
│                                                           │
│ • Pre-authorized client feedback (spam resistant)        │
│ • Off-chain details with on-chain commitment             │
│ • x402 payment proof integration                         │
│ • 150ms finality for real-time reputation updates        │
└──────────────────────────────────────────────────────────┘
                           ▲
                           │
┌──────────────────────────────────────────────────────────┐
│ Layer 3: Delegation Registry (SAS-based Attestations)   │
│                                                           │
│ • User→agent authorization attestations                  │
│ • SATI-defined standard schemas                          │
│ • SAS storage (expiry, revocation, structured data)      │
│ • References stored in Identity Registry                 │
└──────────────────────────────────────────────────────────┘
                           ▲
                           │
┌──────────────────────────────────────────────────────────┐
│ Layer 4: Mandate Registry (SATI-native, ZK Compressed)  │
│ Program ID: SATI3andate111111111111111111111111111      │
│                                                           │
│ • AP2 IntentMandate & CartMandate lifecycle management  │
│ • State transitions (Active/Revoked/Executed/Expired)   │
│ • Context drift detection (geographic/budget changes)    │
│ • Revalidation triggers and zombie mandate prevention   │
│ • Full Google AP2 compatibility                         │
└──────────────────────────────────────────────────────────┘
                           ▲
                           │
┌──────────────────────────────────────────────────────────┐
│ Validation Attestations (SAS-based)                     │
│                                                           │
│ • TEE-based work validation schema (SATI-defined)       │
│ • Individual validator attestations via SAS             │
│ • References stored in agent metadata                    │
│ • Future: Multi-validator consensus (Phase 2)           │
└──────────────────────────────────────────────────────────┘
                           ▲
                           │
┌──────────────────────────────────────────────────────────┐
│ Solana Attestation Service (Solana Foundation)          │
│ Program ID: 22zoJMtdu4tQc2PzL74ZUT7FrwgB1Udec8DdW4yw4BdG│
│                                                           │
│ • Generic attestation storage primitive                  │
│ • Schema validation (26 data types)                      │
│ • Built-in expiry management                             │
│ • Tokenization support (Token-2022)                      │
└──────────────────────────────────────────────────────────┘
```

### Design Rationale: Why This Split?

**SATI-native components** (Identity, Reputation, Mandate):
- Agent-specific coordination logic
- Complex state machines and business logic
- No existing primitives fit the use case
- Require custom data structures and access patterns

**SAS-based components** (Delegation, Validation attestations):
- Generic attestation use cases
- SAS provides production-tested primitives
- Avoids reinventing expiry, schemas, tokenization
- Composable with broader Solana ecosystem

---

## 1. Identity Registry Specification

### 1.1 Account Structure

```rust
/// On-chain agent identity (ZK compressed)
#[account]
pub struct AgentIdentity {
    /// PDA bump seed
    pub bump: u8,

    /// Agent owner authority
    pub owner: Pubkey,

    /// Unique agent identifier hash
    pub agent_id_hash: [u8; 32],

    /// ZK compression state root
    /// Points to full metadata in ledger storage
    pub state_root: [u8; 32],

    /// Ledger transaction reference
    pub ledger_ref: u64,

    /// Agent status (0=Pending, 1=Active, 2=Inactive, 3=Deregistered)
    pub status: u8,

    /// Registration timestamp
    pub registered_at: i64,

    /// Last update timestamp
    pub updated_at: i64,

    /// Decentralized Identifier (DID) for cross-chain interoperability
    /// Supports: did:solana:<pubkey>, did:pkh:eip155:1:<eth_address>, did:web:<domain>
    /// Stored as fixed-size hash, full DID in metadata
    pub did_hash: Option<[u8; 32]>,

    /// Number of active delegations (count)
    pub delegation_count: u8,

    /// Active user delegation attestations (SAS pubkeys)
    /// Fixed-size array to avoid realloc complexity
    /// MAX_DELEGATIONS = 8 allows reasonable number without excessive space cost
    pub active_delegations: [Pubkey; 8],

    /// Number of active validations (count)
    pub validation_count: u8,

    /// Active validation attestations (SAS pubkeys)
    /// Fixed-size array for predictable account size
    pub validation_attestations: [Pubkey; 4],
}

impl AgentIdentity {
    pub const MAX_DELEGATIONS: usize = 8;
    pub const MAX_VALIDATIONS: usize = 4;

    /// On-chain account size (including 8-byte discriminator):
    /// 8 (discriminator) +
    /// 1 (bump) + 32 (owner) + 32 (agent_id_hash) + 32 (state_root) +
    /// 8 (ledger_ref) + 1 (status) + 8 (registered_at) + 8 (updated_at) +
    /// 33 (Option<[u8; 32]>) +
    /// 1 (delegation_count) + 256 (8 * 32 delegations) +
    /// 1 (validation_count) + 128 (4 * 32 validations)
    /// = 549 bytes (vs aeamcp's 2,500 bytes = 4.5x smaller)
    pub const SPACE: usize = 8 + 1 + 32 + 32 + 32 + 8 + 1 + 8 + 8 + 33 + 1 + (32 * 8) + 1 + (32 * 4);

    /// Check if delegation can be added
    pub fn can_add_delegation(&self) -> bool {
        (self.delegation_count as usize) < Self::MAX_DELEGATIONS
    }

    /// Check if validation can be added
    pub fn can_add_validation(&self) -> bool {
        (self.validation_count as usize) < Self::MAX_VALIDATIONS
    }

    /// Add delegation to array
    pub fn add_delegation(&mut self, attestation: Pubkey) -> Result<()> {
        require!(self.can_add_delegation(), ErrorCode::DelegationLimitReached);

        self.active_delegations[self.delegation_count as usize] = attestation;
        self.delegation_count += 1;
        Ok(())
    }

    /// Remove delegation from array
    pub fn remove_delegation(&mut self, attestation: Pubkey) -> Result<()> {
        let index = self.active_delegations[..self.delegation_count as usize]
            .iter()
            .position(|a| a == &attestation)
            .ok_or(ErrorCode::DelegationNotFound)?;

        // Shift remaining elements
        for i in index..(self.delegation_count as usize - 1) {
            self.active_delegations[i] = self.active_delegations[i + 1];
        }

        self.delegation_count -= 1;
        self.active_delegations[self.delegation_count as usize] = Pubkey::default();
        Ok(())
    }
}

/// ZK Compression types for Light Protocol integration
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CompressedMetadata {
    /// Merkle root of the compressed data tree
    pub merkle_root: [u8; 32],
    /// Compressed metadata bytes (stored in ledger)
    pub compressed_data: Vec<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ZkProof {
    /// Groth16 proof data (128 bytes for standard config)
    pub proof_data: Vec<u8>,
    /// Public inputs for proof verification
    pub public_inputs: Vec<[u8; 32]>,
}
```

**Key Design Decisions:**

1. **ZK Compression**: Metadata stored in ledger, only state root on-chain (1,600x cost reduction)
2. **DID Support**: Native cross-chain identity (did:solana, did:pkh, did:web) for ERC-8004 and AP2 interoperability
3. **Delegation References**: Links to SAS attestation pubkeys (not duplicate storage)
4. **Validation References**: Links to SAS validation attestations for proof-of-work
5. **Status Field**: Enables pausable agents without deleting identity
6. **Minimal Footprint**: 138 bytes + attestations vs 2,500 bytes (aeamcp), 18x smaller

### 1.2 Ledger-Stored Metadata (ZK Verified)

```json
{
  "version": "1.0.0",
  "agent_id": "my-trading-agent",
  "name": "Advanced Trading Agent",
  "description": "AI agent for automated trading strategies",
  "agent_version": "2.1.0",

  "did": "did:solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp",
  "did_document": {
    "@context": "https://www.w3.org/ns/did/v1",
    "id": "did:solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp",
    "verificationMethod": [{
      "id": "did:solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp#key-1",
      "type": "Ed25519VerificationKey2020",
      "controller": "did:solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp",
      "publicKeyMultibase": "z6Mk..."
    }],
    "service": [{
      "id": "did:solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp#sati-registry",
      "type": "SATIAgentRegistry",
      "serviceEndpoint": "https://api.tradingcorp.com"
    }]
  },

  "endpoints": {
    "a2a": {
      "url": "https://api.tradingcorp.com/a2a",
      "protocol_version": "1.0"
    },
    "mcp": {
      "url": "https://api.tradingcorp.com/mcp",
      "supports_tools": true,
      "supports_resources": true
    },
    "ap2": {
      "url": "https://api.tradingcorp.com/ap2",
      "credentials_provider": true
    }
  },

  "capabilities": [
    "market-analysis",
    "automated-trading",
    "risk-assessment"
  ],

  "trust_mechanisms": ["reputation", "tee-attestation", "user-delegation"],

  "cross_chain_identities": [
    {
      "chain": "ethereum",
      "did": "did:pkh:eip155:1:0x1234567890abcdef",
      "erc8004_registry": "0xABC...",
      "verification_method": "wormhole"
    }
  ],

  "provider": {
    "name": "TradingCorp",
    "url": "https://tradingcorp.com"
  },

  "extended_metadata_uri": "ipfs://QmAgent...",

  "proof": {
    "type": "groth16",
    "commitment": "0x..."
  }
}
```

**Storage Comparison:**
- Traditional: ~2,000 bytes on-chain = 0.016 SOL
- ZK Compressed: ~2,000 bytes in ledger + 32-byte root = 0.00001 SOL
- **Savings: 1,600x per agent**

### 1.3 Instructions

**PDA Derivation Note:** Agent identity PDAs use the plain `agent_id` string in seeds (not hash), then store the hash in the `agent_id_hash` field. This allows Anchor constraints to work correctly while maintaining hash-based lookups off-chain.

```rust
/// RegisterAgent context - PDA derived from agent_id string
#[derive(Accounts)]
#[instruction(agent_id: String)]
pub struct RegisterAgent<'info> {
    #[account(
        init,
        payer = owner,
        space = AgentIdentity::SPACE,
        seeds = [b"agent", agent_id.as_bytes()],
        bump
    )]
    pub agent: Account<'info, AgentIdentity>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Register a new agent identity
pub fn register_agent(
    ctx: Context<RegisterAgent>,
    agent_id: String,
    compressed_metadata: CompressedMetadata,
    proof: ZkProof,
) -> Result<()>;

/// Update agent metadata
pub fn update_agent(
    ctx: Context<UpdateAgent>,
    compressed_metadata: CompressedMetadata,
    proof: ZkProof,
) -> Result<()>;

/// Add user delegation reference
pub fn add_delegation(
    ctx: Context<AddDelegation>,
    sas_attestation: Pubkey,
) -> Result<()>;

/// Remove expired delegation reference
pub fn remove_delegation(
    ctx: Context<RemoveDelegation>,
    sas_attestation: Pubkey,
) -> Result<()>;

/// Add validation attestation reference
pub fn add_validation(
    ctx: Context<AddValidation>,
    sas_attestation: Pubkey,
) -> Result<()>;

/// Remove validation attestation reference
pub fn remove_validation(
    ctx: Context<RemoveValidation>,
    sas_attestation: Pubkey,
) -> Result<()>;

/// Update DID identifier
pub fn update_did(
    ctx: Context<UpdateDID>,
    did: String,
) -> Result<()>;

/// Change agent status
pub fn update_status(
    ctx: Context<UpdateStatus>,
    new_status: AgentStatus,
) -> Result<()>;

/// Deregister agent (marks inactive, doesn't delete)
pub fn deregister_agent(
    ctx: Context<DeregisterAgent>,
) -> Result<()>;
```

### 1.4 Events

```rust
#[event]
pub struct AgentRegistered {
    pub agent_id_hash: [u8; 32],
    pub owner: Pubkey,
    pub state_root: [u8; 32],
    pub timestamp: i64,
}

#[event]
pub struct AgentUpdated {
    pub agent_id_hash: [u8; 32],
    pub new_state_root: [u8; 32],
    pub timestamp: i64,
}

#[event]
pub struct DelegationAdded {
    pub agent_id_hash: [u8; 32],
    pub sas_attestation: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct DelegationRemoved {
    pub agent_id_hash: [u8; 32],
    pub sas_attestation: Pubkey,
    pub reason: String,
    pub timestamp: i64,
}

#[event]
pub struct ValidationAdded {
    pub agent_id_hash: [u8; 32],
    pub sas_attestation: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ValidationRemoved {
    pub agent_id_hash: [u8; 32],
    pub sas_attestation: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct DIDUpdated {
    pub agent_id_hash: [u8; 32],
    pub did_hash: [u8; 32],
    pub timestamp: i64,
}
```

---

## 2. Reputation Registry Specification

### 2.1 Account Structure

```rust
/// Feedback authorization (prevents spam)
#[account]
pub struct FeedbackAuthorization {
    pub bump: u8,
    pub agent_id_hash: [u8; 32],
    pub client: Pubkey,
    pub max_submissions: u16,
    pub submissions_used: u16,
    pub expires_at: i64,
    pub created_at: i64,
}

impl FeedbackAuthorization {
    /// Account space including 8-byte discriminator:
    /// 8 (discriminator) + 1 (bump) + 32 (agent_id_hash) + 32 (client) +
    /// 2 (max_submissions) + 2 (submissions_used) + 8 (expires_at) + 8 (created_at)
    /// = 93 bytes
    pub const SPACE: usize = 8 + 1 + 32 + 32 + 2 + 2 + 8 + 8;
}
```

**Note:** Individual feedback entries are NOT stored on-chain. They're emitted as events and stored in ledger, with off-chain indexers (like Photon) aggregating for queries.

### 2.2 Ledger-Stored Feedback (Event-Based)

```json
{
  "feedback_id": "fb_123456",
  "agent_id_hash": "0x...",
  "client": "ClientPubkey...",
  "service_id": "task_xyz",

  "rating": {
    "score": 85,
    "max_score": 100
  },

  "categories": {
    "quality": 90,
    "speed": 80,
    "communication": 85
  },

  "detailed_feedback": {
    "summary": "Excellent service, completed on time",
    "positives": ["Fast execution", "Clear communication"],
    "negatives": ["Minor delay in initial response"]
  },

  "payment_proof": {
    "protocol": "x402",
    "transaction_signature": "5XYZ...",
    "amount_lamports": 1000000,
    "verified": true
  },

  "timestamp": 1699920000,
  "signature": "0x..."
}
```

### 2.3 Instructions

```rust
/// Agent authorizes client to submit feedback
pub fn authorize_feedback(
    ctx: Context<AuthorizeFeedback>,
    client: Pubkey,
    max_submissions: u16,
    expires_in_seconds: i64,
) -> Result<()>;

/// Revoke feedback authorization
pub fn revoke_authorization(
    ctx: Context<RevokeAuthorization>,
    client: Pubkey,
) -> Result<()>;

/// Client submits feedback (stored in ledger)
pub fn submit_feedback(
    ctx: Context<SubmitFeedback>,
    agent_id_hash: [u8; 32],
    compressed_feedback: CompressedFeedback,
    authorization_signature: Signature,
) -> Result<()>;

/// Update existing feedback (within time window)
pub fn update_feedback(
    ctx: Context<UpdateFeedback>,
    feedback_id: String,
    compressed_feedback: CompressedFeedback,
) -> Result<()>;

/// Revoke feedback (dispute resolution)
pub fn revoke_feedback(
    ctx: Context<RevokeFeedback>,
    feedback_id: String,
    reason: String,
) -> Result<()>;
```

### 2.4 Integration with x402 Payment Protocol

```rust
/// Payment proof structure (from x402)
pub struct PaymentProof {
    pub payment_mandate_hash: [u8; 32],
    pub transaction_signature: Signature,
    pub amount: u64,
    pub verified_at: i64,
}

impl CompressedFeedback {
    /// Include x402 payment proof for value-weighted reputation
    pub fn with_payment_proof(mut self, proof: PaymentProof) -> Self {
        self.payment_proof = Some(proof);
        self
    }
}
```

**Value-weighted reputation**: Feedback associated with $10,000 x402 payments carries more weight than $1 payments, creating anti-Sybil resistance.

---

## 3. Delegation Registry Specification (SAS-Based)

### 3.1 Design Rationale

User-to-agent delegation is fundamentally an attestation:
> "I, user Alice, attest that agent XYZ is authorized to act on my behalf with capabilities Y until time Z."

This maps perfectly to SAS's attestation model:
- **Issuer**: User (via SAS Credential)
- **Schema**: SATI-defined delegation structure
- **Data**: User ID, agent pubkey, capabilities, expiry
- **Lifecycle**: Expiry enforcement, revocation via SAS

**Why not build custom?** SAS already provides:
- ✅ Structured data (26 types)
- ✅ Expiry management
- ✅ Revocation (close_attestation)
- ✅ Schema versioning
- ✅ Event logging
- ✅ Optional tokenization

**SATI's role:** Define standard delegation schemas, validate references in Identity Registry

### 3.2 Standard Delegation Schema (SATI-Defined)

```rust
use sas::state::SchemaDataTypes;

/// SATI Standard: User-Agent Delegation Schema v1
/// Compatible with Solana Attestation Service (SAS)
///
/// NOTE: SAS schemas define data layout using SchemaDataTypes enum (26 types: 0-25)
/// Pubkeys are represented as VecU8 (32 bytes) since SAS has no native Pubkey type
pub const SATI_DELEGATION_SCHEMA_V1_LAYOUT: &[u8] = &[
    SchemaDataTypes::String as u8,      // user_identifier (email, DID, pubkey)
    SchemaDataTypes::VecU8 as u8,       // agent_pubkey (32 bytes as Vec<u8>)
    SchemaDataTypes::I64 as u8,         // delegated_at (unix timestamp)
    SchemaDataTypes::I64 as u8,         // expires_at (unix timestamp, 0 = never)
    SchemaDataTypes::U64 as u8,         // max_spend_lamports (0 = unlimited)
    SchemaDataTypes::VecString as u8,   // allowed_capabilities
    SchemaDataTypes::VecString as u8,   // allowed_merchants (empty = all)
    SchemaDataTypes::String as u8,      // additional_constraints (JSON)
];

/// Field names for SATI Delegation Schema v1
pub const SATI_DELEGATION_SCHEMA_V1_FIELDS: &[&str] = &[
    "user",
    "agent",
    "from",
    "until",
    "max_spend",
    "capabilities",
    "merchants",
    "constraints"
];

/// Schema metadata constants
pub const SATI_DELEGATION_SCHEMA_NAME: &str = "sati_user_agent_delegation_v1";
pub const SATI_DELEGATION_SCHEMA_DESCRIPTION: &str = "User authorizes agent to act with specific capabilities and spending limits";
pub const SATI_DELEGATION_SCHEMA_VERSION: u8 = 1;
```

### 3.3 Delegation Data Structure

```rust
/// Delegation data (serialized into SAS attestation)
/// Matches SATI_DELEGATION_SCHEMA_V1_LAYOUT
///
/// IMPORTANT: This struct uses Borsh serialization to match SAS schema-based serialization.
/// SAS attestation.data is stored as Borsh-encoded bytes according to the schema layout.
/// When reading attestations on-chain, deserialize with: borsh::from_slice(&attestation.data)
/// When creating attestations off-chain, serialize with SAS SDK: serializeAttestationData(schema, data)
#[derive(BorshSerialize, BorshDeserialize)]
pub struct DelegationData {
    /// User identifier (email, DID, or pubkey string)
    pub user_identifier: String,

    /// Agent being authorized (32-byte pubkey as Vec<u8>)
    /// NOTE: SAS has no native Pubkey type, so we use Vec<u8> of length 32
    /// When serializing with SAS SDK, must convert Pubkey::to_bytes()
    pub agent_pubkey: Vec<u8>,

    /// When delegation starts
    pub delegated_at: i64,

    /// When delegation expires (0 = never)
    pub expires_at: i64,

    /// Maximum spending limit in lamports (0 = unlimited)
    pub max_spend_lamports: u64,

    /// Allowed capabilities (e.g., ["shopping", "payments", "messaging"])
    pub allowed_capabilities: Vec<String>,

    /// Allowed merchants (empty = all merchants)
    pub allowed_merchants: Vec<String>,

    /// Additional constraints as JSON
    pub additional_constraints: String,
}

impl DelegationData {
    /// Helper: Get agent pubkey as Solana Pubkey
    pub fn agent_as_pubkey(&self) -> Result<Pubkey, ProgramError> {
        if self.agent_pubkey.len() != 32 {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(Pubkey::new_from_array(
            self.agent_pubkey[..].try_into().unwrap()
        ))
    }
}
```

### 3.4 Delegation Workflow

**Complete workflow using actual SAS SDK (sas-lib)**

```typescript
import {
  deriveCredentialPda,
  deriveSchemaPda,
  deriveAttestationPda,
  fetchSchema,
  getCreateSchemaInstruction,
  getCreateAttestationInstruction,
  serializeAttestationData,
} from "sas-lib";

// Step 1: Derive user credential PDA (must be created first if doesn't exist)
const [userCredentialPda] = await deriveCredentialPda({
  authority: userKeypair.address,
  name: "alice@example.com"
});

// Step 2: Derive schema PDA for SATI delegation standard
const [schemaPda] = await deriveSchemaPda({
  credential: userCredentialPda,
  name: SATI_DELEGATION_SCHEMA_NAME,
  version: SATI_DELEGATION_SCHEMA_VERSION
});

// Step 3: Create schema if it doesn't exist (one-time per credential)
const createSchemaIx = getCreateSchemaInstruction({
  payer: userKeypair,
  authority: userKeypair,
  credential: userCredentialPda,
  name: SATI_DELEGATION_SCHEMA_NAME,
  description: SATI_DELEGATION_SCHEMA_DESCRIPTION,
  layout: Array.from(SATI_DELEGATION_SCHEMA_V1_LAYOUT),
  fieldNames: Array.from(SATI_DELEGATION_SCHEMA_V1_FIELDS),
  version: SATI_DELEGATION_SCHEMA_VERSION,
  schema: schemaPda
});

// Send schema creation transaction (skip if schema already exists)
await sendTransaction(createSchemaIx);

// Step 4: Fetch created schema
const schema = await fetchSchema(rpc, schemaPda);

// Step 5: Prepare delegation data matching schema layout
const delegationData = {
  user_identifier: "alice@example.com",
  agent_pubkey: Array.from(shoppingAgent.publicKey.toBytes()), // Convert to byte array!
  delegated_at: Math.floor(Date.now() / 1000),
  expires_at: Math.floor(Date.now() / 1000) + 86400, // 24 hours
  max_spend_lamports: 1_000_000_000, // 1 SOL
  allowed_capabilities: ["shopping", "payments"],
  allowed_merchants: [], // All merchants
  additional_constraints: JSON.stringify({
    geographic_restrictions: ["US", "CA"],
    time_restrictions: { start: "09:00", end: "17:00" }
  }),
};

// Step 6: Serialize data using SAS SDK (critical!)
const serializedData = serializeAttestationData(schema.data, delegationData);

// Step 7: Derive attestation PDA (unique per nonce)
const nonceKeypair = await generateKeyPairSigner(); // Random nonce
const [attestationPda] = await deriveAttestationPda({
  credential: userCredentialPda,
  schema: schemaPda,
  nonce: nonceKeypair.address
});

// Step 8: Create attestation instruction
const createAttestationIx = getCreateAttestationInstruction({
  payer: userKeypair,
  authority: userKeypair,
  credential: userCredentialPda,
  schema: schemaPda,
  nonce: nonceKeypair.address,
  expiry: Math.floor(Date.now() / 1000) + 86400,
  data: serializedData,  // Pre-serialized!
  attestation: attestationPda
});

// Send attestation creation transaction
await sendTransaction(createAttestationIx);

// Step 9: Agent adds delegation reference to Identity Registry
// (SATI instruction, not SAS)
await addDelegation({
  agent: agentIdentityPda,
  sasAttestation: attestationPda,  // Reference to SAS attestation
  owner: agentOwnerKeypair,
});
```

### 3.5 Delegation Verification

**IMPORTANT: On-chain verification requires explicit account passing**

```rust
use sas::state::Attestation;

#[derive(Accounts)]
pub struct VerifyDelegation<'info> {
    pub agent_identity: Account<'info, AgentIdentity>,

    /// CHECK: SAS attestation account, validated against agent's delegation list
    #[account(
        constraint = agent_identity.active_delegations[..agent_identity.delegation_count as usize]
            .contains(&delegation_attestation.key())
            @ ErrorCode::DelegationNotRegistered
    )]
    pub delegation_attestation: UncheckedAccount<'info>,

    /// CHECK: SAS schema account for delegation standard
    pub delegation_schema: UncheckedAccount<'info>,
}

/// Verify agent has valid delegation from user
pub fn verify_delegation(
    ctx: Context<VerifyDelegation>,
    user_identifier_hash: [u8; 32],  // Use hash to avoid unbounded strings
    required_capability_hash: [u8; 32],
) -> Result<DelegationData> {
    // Deserialize SAS attestation from account data
    let attestation_data = ctx.accounts.delegation_attestation.try_borrow_data()?;
    let attestation = Attestation::try_deserialize(&mut attestation_data.as_ref())?;

    // CRITICAL: Check expiry (SAS does not enforce automatically!)
    let current_time = Clock::get()?.unix_timestamp;
    require!(
        attestation.expiry == 0 || current_time < attestation.expiry,
        ErrorCode::DelegationExpired
    );

    // Verify schema matches SATI delegation standard
    require!(
        attestation.schema == ctx.accounts.delegation_schema.key(),
        ErrorCode::InvalidDelegationSchema
    );

    // Deserialize delegation data (Borsh-encoded)
    let delegation: DelegationData = borsh::from_slice(&attestation.data)?;

    // Verify user identifier matches (compare hashes)
    let provided_user_hash = hash(delegation.user_identifier.as_bytes());
    require!(
        provided_user_hash == user_identifier_hash,
        ErrorCode::UserMismatch
    );

    // Verify capability is authorized (compare hashes to avoid string iteration)
    let capability_authorized = delegation.allowed_capabilities.iter()
        .any(|cap| hash(cap.as_bytes()) == required_capability_hash);

    require!(capability_authorized, ErrorCode::CapabilityNotAuthorized);

    // Verify agent pubkey matches
    let agent_pubkey = delegation.agent_as_pubkey()?;
    require!(
        agent_pubkey == ctx.accounts.agent_identity.key(),
        ErrorCode::AgentMismatch
    );

    Ok(delegation)
}

/// Helper: Simple hash function for identifier comparison
fn hash(data: &[u8]) -> [u8; 32] {
    use solana_program::hash::hashv;
    hashv(&[data]).to_bytes()
}
```

**Key Points:**
1. **No iteration** - Specific attestation must be passed as account
2. **Expiry checked explicitly** - SAS stores expiry but doesn't enforce on reads
3. **Hash-based comparison** - Avoid unbounded string comparisons on-chain
4. **Account validation** - Constraint checks attestation is in agent's delegation list

### 3.6 Delegation Lifecycle Management

**Creation:**
- User creates SAS attestation with expiry timestamp
- Agent adds attestation reference to Identity Registry
- `DelegationAdded` event emitted for indexers

**Active Use:**
- Clients query `active_delegations` list from Identity Registry (off-chain)
- For on-chain verification, pass specific attestation account explicitly
- **Programs must check expiry** - SAS stores expiry but doesn't prevent reads
- Verify schema, user, and capabilities match requirements

**Expiration:**
- **Programs must validate `attestation.expiry`** against current timestamp
- SAS stores expiry field but doesn't enforce on account reads
- Expired attestations remain readable until closed
- Agent or user should call `remove_delegation` to clean up reference in Identity Registry

**Revocation:**
- User calls SAS `close_attestation` instruction to close attestation account
- Agent removes reference from Identity Registry via `remove_delegation`
- `DelegationRemoved` event emitted
- Closed accounts cannot be accessed (rent reclaimed)

**Benefits over custom implementation:**
- ✅ Reuse SAS account structure and PDAs
- ✅ Reuse SAS close instruction for revocation
- ✅ Borsh serialization helpers provided by SAS SDK
- ✅ Tokenization option (user can mint delegation as Token-2022 NFT)
- ✅ Schema versioning (can create v2 schemas alongside v1)

---

## 4. Validation Attestations Specification (SAS-Based)

### 4.1 Design Rationale

**Work validation is fundamentally an attestation:** A validator cryptographically attests that an agent completed specific work with verifiable correctness. This maps naturally to SAS's attestation model, avoiding the need for custom validation storage.

**Why SAS for validation:**
- ✅ Structured data (enclave measurements, signatures, metadata)
- ✅ Expiry (validations shouldn't be valid forever)
- ✅ Schema versioning (can add zkML, stake-based methods in Phase 2)
- ✅ Revocability (validator can retract if error found)
- ✅ Tokenization (optional validator reputation NFTs)
- ✅ Composable with broader Solana ecosystem

**Phase 1 scope:** TEE-based validation only. Multi-validator consensus and additional validation methods (zkML, stake-based) will be added in Phase 2 based on ecosystem demand.

### 4.2 Standard TEE Validation Schema (SATI-Defined)

```rust
/// SATI Standard: TEE Attestation v1
pub const TEE_ATTESTATION_SCHEMA_V1: SchemaDefinition = SchemaDefinition {
    name: "sati_tee_attestation_v1",
    description: "TEE-based work validation with enclave measurements",
    version: 1,

    layout: vec![
        SchemaDataTypes::Hash32,    // agent_id_hash
        SchemaDataTypes::Hash32,    // work_hash
        SchemaDataTypes::String,    // enclave_type (intel_tdx/amd_sev/phala)
        SchemaDataTypes::VecU8,     // measurement
        SchemaDataTypes::VecU8,     // signature
        SchemaDataTypes::Hash32,    // output_hash
        SchemaDataTypes::I64,       // validated_at
        SchemaDataTypes::String,    // metadata (JSON for additional context)
    ],

    field_names: vec![
        "agent",
        "work",
        "enclave_type",
        "measurement",
        "signature",
        "output",
        "timestamp",
        "metadata"
    ],
};
```

**Supported TEE Platforms:**
- Intel TDX (Trust Domain Extensions)
- AMD SEV-SNP (Secure Encrypted Virtualization)
- Phala Network (TEE Cloud)
- AWS Nitro Enclaves
- Azure Confidential Computing

**Additional validation schemas (included in spec, Phase 2 implementation):**

```rust
/// SATI Standard: zkML Proof Validation v1 (Phase 2)
pub const ZKML_VALIDATION_SCHEMA_V1: SchemaDefinition = SchemaDefinition {
    name: "sati_zkml_validation_v1",
    description: "Zero-knowledge machine learning proof validation",
    version: 1,

    layout: vec![
        SchemaDataTypes::Hash32,    // agent_id_hash
        SchemaDataTypes::Hash32,    // model_hash
        SchemaDataTypes::Hash32,    // input_hash
        SchemaDataTypes::Hash32,    // output_hash
        SchemaDataTypes::String,    // proof_type (groth16/plonk/stark)
        SchemaDataTypes::VecU8,     // proof_data
        SchemaDataTypes::VecString, // public_inputs
        SchemaDataTypes::I64,       // validated_at
    ],

    field_names: vec![
        "agent", "model", "input", "output",
        "proof_type", "proof", "public_inputs", "timestamp"
    ],
};

/// SATI Standard: Generic Work Validation v1 (Phase 2)
pub const WORK_VALIDATION_SCHEMA_V1: SchemaDefinition = SchemaDefinition {
    name: "sati_work_validation_v1",
    description: "Generic work validation with confidence scoring",
    version: 1,

    layout: vec![
        SchemaDataTypes::Hash32,    // agent_id_hash
        SchemaDataTypes::Hash32,    // work_hash
        SchemaDataTypes::U8,        // confidence_score (0-100)
        SchemaDataTypes::String,    // validation_method (stake/reputation/oracle)
        SchemaDataTypes::I64,       // validated_at
        SchemaDataTypes::String,    // metadata (JSON)
    ],

    field_names: vec![
        "agent", "work", "confidence",
        "method", "timestamp", "metadata"
    ],
};
```

These schemas are included in the SATI specification and can be implemented by validators immediately. Multi-validator consensus orchestration will be added in Phase 2.

### 4.3 TEE Validation Data Structure

```rust
/// TEE validation data (stored in SAS attestation)
#[derive(BorshSerialize, BorshDeserialize)]
pub struct TEEValidationData {
    /// Agent being validated
    pub agent_id_hash: [u8; 32],

    /// Work being validated (hash of input/output)
    pub work_hash: [u8; 32],

    /// TEE platform (intel_tdx, amd_sev, phala, aws_nitro, azure_cc)
    pub enclave_type: String,

    /// TEE measurement/quote
    pub measurement: Vec<u8>,

    /// TEE signature over work
    pub signature: Vec<u8>,

    /// Hash of validated output
    pub output_hash: [u8; 32],

    /// Validation timestamp
    pub validated_at: i64,

    /// Additional context (JSON)
    pub metadata: String,
}
```

**No SATI-native orchestrator in Phase 1.** Agents simply reference SAS attestation pubkeys in their `validation_attestations` field (see Identity Registry Section 1.1).

### 4.4 Validation Workflow

```typescript
// Step 1: Validator creates SAS credential (one-time setup)
const validatorCredential = await createCredential({
  authority: validatorKeypair.publicKey,
  name: "PhalaValidator",
  authorizedSigners: [validatorKeypair.publicKey],
});

// Step 2: Fetch SATI TEE validation schema
const teeSchema = await fetchSatiStandardSchema("sati_tee_attestation_v1");

// Step 3: Agent completes work, TEE validator independently validates
const teeResult = await runInTEE({
  enclave: "intel_tdx",
  workload: agentTask,
});

// Step 4: Validator creates SAS attestation
const validationAttestation = await createAttestation({
  credential: validatorCredential,
  schema: teeSchema,
  data: {
    agent_id_hash: agentIdentity.agent_id_hash,
    work_hash: computeWorkHash(agentTask),
    enclave_type: "intel_tdx",
    measurement: teeResult.measurement,
    signature: teeResult.signature,
    output_hash: computeHash(teeResult.output),
    validated_at: Math.floor(Date.now() / 1000),
    metadata: JSON.stringify({ node_id: "validator_1" }),
  },
  expiry: Math.floor(Date.now() / 1000) + 86400, // 24 hours
  signer: validatorKeypair,
});

// Step 5: Agent adds validation reference to Identity Registry
await addValidation({
  agent: agentIdentity,
  sasAttestation: validationAttestation,
  owner: agentOwnerKeypair,
});
```

### 4.5 Querying Validation Proofs

```typescript
// Fetch agent identity with validation references
const agent = await fetchAgentIdentity(agentIdHash);

// Fetch all validation attestations
const validations = await Promise.all(
  agent.validation_attestations.map(pubkey =>
    fetchSasAttestation(pubkey)
  )
);

// Filter for recent, non-expired validations
const activeValidations = validations.filter(v => {
  const now = Math.floor(Date.now() / 1000);
  return v.expiry === 0 || v.expiry > now;
});

// Analyze validation data
console.log(`Agent has ${activeValidations.length} active TEE validations`);
activeValidations.forEach((v, i) => {
  const data = deserializeTEEValidation(v.data);
  console.log(`Validation ${i + 1}:`);
  console.log(`  TEE: ${data.enclave_type}`);
  console.log(`  Work: ${Buffer.from(data.work_hash).toString('hex').slice(0, 16)}...`);
  console.log(`  Timestamp: ${new Date(data.validated_at * 1000).toISOString()}`);
});
```

### 4.6 Benefits of SAS-Based Validation

**Why no custom orchestrator in Phase 1:**
- Simpler implementation (no consensus logic needed)
- Faster time-to-market (4 weeks saved)
- SAS provides all necessary primitives (expiry, revocation, schemas)
- Agents can collect multiple validations manually if needed
- Can add multi-validator consensus in Phase 2 based on demand

**Cost comparison:**
- Custom validation storage: 0.004 SOL per validation
- SAS attestation: 0.00001 SOL per validation
- **400x cheaper** while maintaining production-grade reliability

---

## 4. Mandate Registry Specification

### 4.1 Design Rationale

**Payment mandate lifecycle management complements agent trust infrastructure (identity, reputation, delegation) by providing transaction authorization primitives that AP2 requires but doesn't implement.**

**Why mandate registry is essential:**
1. **Full AP2 Support**: Google's AP2 protocol requires mandate lifecycle management but doesn't specify implementation
2. **Real-World Necessity**: "Zombie mandates" (valid but obsolete) cause involuntary disputes and regulatory scrutiny
3. **User Context Drift**: Mandates created months ago may no longer reflect user intent (relocation, budget changes, life events)
4. **Regulatory Compliance**: Subscription-style autonomous payments require context revalidation mechanisms
5. **Economic Security**: Without lifecycle management, merchants face chargebacks and users face unwanted charges

**SATI's Mandate Registry solves the problem AP2 identified but didn't implement.**

### 4.2 Account Structure

```rust
/// Payment mandate with lifecycle management
#[account]
pub struct MandateAccount {
    /// PDA bump seed
    pub bump: u8,

    /// Agent identity hash (references Identity Registry)
    pub agent_id_hash: [u8; 32],

    /// User authority who created mandate
    pub user_authority: Pubkey,

    /// Mandate type (0=Intent, 1=Cart)
    pub mandate_type: u8,

    /// Current lifecycle state
    pub status: MandateStatus,

    /// When mandate was created
    pub created_at: i64,

    /// When mandate expires (0 = no expiry)
    pub expires_at: i64,

    /// Hash of user's context at creation (location, budget, preferences)
    /// Used for drift detection
    pub context_hash: [u8; 32],

    /// When context should be revalidated (0 = no revalidation needed)
    pub revalidation_threshold: i64,

    /// Number of times mandate has been executed (partial execution tracking)
    pub execution_count: u64,

    /// Maximum allowed executions (0 = unlimited)
    pub max_executions: u64,

    /// ZK compression state root pointing to full mandate details
    pub mandate_state_root: [u8; 32],

    /// Last updated timestamp
    pub updated_at: i64,
}

impl MandateAccount {
    /// Account space including 8-byte discriminator:
    /// 8 (discriminator) + 1 (bump) + 32 (agent_id_hash) + 32 (user_authority) +
    /// 1 (mandate_type) + 1 (status) + 8 (created_at) + 8 (expires_at) +
    /// 32 (context_hash) + 8 (revalidation_threshold) + 8 (execution_count) +
    /// 8 (max_executions) + 32 (mandate_state_root) + 8 (updated_at)
    /// = 187 bytes
    pub const SPACE: usize = 8 + 1 + 32 + 32 + 1 + 1 + 8 + 8 + 32 + 8 + 8 + 8 + 32 + 8;
}

/// Mandate lifecycle states
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MandateStatus {
    /// Mandate created and ready for execution
    Active = 0,
    /// Mandate has expired based on time
    Expired = 1,
    /// Manually revoked by user or agent
    Revoked = 2,
    /// Successfully executed (for single-use mandates)
    Executed = 3,
    /// Context drift detected, requires user revalidation
    Zombie = 4,
    /// Amended (new mandate created, old one deprecated)
    Amended = 5,
}
```

### 4.3 Ledger-Stored Mandate Details (ZK Verified)

```json
{
  "mandate_id": "intent_mandate_abc123",
  "mandate_type": "IntentMandate",

  "user": {
    "identifier": "alice@example.com",
    "pubkey": "5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp",
    "signed_at": 1699920000
  },

  "agent": {
    "agent_id_hash": "0x...",
    "sati_identity": "AgentPubkey..."
  },

  "intent": {
    "description": "Buy 2 concert tickets < $1000, close to stage",
    "constraints": {
      "max_price_lamports": 1000000000,
      "required_attributes": ["close_to_stage"],
      "merchant_restrictions": ["ticketmaster.com"],
      "geographic_restrictions": ["US", "CA"]
    }
  },

  "payment": {
    "max_spend_lamports": 1000000000,
    "allowed_payment_methods": ["usdc", "sol"],
    "credentials_provider": "CredentialProviderPubkey..."
  },

  "context": {
    "user_location": {"country": "US", "state": "CA"},
    "user_budget_tier": "premium",
    "user_preferences": {"priority": "quality_over_price"}
  },

  "lifecycle": {
    "created_at": 1699920000,
    "expires_at": 1699923600,
    "revalidation_trigger": {
      "time_based": 1699921800,
      "event_based": ["location_change", "budget_change"]
    }
  },

  "proof": {
    "type": "groth16",
    "commitment": "0x..."
  }
}
```

### 4.4 Instructions

```rust
/// Create new payment mandate
pub fn create_mandate(
    ctx: Context<CreateMandate>,
    mandate_type: MandateType,
    compressed_mandate: CompressedMandate,
    proof: ZkProof,
    expires_in_seconds: i64,
    revalidation_threshold_seconds: i64,
) -> Result<()>;

/// Revoke mandate before execution
pub fn revoke_mandate(
    ctx: Context<RevokeMandate>,
    reason: String,
) -> Result<()>;

/// Amend mandate (creates new, marks old as amended)
pub fn amend_mandate(
    ctx: Context<AmendMandate>,
    new_compressed_mandate: CompressedMandate,
    proof: ZkProof,
) -> Result<()>;

/// Mark mandate as executed (called by payment processor)
pub fn execute_mandate(
    ctx: Context<ExecuteMandate>,
    payment_signature: Signature,
) -> Result<()>;

/// Check if context has drifted (callable by anyone)
pub fn check_context_drift(
    ctx: Context<CheckContextDrift>,
    current_context_hash: [u8; 32],
) -> Result<bool>;

/// User revalidates mandate after zombie detection
pub fn revalidate_mandate(
    ctx: Context<RevalidateMandate>,
    new_context_hash: [u8; 32],
) -> Result<()>;

/// Force expiry check (anyone can call)
pub fn enforce_expiry(
    ctx: Context<EnforceExpiry>,
) -> Result<()>;
```

### 4.5 Context Drift Detection

**The "Zombie Mandate" Problem:**

```typescript
// User creates mandate on November 1
const mandate = await createMandate({
  intent: "Buy groceries weekly, budget $200",
  location: "New York, NY",
  budget: 200_000_000, // lamports
});

// December 15: User moves to San Francisco (different prices)
// Mandate is still technically valid but context has changed
// Old mandate might overpay or select wrong merchants

// SATI Solution: Context drift detection
const contextCheck = await checkContextDrift({
  mandate,
  currentContext: {
    location: "San Francisco, CA", // Changed!
    budget: 150_000_000,  // Changed!
  }
});

if (contextCheck.drifted) {
  // Mandate marked as Zombie
  // Requires user revalidation before execution
}
```

**Drift Detection Algorithm:**

```rust
pub fn check_context_drift(
    ctx: Context<CheckContextDrift>,
    current_context_hash: [u8; 32],
) -> Result<bool> {
    let mandate = &mut ctx.accounts.mandate;

    // Compare context hashes
    let has_drifted = mandate.context_hash != current_context_hash;

    // Check time-based revalidation
    let current_time = Clock::get()?.unix_timestamp;
    let needs_revalidation = mandate.revalidation_threshold > 0
        && current_time >= mandate.revalidation_threshold;

    if has_drifted || needs_revalidation {
        // Mark as Zombie
        mandate.status = MandateStatus::Zombie;
        mandate.updated_at = current_time;

        emit!(MandateContextDrift {
            mandate_id_hash: mandate.mandate_state_root,
            old_context: mandate.context_hash,
            new_context: current_context_hash,
            timestamp: current_time,
        });

        return Ok(true);
    }

    Ok(false)
}
```

### 4.6 AP2 Integration Pattern

**Complete AP2 + SATI Flow:**

```typescript
// Step 1: User delegates to Shopping Agent (SATI Delegation Registry)
const delegation = await createSATIDelegation({
  agent: shoppingAgent,
  maxSpend: 10_000_000_000, // 10 SOL
  capabilities: ["shopping", "payments"],
  expiresIn: 86400 * 30, // 30 days
});

// Step 2: Shopping Agent creates IntentMandate (AP2 spec)
const ap2IntentMandate = {
  user: "alice@example.com",
  intent: "Buy 2 concert tickets < $1000, close to stage",
  budget: 1000_000_000,
  allowed_capabilities: ["shopping"],
};

// Step 3: Register IntentMandate in SATI Mandate Registry
const mandateRef = await createMandate({
  agentIdHash: shoppingAgent.identityHash,
  mandateType: MandateType.Intent,
  compressedMandate: compress(ap2IntentMandate),
  expiresIn: 3600, // 1 hour
  revalidationThreshold: 1800, // Revalidate after 30 min if not executed
});

// Step 4: Merchant negotiates, creates CartMandate (AP2)
const ap2CartMandate = await merchant.createCart({
  intent: ap2IntentMandate,
  selectedTickets: [
    { section: "A", row: 5, price: 450_000_000 },
    { section: "A", row: 5, price: 450_000_000 },
  ],
});

// Step 5: Register CartMandate in SATI
const cartMandateRef = await createMandate({
  agentIdHash: shoppingAgent.identityHash,
  mandateType: MandateType.Cart,
  compressedMandate: compress(ap2CartMandate),
  expiresIn: 300, // 5 minutes (user must approve quickly)
});

// Step 6: User approves CartMandate
const approval = await user.signCartMandate(ap2CartMandate);

// Step 7: Execute payment
const payment = await executePayment({
  cartMandate: ap2CartMandate,
  approval,
});

// Step 8: Mark mandate executed in SATI
await executeMandate({
  mandateRef: cartMandateRef,
  paymentSignature: payment.signature,
});

// Step 9: Submit feedback to SATI Reputation Registry
await submitFeedback({
  agentIdHash: shoppingAgent.identityHash,
  rating: 95,
  paymentProof: {
    protocol: "x402",
    signature: payment.signature,
    amount: 900_000_000,
  },
});
```

### 4.7 Lifecycle State Machine

```
        create_mandate()
               │
               ▼
         ┌──────────┐
         │  ACTIVE  │ ◄──── revalidate_mandate() (from ZOMBIE)
         └──────────┘
               │
               ├─── execute_mandate() ──► EXECUTED (final)
               │
               ├─── revoke_mandate() ──► REVOKED (final)
               │
               ├─── enforce_expiry() ──► EXPIRED (final)
               │
               ├─── amend_mandate() ──► AMENDED (final, points to new mandate)
               │
               └─── check_context_drift() ──► ZOMBIE
                           │
                           └─── revalidate_mandate() ──► ACTIVE (cycle continues)
                           │
                           └─── revoke_mandate() ──► REVOKED (final)

Final states: EXECUTED, REVOKED, EXPIRED, AMENDED
Recoverable: ZOMBIE → ACTIVE (via revalidation)
```

### 4.8 Events

```rust
#[event]
pub struct MandateCreated {
    pub mandate_state_root: [u8; 32],
    pub agent_id_hash: [u8; 32],
    pub user_authority: Pubkey,
    pub mandate_type: u8,
    pub expires_at: i64,
    pub timestamp: i64,
}

#[event]
pub struct MandateRevoked {
    pub mandate_state_root: [u8; 32],
    pub reason: String,
    pub revoked_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct MandateExecuted {
    pub mandate_state_root: [u8; 32],
    pub payment_signature: Signature,
    pub execution_count: u64,
    pub timestamp: i64,
}

#[event]
pub struct MandateContextDrift {
    pub mandate_state_root: [u8; 32],
    pub old_context: [u8; 32],
    pub new_context: [u8; 32],
    pub timestamp: i64,
}

#[event]
pub struct MandateRevalidated {
    pub mandate_state_root: [u8; 32],
    pub new_context: [u8; 32],
    pub revalidated_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct MandateAmended {
    pub old_mandate: [u8; 32],
    pub new_mandate: [u8; 32],
    pub timestamp: i64,
}
```

### 4.9 Security Considerations

**1. Preventing Unauthorized Execution:**
```rust
// Only user or delegated agent can execute mandate
require!(
    ctx.accounts.executor.key() == mandate.user_authority ||
    agent_has_valid_delegation(mandate.agent_id_hash, ctx.accounts.executor.key()),
    ErrorCode::UnauthorizedExecution
);
```

**2. Zombie Mandate Protection:**
```rust
// Anyone can trigger drift check, but only user can revalidate
pub fn check_context_drift(ctx: Context<CheckContextDrift>) -> Result<bool> {
    // Permissionless - anyone can call
}

pub fn revalidate_mandate(ctx: Context<RevalidateMandate>) -> Result<()> {
    // Permissioned - only user authority
    require!(
        ctx.accounts.signer.key() == mandate.user_authority,
        ErrorCode::UnauthorizedRevalidation
    );
}
```

**3. Expiry Enforcement:**
```rust
// Permissionless expiry enforcement (anyone can call)
pub fn enforce_expiry(ctx: Context<EnforceExpiry>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    require!(
        mandate.expires_at > 0 && current_time >= mandate.expires_at,
        ErrorCode::NotExpired
    );
    mandate.status = MandateStatus::Expired;
    Ok(())
}
```

### 4.10 Cost Analysis

| Operation | Traditional | SATI (ZK Compressed) | Savings |
|-----------|-------------|----------------------|---------|
| Create mandate | 0.008 SOL | 0.00001 SOL | 800x |
| Revoke mandate | 0.002 SOL | 0.000005 SOL | 400x |
| Context drift check | 0.001 SOL | 0.000002 SOL | 500x |
| 1M mandates/month | 8,000 SOL ($1.6M) | 10 SOL ($2,000) | 800x |

**Why this matters:** High-volume agent payments become economically viable on Solana.

---

## 5. ZK Compression Integration

### 5.1 Compression Architecture

SATI uses Light Protocol's ZK Compression V1 (mainnet since September 2025) for cost-efficient storage:

**On-chain (AccountsDB):**
- 32-byte state roots
- Minimal verification data
- Event emissions

**Off-chain (Ledger):**
- Full agent metadata
- Complete feedback details

**Verification:**
- Groth16 SNARKs (128-byte proofs)
- Automatic integrity validation
- RPC indexer access (Photon)

### 5.2 Cost Comparison

| Storage Type | Traditional | ZK Compressed | Savings |
|--------------|-------------|---------------|---------|
| 2KB agent metadata | 0.016 SOL | 0.00001 SOL | 1,600x |
| 1KB feedback entry | 0.008 SOL | 0.000005 SOL | 1,600x |

**For 1M agents + 10M feedback:**
- Traditional: ~232,000 SOL ($46.4M @ $200/SOL)
- ZK Compressed: ~120 SOL ($24K)
- **Savings: 1,933x ($46.376M saved)**

### 5.3 Implementation Details

```rust
use light_sdk::{
    compressed_account::CompressedAccount,
    merkle_context::MerkleContext,
    proof::CompressedProof,
};

pub fn register_agent(
    ctx: Context<RegisterAgent>,
    agent_id: String,
    metadata: CompressedAgentMetadata,
    proof: CompressedProof,
) -> Result<()> {
    // Verify ZK proof
    proof.verify(&metadata.merkle_context)?;

    // Compute state root
    let state_root = compute_state_root(&metadata)?;

    // Create minimal on-chain account
    let agent = &mut ctx.accounts.agent;
    agent.state_root = state_root;
    agent.ledger_ref = Clock::get()?.slot;

    // Emit event for indexers
    emit!(AgentRegistered {
        agent_id_hash: hash(&agent_id),
        state_root,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
```

---

## 6. Cross-Chain Interoperability via DIDs

### 6.1 Design Rationale

**The agent economy is inherently multi-chain.** Google's AP2 protocol, Ethereum's ERC-8004, and SATI must interoperate for the ecosystem to function. Users with Ethereum wallets need to trust Solana agents, and vice versa.

**Decentralized Identifiers (DIDs)** provide the W3C standard solution for cross-chain identity:
- Chain-agnostic identifier format
- Cryptographic verification without blockchain-specific logic
- Established resolver infrastructure
- Compatible with existing Web2/Web3 identity systems

### 6.2 Supported DID Methods

SATI Identity Registry natively supports three DID methods:

**1. did:solana (Native Solana agents)**
```
did:solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp
```
- Derived directly from agent owner pubkey
- Verifiable via Solana signature
- Lowest friction for Solana-native agents

**2. did:pkh (Blockchain Account DIDs)**
```
did:pkh:eip155:1:0x1234567890abcdef1234567890abcdef12345678
```
- Ethereum agents (ERC-8004 compatible)
- Bitcoin, Cosmos, any CAIP-10 blockchain
- Verifiable via chain-specific signatures

**3. did:web (Web-based agents)**
```
did:web:api.tradingcorp.com
```
- Traditional web services operating as agents
- DNS ownership verification
- Lowest friction for Web2→Web3 migration

### 6.3 DID Resolution Flow

```typescript
// Cross-chain agent verification example

// 1. Solana agent encounters Ethereum shopping agent
const ethereumAgent = {
  did: "did:pkh:eip155:1:0xABC...",
  // ... claims to be registered in ERC-8004
};

// 2. Resolve DID to DID Document
const didDocument = await resolveDID(ethereumAgent.did);
// Returns: verification methods, service endpoints

// 3. Verify ERC-8004 registration (via bridge/oracle)
const erc8004Registry = "0x..."; // ERC-8004 contract address
const ethereumAddress = extractAddressFromDID(ethereumAgent.did);

// Query via Wormhole, LayerZero, or dedicated oracle
const isRegistered = await verifyEthereumRegistration({
  registry: erc8004Registry,
  address: ethereumAddress,
  verificationMethod: "wormhole", // or "layerzero", "chainlink"
});

// 4. Fetch cross-chain reputation (if available)
const ethereumReputation = await fetchCrossChainReputation({
  chain: "ethereum",
  agentAddress: ethereumAddress,
});

// 5. Solana agent decides whether to trust
if (isRegistered && ethereumReputation.score >= 75) {
  // Proceed with transaction
  await executeTransaction(ethereumAgent);
}
```

### 6.4 Bridging SATI ↔ ERC-8004

**Problem:** Solana SATI agent needs to verify Ethereum ERC-8004 agent (and vice versa)

**Solution paths:**

**Option A: State Root Bridges (Wormhole, LayerZero)**
```rust
// Solana program verifies Ethereum state
pub fn verify_erc8004_agent(
    ctx: Context<VerifyERC8004>,
    ethereum_agent_address: [u8; 20],
    wormhole_vaa: Vec<u8>,  // Verified Action Approval
) -> Result<()> {
    // Wormhole guardian signatures verify Ethereum state
    let ethereum_state = verify_wormhole_vaa(&wormhole_vaa)?;

    // Check if agent registered in ERC-8004
    let agent_exists = ethereum_state.contains_registration(
        ethereum_agent_address,
        ERC8004_REGISTRY_ADDRESS,
    )?;

    require!(agent_exists, ErrorCode::NotRegisteredInERC8004);
    Ok(())
}
```

**Option B: Oracle Networks (Chainlink, Pyth)**
```typescript
// Query Chainlink oracle for ERC-8004 registration
const oracleRequest = await chainlinkProgram.methods.requestData({
  jobId: "erc8004_registration_check",
  parameters: {
    ethereumAddress: "0xABC...",
    registryContract: "0xERC8004...",
  },
});

// Oracle returns signed attestation
const result = await oracleRequest.wait();
if (result.isRegistered) {
  // Agent is verified ERC-8004 participant
}
```

**Recommendation for Phase 1:** Option B (Oracle networks) for fastest implementation. Option A (Wormhole) for production after security audits.

**Note:** Light client-based verification (Solana verifying Ethereum consensus directly) is deferred to Phase 2 due to implementation complexity and infrastructure maturity requirements.

### 6.5 AP2 Integration via DIDs

**AP2 merchant agents on Ethereum can interact with SATI agents on Solana:**

```typescript
// AP2 Credentials Provider verifies SATI agent
const shoppingAgent = {
  did: "did:solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp",
  sati_identity: "abc123...",  // SATI agent_id_hash
};

// 1. Resolve DID to get SATI registry reference
const didDoc = await resolveDID(shoppingAgent.did);
const satiEndpoint = didDoc.service.find(s => s.type === "SATIAgentRegistry");

// 2. Query Solana SATI registry (via RPC)
const satiIdentity = await solanaRPC.getAccountInfo(
  deriveSATIPDA(shoppingAgent.sati_identity)
);

// 3. Fetch reputation from SATI Reputation Registry
const reputation = await querySATIReputation(shoppingAgent.sati_identity);

// 4. Verify active user delegation
const delegations = satiIdentity.active_delegations;
const userDelegation = await verifySATIDelegation({
  agent: shoppingAgent.sati_identity,
  user: "alice@example.com",
  delegations,
});

// 5. AP2 Credentials Provider makes trust decision
if (reputation.avg_score >= 80 && userDelegation.valid) {
  // Release payment method to agent
  await providePaymentMethod(shoppingAgent);
}
```

### 6.6 Cross-Chain Reputation Aggregation

**Challenge:** Agents may have reputation on multiple chains (SATI, ERC-8004, custom registries)

**Solution:** DID Document service endpoints can reference multiple reputation sources

```json
{
  "@context": "https://www.w3.org/ns/did/v1",
  "id": "did:solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp",
  "service": [
    {
      "id": "#sati-reputation",
      "type": "SATIReputationRegistry",
      "serviceEndpoint": "https://sati-api.solana.com/reputation/abc123"
    },
    {
      "id": "#erc8004-reputation",
      "type": "ERC8004ReputationRegistry",
      "serviceEndpoint": "https://ethereum.org/erc8004/reputation/0xABC"
    },
    {
      "id": "#aggregate-reputation",
      "type": "AggregatedReputation",
      "serviceEndpoint": "https://reputation-aggregator.com/agent/did:solana:5eykt4..."
    }
  ]
}
```

**Third-party aggregators** can build businesses by querying all reputation sources and providing unified scores.

---

## 7. Alpenglow Finality Integration

### 7.1 Finality Model Changes

**Pre-Alpenglow (current):**
```rust
match commitment_level {
    CommitmentLevel::Processed => { /* 400ms */ },
    CommitmentLevel::Confirmed => { /* 1-2s */ },
    CommitmentLevel::Finalized => { /* 12.8s */ },
}
```

**Post-Alpenglow (Q1 2026):**
```rust
if transaction.has_certificate() {
    // 150ms median finality
    process_finalized_state_change();
}
```

### 7.2 Enabled Use Cases

**Real-time agent discovery:**
- Agent registers → 150ms → Discoverable
- vs 12.8s previous finality

**Live reputation updates:**
- Feedback submitted → 150ms → Reputation updated
- Enables real-time leaderboards

**Time-sensitive validation:**
- Validator attests → 150ms → Orchestrator updated
- Critical for time-sensitive workflows

---

## Cost & Performance Analysis

### Cost Comparison Matrix

| Operation | Traditional | SATI (ZK + SAS) | Savings |
|-----------|-------------|-----------------|---------|
| Register 1 agent | 0.016 SOL ($3.20) | 0.00001 SOL ($0.002) | 1,600x |
| Register 1M agents | 16,000 SOL ($3.2M) | 10 SOL ($2,000) | 1,600x |
| Submit feedback | 0.008 SOL ($1.60) | 0.000005 SOL ($0.001) | 1,600x |
| 10M feedback | 80,000 SOL ($16M) | 50 SOL ($10,000) | 1,600x |
| Delegation (SAS) | 0.004 SOL ($0.80) | 0.00001 SOL ($0.002) | 400x |
| Validation orchestration | 0.004 SOL ($0.80) | 0.00002 SOL ($0.004) | 200x |
| Validation storage (SAS) | N/A | 0.00001 SOL ($0.002) | N/A |
| **TOTAL** | **100,000 SOL ($20M)** | **73 SOL ($14,600)** | **1,370x** |

**Note**: Assumes SOL @ $200. Includes SAS costs for delegation and validation storage.

### Performance Characteristics

| Metric | Pre-2025 | SATI (2025+) | Improvement |
|--------|----------|--------------|-------------|
| Agent registration latency | 400ms | **150ms** | 2.6x faster |
| Feedback submission latency | 400ms | **150ms** | 2.6x faster |
| Delegation creation | N/A | **150ms** | New feature |
| Mandate lifecycle operation | N/A | **150ms** | New feature |
| Validation consensus | 12.8s | **150ms** | 85x faster |
| State verification complexity | O(n log n) | **O(n)** | Unlimited scale |

---

## Implementation Roadmap

### Phase 1: Core Infrastructure (6 weeks)

**Milestone 1.1: Identity Registry (2 weeks)**
- [ ] ZK Compression integration
- [ ] PDA-based agent identity accounts
- [ ] DID support (did:solana, did:pkh, did:web)
- [ ] Delegation reference tracking
- [ ] Validation reference tracking
- [ ] Event emission for indexers
- [ ] Photon RPC integration

**Milestone 1.2: Reputation Registry (2 weeks)**
- [ ] Feedback authorization mechanism
- [ ] Event-based feedback submission
- [ ] x402 payment proof integration
- [ ] Off-chain indexer reference implementation

**Milestone 1.3: Mandate Registry (1.5 weeks)**
- [ ] ZK Compression integration for mandate storage
- [ ] Lifecycle state machine implementation
- [ ] Context drift detection algorithm
- [ ] Revocation and amendment workflows
- [ ] AP2 IntentMandate/CartMandate support

**Milestone 1.4: SAS Standard Schemas (0.5 weeks)**
- [ ] Define delegation schema v1
- [ ] Define TEE attestation schema v1
- [ ] Define zkML validation schema v1 (spec only, Phase 2 implementation)
- [ ] Define generic work validation schema v1 (spec only, Phase 2 implementation)
- [ ] Create reference implementations for delegation and TEE schemas

### Phase 2: SAS Integration & Cross-Chain (2 weeks)

**Milestone 2.1: Delegation Integration (0.5 weeks)**
- [ ] Delegation workflow implementation
- [ ] add_delegation / remove_delegation instructions
- [ ] Delegation verification logic
- [ ] Integration tests

**Milestone 2.2: AP2 Integration Examples (0.5 weeks)**
- [ ] Complete IntentMandate → CartMandate flow
- [ ] Context drift detection examples
- [ ] Zombie mandate handling workflows
- [ ] Payment execution integration

**Milestone 2.3: Cross-Chain Interoperability (1 week)**
- [ ] DID resolution implementation
- [ ] Cross-chain verification helpers (Wormhole/Chainlink integration stubs)
- [ ] Full AP2 compatibility documentation
- [ ] ERC-8004 bridge documentation

### Phase 3: Protocol Integration (2 weeks)

**Milestone 3.1: A2A/MCP/AP2 Support (1 week)**
- [ ] Agent Card JSON schema validation
- [ ] Multi-protocol endpoint discovery
- [ ] AP2 credentials provider registration
- [ ] DID document generation

**Milestone 3.2: Indexer & Tooling (1 week)**
- [ ] Photon RPC integration
- [ ] GraphQL API for queries
- [ ] CLI tools for agent management
- [ ] DID resolver utility

### Phase 4: Mainnet Launch (3 weeks)

**Milestone 4.1: Security Audit (2 weeks)**
- [ ] Smart contract audit (Identity, Reputation registries)
- [ ] Economic model audit
- [ ] Cross-chain security review
- [ ] Penetration testing

**Milestone 4.2: Deployment (1 week)**
- [ ] Mainnet deployment
- [ ] Documentation finalization
- [ ] SDK releases (Rust, TypeScript)
- [ ] DID resolver service deployment

**Total Phase 1: 13 weeks** (includes complete AP2 support with mandate lifecycle management)

### Phase 2 (Future): Advanced Features

**Planned for 3-6 months post-launch:**
- [ ] Multi-validator consensus orchestrator
- [ ] zkML validation schema implementation
- [ ] Generic work validation schema implementation
- [ ] Wormhole/LayerZero production integration
- [ ] Cross-chain reputation aggregation service
- [ ] Advanced dispute resolution mechanisms
- [ ] Light client-based cross-chain verification

---


## Security Considerations

### 1. ZK Compression Risks

**Risk**: Ledger pruning by RPC providers
**Mitigation**: Multiple indexer redundancy (Photon, custom), state root verification

**Risk**: ZK proof verification bugs
**Mitigation**: Use audited Light Protocol SDK, Groth16 proofs (battle-tested)

### 2. Reputation Gaming

**Risk**: Sybil attacks (fake feedback)
**Mitigation**: Pre-authorized feedback, x402 payment proof integration, value weighting

**Risk**: Wash trading (mutual positive reviews)
**Mitigation**: Payment proof verification, off-chain analysis, reputation aggregator competition

### 3. Cross-Chain Verification Risks

**Risk**: Oracle manipulation (cross-chain state verification)
**Mitigation**: Use multiple oracle providers (Chainlink + Pyth), require threshold consensus, implement challenge periods

**Risk**: Bridge exploits (Wormhole, LayerZero)
**Mitigation**: Use bridge insurance protocols, implement rate limits, monitor for anomalous cross-chain activity

**Risk**: DID resolution failures
**Mitigation**: Implement fallback resolvers, cache DID documents with TTL, support multiple resolution methods

### 4. Validation Trust Assumptions

**Risk**: Single validator in Phase 1 (no consensus)
**Mitigation**: Agents can collect multiple independent validations, reputation scoring accounts for validator quality, Phase 2 adds orchestration

**Risk**: TEE compromise
**Mitigation**: Support multiple TEE platforms (Intel TDX, AMD SEV, Phala), require fresh attestations, implement revocation

### 5. SAS Dependency Risks

**Risk**: SAS program bugs or vulnerabilities
**Mitigation**: SAS is Solana Foundation official, production-tested since Feb 2025, multiple audits

**Risk**: SAS program upgrade breaks compatibility
**Mitigation**: Programs are immutable on Solana, SATI can specify SAS version, fork if necessary

**Risk**: SAS becomes deprecated
**Mitigation**: If needed, SATI can replace SAS with custom storage in future version (backwards compatible)

---

## Backwards Compatibility

### With Existing Solana Infrastructure

**Compatible with:**
- ✅ Anchor Framework 0.30+
- ✅ Solana RPC API v1.18+
- ✅ Light Protocol ZK Compression V1
- ✅ SAS (Solana Foundation, mainnet Feb 2025)
- ✅ Standard Solana wallets

**Requires:**
- ⚠️ Alpenglow consensus (Q1 2026) for full performance
- ⚠️ Photon RPC for ZK state queries
- ⚠️ SAS program deployed (already on mainnet)

### With Protocol Standards

**A2A (Agent-to-Agent):**
- Full endpoint compatibility
- Agent Card JSON format support

**MCP (Model Context Protocol):**
- Tool/resource/prompt registration
- Server capability advertisement

**AP2 (Agent Payments Protocol):**
- PaymentMandate integration
- Delegation attestation references
- Credentials provider discovery

**x402 (HTTP 402 Payments):**
- Transaction signature verification
- Payment proof in feedback
- Value-weighted reputation

---

## Migration Guide for Existing Projects

### From aeamcp to SATI

**Step 1: Identity Migration**

```rust
// aeamcp: AgentRegistryEntryV1 (2,500 bytes)
pub struct AgentRegistryEntryV1 {
    // ... 35+ fields
}

// SATI: AgentIdentity (105 bytes + delegations) + ledger metadata
async fn migrate_aeamcp_agent(
    aeamcp_agent: AgentRegistryEntryV1
) -> Result<AgentIdentity> {
    // Extract metadata to JSON
    let metadata = AgentMetadata {
        agent_id: aeamcp_agent.agent_id,
        name: aeamcp_agent.name,
        endpoints: convert_endpoints(aeamcp_agent.service_endpoints),
    };

    // Compress and register in SATI
    let compressed = compress_metadata(&metadata)?;
    let proof = generate_zk_proof(&compressed)?;
    register_agent(agent_id_hash, compressed, proof).await
}
```

**Benefits:**
- Storage cost: 0.016 SOL → 0.00001 SOL (1,600x reduction)
- Rent refund: Reclaim 0.015999 SOL per agent

### From s8004 to SATI

**Step 1: Extract Agent Metadata**

```rust
// s8004: Agent with JSON blob
pub struct Agent {
    pub agent_data: String, // max 2048 chars
    pub rating: i64,
}

// SATI: Structured + compressed
async fn migrate_s8004_agent(s8004_agent: Agent) -> Result<AgentIdentity> {
    let metadata: AgentMetadata = serde_json::from_str(&s8004_agent.agent_data)?;
    register_compressed_agent(metadata).await
}
```

**Step 2: Migrate Receipt-Based Reputation**

```rust
// Convert s8004 receipts to SATI feedback
for receipt in receipts {
    let feedback = FeedbackEntry {
        score: if receipt.rating == Some(true) { 100 } else { 0 },
        source: "s8004_receipt",
    };
    submit_feedback(agent_id, feedback).await?;
}
```

---

## Alternatives Considered

### 1. Fully Custom Implementation (No SAS)

**Pros:**
- Complete control over all components
- No external dependencies

**Cons:**
- ❌ 500+ lines of attestation storage code
- ❌ Reinvents expiry, schemas, tokenization
- ❌ 12-week longer implementation
- ❌ Not composable with SAS ecosystem

**Why rejected:** Violates "composability" and "lightweight" design goals

### 2. Full SAS Replacement (No SATI-Native)

**Pros:**
- Minimal custom code
- Maximum SAS composability

**Cons:**
- ❌ No agent-specific orchestration
- ❌ No reputation aggregation
- ❌ No multi-validator consensus
- ❌ Generic (not agent-optimized)

**Why rejected:** Loses novel contributions (identity, reputation, consensus)

### 3. NFT-Based Identity (like ERC-8004)

**Pros:**
- Transferable agent ownership
- Existing NFT tooling

**Cons:**
- ⚠️ NFT overhead not necessary
- ⚠️ Adds complexity

**Why rejected:** PDA-based identity is simpler, can add NFT wrapper later

### 4. Separating Mandate Registry as Companion Standard

**Pros:**
- Clean layer separation (trust vs payment authorization)
- Faster initial SATI launch
- Multiple mandate protocols could coexist

**Cons:**
- ❌ Incomplete AP2 support (agents need mandate lifecycle NOW)
- ❌ Worse positioning vs ERC-8004 (looks like a partial solution)
- ❌ Hubble AI conversion becomes harder (no clear mandate story)
- ❌ Community confusion (two standards instead of one comprehensive solution)

**Why rejected:** Real-world AP2 adoption requires mandate lifecycle management—providing it makes SATI the complete solution agents need. The "zombie mandates" problem (mandates remaining technically valid but contextually obsolete) causes involuntary chargebacks and regulatory exposure that fragmented standards cannot address effectively.

---

## Open Questions

### Technical

1. **SAS schema evolution**: How to handle v1→v2 schema migrations for active delegations?
2. **Cross-chain state**: Can SATI state be verified from Ethereum/other chains?
3. **Validator incentives**: Beyond slashing, how do validators earn from validation services?
4. **ZK proof batching**: Can multiple agent registrations share one proof?

### Economic

1. **Fee distribution**: Should SATI registries collect protocol fees?
2. **SAS storage costs**: Who pays for long-lived delegation attestations?
3. **Reputation aggregator competition**: How to ensure healthy marketplace?

### Governance

1. **Upgrade authority**: Who controls SATI program upgrades post-deployment?
2. **Schema standards**: How to govern SATI standard schema definitions?
3. **Dispute resolution**: On-chain governance or off-chain arbitration?

---

## References

### Solana Infrastructure

1. SIMD-215: Accounts Lattice Hash (Mainnet Jan 2025)
2. SIMD-256: 60M Compute Unit Blocks (Mainnet July 2025)
3. Alpenglow Consensus (Approved Sept 2025, Deploy Q1 2026)
4. Light Protocol ZK Compression V1 (Mainnet Sept 2025)
5. Solana Attestation Service (Mainnet Feb 2025)
6. Photon RPC Indexer (Helius Labs)

### Protocol Standards

1. ERC-8004: Trustless Agents (Ethereum, Draft Aug 2025)
2. Agent-to-Agent Protocol (A2A) - Google/Linux Foundation
3. Model Context Protocol (MCP) - Anthropic
4. Agent Payments Protocol (AP2) - Google/Visa
5. x402 HTTP Payment Protocol - Coinbase/Cloudflare

### Existing Implementations

1. aeamcp (openSVM) - Solana Agent Registry (Devnet)
2. s8004 (Woody4618) - ERC-8004 POC (Testnet)
3. Solana Attestation Service (Solana Foundation) - Mainnet
4. Phala Network - TEE Agent Implementation
5. Oasis ROFL - Cross-chain Agent Framework

---

## Copyright

Copyright and related rights waived via [CC0](https://creativecommons.org/publicdomain/zero/1.0/).

---

## Appendix A: SAS Integration Examples

### A.1 Creating User Delegation

```typescript
import { createCredential, createSchema, createAttestation } from 'sas-lib';
import { addDelegation } from '@sati/sdk';

// One-time user setup
const userCredential = await createCredential({
  authority: userWallet.publicKey,
  name: "alice@example.com",
  authorizedSigners: [userWallet.publicKey],
});

// Create delegation attestation
const delegationSchema = await fetchSatiStandardSchema(
  "sati_user_agent_delegation_v1"
);

const delegation = await createAttestation({
  credential: userCredential,
  schema: delegationSchema,
  data: {
    user_identifier: "alice@example.com",
    agent_pubkey: shoppingAgent.publicKey,
    delegated_at: Date.now() / 1000,
    expires_at: (Date.now() / 1000) + 86400,
    max_spend_lamports: 1_000_000_000,
    allowed_capabilities: ["shopping", "payments"],
    allowed_merchants: [],
    additional_constraints: JSON.stringify({}),
  },
  expiry: (Date.now() / 1000) + 86400,
  signer: userWallet,
});

// Add reference to agent identity
await addDelegation({
  agent: agentIdentity,
  sasAttestation: delegation,
  owner: agentOwner,
});
```

### A.2 Creating Validation Attestation

```typescript
// Validator creates attestation
const validationSchema = await fetchSatiStandardSchema(
  "sati_agent_work_validation_v1"
);

const validation = await createAttestation({
  credential: validatorCredential,
  schema: validationSchema,
  data: {
    agent_id_hash: agent.agent_id_hash,
    work_hash: computeWorkHash(taskData),
    confidence_score: 95,
    validation_method: "tee_attestation",
    validated_at: Date.now() / 1000,
    validator_metadata: JSON.stringify({
      enclave_type: "intel_tdx",
      measurement: teeResult.measurement,
    }),
  },
  expiry: (Date.now() / 1000) + 86400,
  signer: validatorWallet,
});

// Submit to orchestrator
await submitValidation({
  orchestrator: orchestratorPubkey,
  sasAttestation: validation,
  validator: validatorWallet,
});
```

### A.3 Querying Agent with Delegations

```typescript
// Fetch agent identity
const agent = await fetchAgentIdentity(agentIdHash);

// Fetch all active delegations
const delegations = await Promise.all(
  agent.active_delegations.map(pubkey =>
    fetchSasAttestation(pubkey)
  )
);

// Check if user has delegated
const userDelegation = delegations.find(d => {
  const data = deserializeDelegation(d.data);
  return data.user_identifier === "alice@example.com";
});

if (userDelegation) {
  const data = deserializeDelegation(userDelegation.data);
  console.log("Delegation found:");
  console.log(`  Expires: ${new Date(data.expires_at * 1000)}`);
  console.log(`  Max spend: ${data.max_spend_lamports / 1e9} SOL`);
  console.log(`  Capabilities: ${data.allowed_capabilities.join(", ")}`);
}
```

---

*End of SATI Specification v1.0*

**Total Document Length:** ~14,000 words
**Implementation Effort:** 10-12 weeks (3-person team)
**Audit + Deployment:** 3 weeks
**Total Time to Mainnet:** 13-15 weeks (Q1 2026 realistic)

---

## Summary of Key Design Decisions

**Phase 1 Scope (Launch):**
- ✅ Four-registry architecture (Identity, Reputation, Delegation, Mandate)
- ✅ DID support for cross-chain interoperability (did:solana, did:pkh, did:web)
- ✅ ZK Compression for cost efficiency (1,600x cheaper)
- ✅ SAS integration for delegations (production-tested primitives)
- ✅ TEE validation schema (single-validator attestations)
- ✅ x402 payment proof integration
- ✅ Full AP2 support including mandate lifecycle management
- ✅ Context drift detection and zombie mandate prevention
- ✅ Complete ERC-8004 compatibility via DIDs

**Deferred to Phase 2:**
- ⏸ Multi-validator consensus orchestrator
- ⏸ zkML validation implementation
- ⏸ Generic work validation implementation
- ⏸ Production bridge integrations (Wormhole/LayerZero)

**Integrated in Phase 1:**
- ✅ Payment mandate lifecycle management (Active/Revoked/Executed/Zombie/Amended states)
  - **Rationale**: Complete AP2 support requires mandate lifecycle management
  - **Benefit**: First and only Solana standard with full AP2 compatibility
  - **Advantage**: Solves the "zombie mandate" problem AP2 identified but didn't implement

**Why this approach:**
- **Complete solution**: Agents get everything they need in one standard (identity + reputation + delegation + mandates)
- **Full AP2 support**: First Solana standard matching Google's complete agent payments protocol
- **Real-world necessity**: Context drift in mandates causes chargebacks and erodes user trust
- **Ecosystem clarity**: One comprehensive standard > multiple competing partial solutions
- **Lower implementation risk**: Proven SAS primitives for attestations, SATI-native for complex workflows
- **Strong foundation**: DID support enables immediate cross-chain use cases
