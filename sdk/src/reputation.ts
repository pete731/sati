import * as anchor from "@coral-xyz/anchor";
import { toAddress, toPublicKey } from "./helpers";

/**
 * SATI Reputation Registry SDK
 *
 * Manages pre-authorized client feedback with spam resistance.
 */
export class SATIReputation {
  constructor(
    private program: anchor.Program,
    private provider: anchor.AnchorProvider
  ) {}

  /**
   * Derive feedback authorization PDA
   */
  deriveFeedbackAuthPDA(
    agentIdHash: number[],
    client: anchor.web3.PublicKey
  ): [anchor.web3.PublicKey, number] {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("feedback_auth"),
        Buffer.from(agentIdHash),
        client.toBuffer(),
      ],
      this.program.programId
    );
  }

  /**
   * Authorize client to submit feedback
   *
   * @param agentIdHash - Agent identifier hash
   * @param client - Client public key
   * @param maxSubmissions - Maximum number of feedback submissions
   * @param expiresInSeconds - Authorization expiry duration
   */
  async authorizeFeedback(
    agentIdHash: number[],
    client: anchor.web3.PublicKey,
    maxSubmissions: number,
    expiresInSeconds: number
  ): Promise<anchor.web3.PublicKey> {
    const [authPda] = this.deriveFeedbackAuthPDA(agentIdHash, client);

    await this.program.methods
      .authorizeFeedback(agentIdHash, client, maxSubmissions, expiresInSeconds)
      .accounts({
        authorization: authPda,
        agentOwner: this.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    return authPda;
  }

  /**
   * Submit feedback (client side)
   *
   * @param authPda - Authorization PDA
   * @param feedbackHash - Hash of feedback content
   */
  async submitFeedback(
    authPda: anchor.web3.PublicKey,
    feedbackHash: number[]
  ): Promise<void> {
    await this.program.methods
      .submitFeedback(feedbackHash)
      .accounts({
        authorization: authPda,
        client: this.provider.wallet.publicKey,
      })
      .rpc();
  }
}

/**
 * Factory function to create SATI Reputation client
 */
export async function createSATIReputationClient(
  connection: anchor.web3.Connection,
  wallet: anchor.Wallet,
  idl: any
): Promise<SATIReputation> {
  const provider = new anchor.AnchorProvider(connection, wallet, {});
  const program = new anchor.Program(idl, provider);
  return new SATIReputation(program, provider);
}
