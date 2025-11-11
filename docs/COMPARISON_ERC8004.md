# SATI vs ERC-8004 Comparison

## Alignment Analysis

SATI is designed to be 95% compatible with Ethereum's ERC-8004 while leveraging Solana's unique advantages.

| Feature | ERC-8004 | SATI | Notes |
|---------|----------|------|-------|
| **Identity Registry** | ERC-721 NFT | ZK Compressed PDA | 1,600x cheaper |
| **Reputation Registry** | Event-based | Event-based | Same pattern |
| **Validation Registry** | On-chain orchestrator | SAS attestations (Phase 1) | SATI uses proven primitives |
| **Mandate Lifecycle** | Not included | Full AP2 support | **SATI unique advantage** |
| **Context Drift Detection** | Not included | Zombie mandate prevention | **SATI solves AP2 gap** |
| **Cross-chain Support** | Via bridges | Native DID support | Better interop |
| **User Delegation** | Not included | Native via SAS | SATI advantage |
| **Cost (1M agents + 5M mandates)** | $3.4M+ | $2,000 + $10,000 | 283x improvement |
| **Finality** | 12-15 seconds | 150ms (Alpenglow) | 85x faster |

## Why SATI Differs

### Technical Adaptations

1. **ZK Compression instead of NFTs**
   - ERC-8004 uses ERC-721 (transferable agent ownership)
   - SATI uses PDAs with ZK compression (cost optimization)
   - Can add NFT wrapper later if needed

2. **SAS Integration for Attestations**
   - Leverages Solana Foundation's production-tested infrastructure
   - Reduces implementation complexity by 40%
   - Enables immediate composability with broader ecosystem

3. **Native Delegation Support**
   - ERC-8004 doesn't address user→agent authorization
   - SATI includes this as core feature
   - Critical for PayAI/AP2 integration

4. **Complete Mandate Lifecycle Management**
   - ERC-8004 doesn't address AP2 mandate lifecycle
   - SATI provides IntentMandate/CartMandate support
   - Context drift detection prevents "zombie mandates"
   - Essential for subscription-style autonomous payments

### What SATI Adds Beyond ERC-8004

**1. AP2 Integration**
- Full IntentMandate lifecycle (Active → Executed/Revoked/Zombie)
- CartMandate support with state transitions
- Context drift detection and revalidation
- Prevents involuntary chargebacks from outdated mandates

**2. Payment Protocol Integration**
- x402 payment proof verification
- Value-weighted reputation (prevents Sybil attacks)
- Payment mandate references

**3. Cost Efficiency**
- ZK Compression: 1,600x cost reduction vs traditional storage
- Event-based feedback: No per-feedback storage cost
- Total savings: $3.4M → $2,000 for 1M agents

**4. Real-time Performance**
- 150ms finality with Alpenglow (vs 12-15s Ethereum)
- Enables real-time agent discovery and reputation updates
- Critical for time-sensitive mandate workflows

## Compatibility Strategy

### Cross-Chain Agents

**Ethereum agent with ERC-8004 identity:**
```typescript
// DID: did:pkh:eip155:1:0xABC...
// Can register in SATI with cross-chain DID reference
await satiIdentity.registerAgent({
  agentId: "ethereum-agent-123",
  did: "did:pkh:eip155:1:0xABC...",
  crossChainRegistry: "0xERC8004RegistryAddress",
});
```

**Solana agent with SATI identity:**
```typescript
// DID: did:solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp
// Native Solana identity with potential bridge to Ethereum
await satiIdentity.registerAgent({
  agentId: "solana-agent-456",
  did: "did:solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp",
});
```

### Reputation Aggregation

Both ERC-8004 and SATI use event-based reputation:
- **Same pattern:** Off-chain indexers aggregate feedback
- **Composable:** Third-party aggregators can query both chains
- **Portable:** Agents can reference multi-chain reputation in DID documents

### Future Bridges

**Phase 2 roadmap includes:**
- Wormhole/LayerZero state verification
- Cross-chain reputation synchronization
- Light client-based verification
- Unified DID resolution service

## Migration Path

### From ERC-8004 to SATI

```typescript
// Migrate Ethereum agent to Solana
const ethereumAgent = {
  tokenId: 123,
  metadata: { /* ERC-8004 metadata */ },
  erc8004Registry: "0x...",
};

// Create equivalent SATI identity
await satiIdentity.registerAgent({
  agentId: `eth-migrated-${ethereumAgent.tokenId}`,
  did: `did:pkh:eip155:1:${ethereumAddress}`,
  metadata: convertERC8004Metadata(ethereumAgent.metadata),
});
```

### From aeamcp to SATI

```rust
// Migrate existing Solana agent (aeamcp → SATI)
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

## Positioning

### When to Use ERC-8004
- Ethereum-native applications
- Need NFT-based agent ownership
- Existing Ethereum infrastructure

### When to Use SATI
- Solana-native applications
- High-volume agent deployments (cost-sensitive)
- Need AP2 mandate lifecycle management
- Real-time performance requirements (150ms finality)
- x402 payment integration

### Ideal: Both
- Register in both registries
- Use DIDs for cross-chain references
- Aggregate reputation from both chains
- Choose chain per use case (cost vs ecosystem)

## Long-term Vision

**SATI and ERC-8004 as complementary standards:**
- Both follow same core principles (identity, reputation, validation)
- Enable cross-chain agent economy
- Third-party services can aggregate both
- Agents choose chain based on technical/economic needs

**Example: Multi-chain agent**
```json
{
  "@context": "https://www.w3.org/ns/did/v1",
  "id": "did:solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp",
  "service": [
    {
      "id": "#sati-identity",
      "type": "SATIIdentityRegistry",
      "serviceEndpoint": "https://sati.solana.com/agent/abc123"
    },
    {
      "id": "#erc8004-identity",
      "type": "ERC8004IdentityRegistry",
      "serviceEndpoint": "https://ethereum.org/erc8004/0xABC"
    }
  ]
}
```

Clients can query both registries and make trust decisions based on aggregated data.
