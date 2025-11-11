import * as anchor from "@coral-xyz/anchor";
import { toAddress, toPublicKey } from "./helpers";

/**
 * SATI Identity Registry SDK
 *
 * Manages agent identities with ZK compression, DID support,
 * and delegation/validation references.
 */
export class SATIIdentity {
  constructor(
    private program: anchor.Program,
    private provider: anchor.AnchorProvider
  ) {}

  /**
   * Derive agent identity PDA from agent_id string
   */
  deriveAgentPDA(agentId: string): [anchor.web3.PublicKey, number] {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("agent"), Buffer.from(agentId)],
      this.program.programId
    );
  }

  /**
   * Register a new agent identity
   *
   * @param agentId - Unique agent identifier string
   * @returns Agent PDA public key
   */
  async registerAgent(agentId: string): Promise<anchor.web3.PublicKey> {
    const [agentPda] = this.deriveAgentPDA(agentId);

    await this.program.methods
      .registerAgent(agentId)
      .accounts({
        agent: agentPda,
        owner: this.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    return agentPda;
  }

  /**
   * Build register agent instruction (for bundled transactions)
   *
   * @param agentId - Unique agent identifier string
   * @returns Transaction instruction
   */
  async buildRegisterAgentInstruction(
    agentId: string
  ): Promise<anchor.web3.TransactionInstruction> {
    const [agentPda] = this.deriveAgentPDA(agentId);

    return await this.program.methods
      .registerAgent(agentId)
      .accounts({
        agent: agentPda,
        owner: this.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .instruction();
  }

  /**
   * Add delegation reference to agent
   *
   * @param agentId - Agent identifier
   * @param sasAttestation - SAS attestation public key
   */
  async addDelegation(
    agentId: string,
    sasAttestation: anchor.web3.PublicKey
  ): Promise<void> {
    const [agentPda] = this.deriveAgentPDA(agentId);

    await this.program.methods
      .addDelegation(sasAttestation)
      .accounts({
        agent: agentPda,
        owner: this.provider.wallet.publicKey,
      })
      .rpc();
  }

  /**
   * Fetch agent identity account
   *
   * @param agentId - Agent identifier
   * @returns Agent identity data or null
   */
  async fetchAgentIdentity(agentId: string): Promise<any | null> {
    const [agentPda] = this.deriveAgentPDA(agentId);

    try {
      return await this.program.account.agentIdentity.fetch(agentPda);
    } catch (error) {
      return null;
    }
  }
}

/**
 * Factory function to create SATI Identity client
 */
export async function createSATIIdentityClient(
  connection: anchor.web3.Connection,
  wallet: anchor.Wallet,
  idl: any
): Promise<SATIIdentity> {
  const provider = new anchor.AnchorProvider(connection, wallet, {});
  const program = new anchor.Program(idl, provider);
  return new SATIIdentity(program, provider);
}
