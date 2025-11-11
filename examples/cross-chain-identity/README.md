# Cross-Chain Identity Example

This example shows how to create an agent identity with cross-chain DID support.

## Use Case

An agent operating on both Solana and Ethereum needs unified identity:
- Solana: SATI Identity Registry
- Ethereum: ERC-8004 Registry
- DID: Cross-chain identifier

## Workflow

1. **Create Solana identity** - Register in SATI
2. **Create cross-chain DID** - did:pkh or did:web
3. **Link Ethereum registry** - Reference ERC-8004 address
4. **Verify cross-chain** - Query both registries

## Implementation (Phase 2)

```typescript
// Register with cross-chain DID
await satiIdentity.registerAgent({
  agentId: "multi-chain-agent",
  did: "did:pkh:eip155:1:0xABC...",
  crossChainReferences: [{
    chain: "ethereum",
    registry: "0xERC8004Address",
    verificationMethod: "wormhole"
  }]
});

// Query both chains
const solanaData = await satiIdentity.fetchAgentIdentity("multi-chain-agent");
const ethereumData = await queryERC8004("0xABC...");

// Aggregate reputation
const totalReputation = aggregateReputation([solanaData, ethereumData]);
```

## Features Demonstrated

- ⏳ DID creation (Phase 1)
- ⏳ Cross-chain verification (Phase 2)
- ⏳ Wormhole/LayerZero integration (Phase 2)
- ⏳ Reputation aggregation (Phase 2)

## Next Steps

- Implement bridge verification
- Add light client support
- Create reputation aggregator service
