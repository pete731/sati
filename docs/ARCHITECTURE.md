# SATI Architecture Overview

## System Design

SATI consists of four independent but composable registries:

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

## Design Rationale: Why This Split?

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

## Data Flow

### Agent Registration
```
Developer → Identity Registry
    ├── ZK compress metadata → Ledger
    ├── Store state root → AccountsDB (549 bytes)
    └── Emit AgentRegistered event → Indexers
```

### User Delegation
```
User → SAS (create delegation attestation)
    ├── Store attestation → SAS program
    └── Reference → Identity Registry (active_delegations)
```

### Feedback Submission
```
Agent → Authorize client (FeedbackAuthorization PDA)
Client → Submit feedback (emit event)
    └── Indexers aggregate → Off-chain reputation scores
```

### Mandate Lifecycle
```
User → Create IntentMandate
    ├── ZK compress details → Ledger
    ├── Store state → Mandate Registry (187 bytes)
    └── Context drift check → Zombie detection

Agent → Check context
    ├── If drifted → Mark as Zombie
    └── Require revalidation → User signature
```

## Cost Analysis

| Operation | Traditional | SATI (ZK + SAS) | Savings |
|-----------|-------------|-----------------|---------|
| Register 1 agent | 0.016 SOL ($3.20) | 0.00001 SOL ($0.002) | 1,600x |
| Register 1M agents | 16,000 SOL ($3.2M) | 10 SOL ($2,000) | 1,600x |
| Submit feedback | 0.008 SOL ($1.60) | 0.000005 SOL ($0.001) | 1,600x |
| Create mandate | 0.008 SOL ($1.60) | 0.00001 SOL ($0.002) | 800x |
| **TOTAL (1M agents)** | **$3.4M** | **$2,000** | **1,700x** |

## Performance Characteristics

| Metric | Pre-Alpenglow | With Alpenglow | Improvement |
|--------|---------------|----------------|-------------|
| Agent registration | 400ms (processed) | 150ms (finalized) | 2.6x faster |
| Feedback submission | 12.8s (finalized) | 150ms (finalized) | 85x faster |
| Mandate lifecycle | 12.8s (finalized) | 150ms (finalized) | 85x faster |

## Security Model

**Identity Registry:**
- Owner-controlled updates
- ZK proof verification (Phase 1 implementation)
- Immutable agent_id_hash

**Reputation Registry:**
- Pre-authorization prevents spam
- Payment proofs prevent Sybil attacks
- Off-chain aggregation allows filtering

**Delegation Registry:**
- User-controlled expiry via SAS
- Agent cannot forge delegations
- Explicit revocation support

**Mandate Registry:**
- User-only revalidation
- Anyone can trigger drift check
- Permissionless expiry enforcement
- Zombie state prevents accidental execution
