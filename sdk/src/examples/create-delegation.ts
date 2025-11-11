/**
 * Example: Create User-Agent Delegation
 *
 * This example shows how to create a user-to-agent delegation
 * using Solana Attestation Service (SAS).
 *
 * Note: Requires sas-lib SDK for actual attestation creation.
 */

// TODO: Import sas-lib when available
// import { createCredential, createSchema, createAttestation } from "sas-lib";

import {
  SATI_DELEGATION_SCHEMA_NAME,
  SATI_DELEGATION_SCHEMA_V1_LAYOUT,
  SATI_DELEGATION_SCHEMA_V1_FIELDS,
  exampleDelegation,
} from "../sas/delegation";

async function main() {
  console.log("Creating user-agent delegation...");

  // Step 1: Create user credential (one-time setup)
  // const userCredential = await createCredential({
  //   authority: userWallet.publicKey,
  //   name: "alice@example.com",
  // });

  // Step 2: Create SATI delegation schema
  // const schema = await createSchema({
  //   credential: userCredential,
  //   name: SATI_DELEGATION_SCHEMA_NAME,
  //   layout: SATI_DELEGATION_SCHEMA_V1_LAYOUT,
  //   fieldNames: SATI_DELEGATION_SCHEMA_V1_FIELDS,
  // });

  // Step 3: Create delegation attestation
  // const attestation = await createAttestation({
  //   credential: userCredential,
  //   schema: schema,
  //   data: exampleDelegation,
  //   expiry: exampleDelegation.expires_at,
  // });

  console.log("Delegation schema:", SATI_DELEGATION_SCHEMA_NAME);
  console.log("Example delegation data:", exampleDelegation);

  // Step 4: Add delegation reference to agent identity
  // await satiIdentity.addDelegation(agentId, attestation.publicKey);

  console.log("Delegation created! (example code - requires sas-lib)");
}

if (require.main === module) {
  main()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error(error);
      process.exit(1);
    });
}
