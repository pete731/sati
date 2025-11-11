import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";

describe("SATI Identity Registry", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.IdentityRegistry as Program;

  it("Registers a new agent identity", async () => {
    // TODO: Implement test
    // const agentId = "test-agent";
    // const tx = await program.methods
    //   .registerAgent(agentId)
    //   .accounts({
    //     agent: agentPda,
    //     owner: provider.wallet.publicKey,
    //     systemProgram: anchor.web3.SystemProgram.programId,
    //   })
    //   .rpc();
    //
    // const agentAccount = await program.account.agentIdentity.fetch(agentPda);
    // assert.equal(agentAccount.owner.toBase58(), provider.wallet.publicKey.toBase58());
  });

  it("Adds delegation reference", async () => {
    // TODO: Implement test
  });

  it("Removes delegation reference", async () => {
    // TODO: Implement test
  });

  it("Updates DID", async () => {
    // TODO: Implement test
  });
});
