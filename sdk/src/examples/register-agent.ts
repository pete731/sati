/**
 * Example: Register Agent Identity
 *
 * This example shows how to register a new agent identity
 * in the SATI Identity Registry.
 */

import * as anchor from "@coral-xyz/anchor";
import { createSATIIdentityClient } from "../identity";

async function main() {
  // Setup connection and wallet
  const connection = new anchor.web3.Connection("https://api.devnet.solana.com");
  const wallet = anchor.Wallet.local(); // Or use your wallet

  // Load IDL (replace with actual IDL)
  const idl = {}; // TODO: Load from file

  // Create SATI Identity client
  const satiIdentity = await createSATIIdentityClient(connection, wallet, idl);

  // Register agent
  const agentId = "my-trading-agent";
  const agentPda = await satiIdentity.registerAgent(agentId);

  console.log("Agent registered!");
  console.log("Agent PDA:", agentPda.toBase58());

  // Fetch agent identity
  const agentData = await satiIdentity.fetchAgentIdentity(agentId);
  console.log("Agent data:", agentData);
}

// Run example
if (require.main === module) {
  main()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error(error);
      process.exit(1);
    });
}
