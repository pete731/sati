/**
 * Example: AP2 Integration with Mandate Lifecycle
 *
 * This example shows how to integrate SATI with Google's Agent Payments Protocol (AP2)
 * for complete mandate lifecycle management.
 */

import * as anchor from "@coral-xyz/anchor";
import { createSATIMandateClient } from "../mandate";
import { createSATIIdentityClient } from "../identity";

async function main() {
  // Setup connection and wallet
  const connection = new anchor.web3.Connection("https://api.devnet.solana.com");
  const wallet = anchor.Wallet.local();

  // Load IDLs
  const identityIdl = {}; // TODO: Load identity IDL
  const mandateIdl = {}; // TODO: Load mandate IDL

  // Create clients
  const satiIdentity = await createSATIIdentityClient(connection, wallet, identityIdl);
  const satiMandate = await createSATIMandateClient(connection, wallet, mandateIdl);

  // Step 1: User creates IntentMandate (AP2 spec)
  const ap2IntentMandate = {
    user: "alice@example.com",
    intent: "Buy 2 concert tickets < $1000, close to stage",
    budget: 1000_000_000, // lamports
  };

  // Step 2: Register IntentMandate in SATI
  const mandateId = "intent_" + Date.now();
  const agentIdHash = new Array(32).fill(0); // Example hash

  // Compute context hash (location, budget, preferences)
  const userContext = {
    location: { country: "US", state: "CA" },
    budget: 1000_000_000,
    preferences: { priority: "quality_over_price" },
  };
  const contextHash = new Array(32).fill(0); // TODO: Hash context

  const mandatePda = await satiMandate.createMandate(
    mandateId,
    agentIdHash,
    0, // MandateType.Intent
    contextHash,
    3600, // Expires in 1 hour
    1800, // Revalidate after 30 min if not executed
    1 // Single execution
  );

  console.log("IntentMandate created:", mandatePda.toBase58());

  // Step 3: Check for context drift before execution
  const hasDrifted = await satiMandate.checkContextDrift(mandateId, contextHash);

  if (hasDrifted) {
    console.log("Context drift detected! Requires user revalidation.");

    // User revalidates with updated context
    const newContextHash = new Array(32).fill(1); // Updated context
    await satiMandate.revalidateMandate(mandateId, newContextHash);

    console.log("Mandate revalidated!");
  }

  // Step 4: Execute payment (would integrate with actual payment processor)
  console.log("Mandate ready for execution!");

  // Fetch mandate status
  const mandateData = await satiMandate.fetchMandate(mandateId);
  console.log("Mandate data:", mandateData);
}

if (require.main === module) {
  main()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error(error);
      process.exit(1);
    });
}
