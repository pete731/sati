import * as anchor from "@coral-xyz/anchor";
import { toAddress, toPublicKey } from "./helpers";

/**
 * SATI Mandate Registry SDK
 *
 * Manages AP2 mandate lifecycle with context drift detection.
 */
export class SATIMandate {
  constructor(
    private program: anchor.Program,
    private provider: anchor.AnchorProvider
  ) {}

  /**
   * Derive mandate PDA from mandate_id string
   */
  deriveMandatePDA(mandateId: string): [anchor.web3.PublicKey, number] {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("mandate"), Buffer.from(mandateId)],
      this.program.programId
    );
  }

  /**
   * Create new payment mandate
   *
   * @param mandateId - Unique mandate identifier
   * @param agentIdHash - Agent identity hash
   * @param mandateType - 0=Intent, 1=Cart
   * @param contextHash - Hash of user context (location, budget, preferences)
   * @param expiresInSeconds - Mandate expiry duration (0 = no expiry)
   * @param revalidationThresholdSeconds - Context revalidation threshold
   * @param maxExecutions - Maximum execution count (0 = unlimited)
   */
  async createMandate(
    mandateId: string,
    agentIdHash: number[],
    mandateType: number,
    contextHash: number[],
    expiresInSeconds: number,
    revalidationThresholdSeconds: number,
    maxExecutions: number
  ): Promise<anchor.web3.PublicKey> {
    const [mandatePda] = this.deriveMandatePDA(mandateId);

    await this.program.methods
      .createMandate(
        mandateId,
        agentIdHash,
        mandateType,
        contextHash,
        expiresInSeconds,
        revalidationThresholdSeconds,
        maxExecutions
      )
      .accounts({
        mandate: mandatePda,
        user: this.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    return mandatePda;
  }

  /**
   * Check for context drift
   *
   * @param mandateId - Mandate identifier
   * @param currentContextHash - Current user context hash
   * @returns True if drift detected
   */
  async checkContextDrift(
    mandateId: string,
    currentContextHash: number[]
  ): Promise<boolean> {
    const [mandatePda] = this.deriveMandatePDA(mandateId);

    const result = await this.program.methods
      .checkContextDrift(currentContextHash)
      .accounts({
        mandate: mandatePda,
      })
      .rpc();

    // TODO: Parse return value from transaction
    return false;
  }

  /**
   * Revalidate mandate after zombie detection
   *
   * @param mandateId - Mandate identifier
   * @param newContextHash - Updated user context hash
   */
  async revalidateMandate(
    mandateId: string,
    newContextHash: number[]
  ): Promise<void> {
    const [mandatePda] = this.deriveMandatePDA(mandateId);

    await this.program.methods
      .revalidateMandate(newContextHash)
      .accounts({
        mandate: mandatePda,
        user: this.provider.wallet.publicKey,
      })
      .rpc();
  }

  /**
   * Fetch mandate account
   */
  async fetchMandate(mandateId: string): Promise<any | null> {
    const [mandatePda] = this.deriveMandatePDA(mandateId);

    try {
      return await this.program.account.mandateAccount.fetch(mandatePda);
    } catch (error) {
      return null;
    }
  }
}

/**
 * Factory function to create SATI Mandate client
 */
export async function createSATIMandateClient(
  connection: anchor.web3.Connection,
  wallet: anchor.Wallet,
  idl: any
): Promise<SATIMandate> {
  const provider = new anchor.AnchorProvider(connection, wallet, {});
  const program = new anchor.Program(idl, provider);
  return new SATIMandate(program, provider);
}
