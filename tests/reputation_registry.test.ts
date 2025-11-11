import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";

describe("SATI Reputation Registry", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ReputationRegistry as Program;

  it("Authorizes client for feedback", async () => {
    // TODO: Implement test
  });

  it("Submits feedback with authorization", async () => {
    // TODO: Implement test
  });

  it("Revokes feedback authorization", async () => {
    // TODO: Implement test
  });
});
