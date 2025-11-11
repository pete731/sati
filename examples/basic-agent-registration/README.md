# Basic Agent Registration Example

This example demonstrates how to register a basic agent identity in SATI.

## Workflow

1. **Generate agent ID** - Create unique identifier for agent
2. **Prepare metadata** - Define agent capabilities, endpoints
3. **Register identity** - Store compressed state on-chain
4. **Verify registration** - Fetch and validate agent data

## Implementation (Phase 1)

```typescript
import { createSATIIdentityClient } from "@sati/sdk";

// Setup client
const satiIdentity = await createSATIIdentityClient(connection, wallet, idl);

// Register agent
const agentPda = await satiIdentity.registerAgent("my-trading-agent");

// Fetch agent data
const agentData = await satiIdentity.fetchAgentIdentity("my-trading-agent");
console.log("Agent registered at:", agentPda.toBase58());
```

## Features Demonstrated

- ✅ Agent identity creation
- ✅ PDA derivation
- ✅ State storage
- ⏳ ZK compression (Phase 1 implementation)
- ⏳ DID registration (Phase 1 implementation)

## Next Steps

- Add delegation attestations
- Submit to reputation registry
- Create payment mandates
