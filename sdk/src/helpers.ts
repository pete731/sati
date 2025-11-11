import { address, type Address } from "@solana/kit";
import * as anchor from "@coral-xyz/anchor";

/**
 * Convert Anchor PublicKey to @solana/kit Address
 *
 * Use this when calling @solana-program/token functions that expect Address type
 */
export function toAddress(pubkey: anchor.web3.PublicKey): Address {
  return address(pubkey.toBase58());
}

/**
 * Convert @solana/kit Address to Anchor PublicKey
 *
 * Use this when passing Kit results to Anchor .accounts() or instructions
 */
export function toPublicKey(addr: Address): anchor.web3.PublicKey {
  return new anchor.web3.PublicKey(addr);
}

/**
 * Convert @solana/kit instruction to Anchor TransactionInstruction
 *
 * Account role mapping:
 * - Role 0: Read-only
 * - Role 1: Writable
 * - Role 2: Signer
 * - Role 3: Writable + Signer
 */
export function kitInstructionToAnchor(
  kitInstruction: {
    accounts: Array<{ address: Address; role: number }>;
    programAddress: Address;
    data: Uint8Array;
  }
): anchor.web3.TransactionInstruction {
  return new anchor.web3.TransactionInstruction({
    keys: kitInstruction.accounts.map((acc) => ({
      pubkey: toPublicKey(acc.address),
      isSigner: acc.role === 2 || acc.role === 3,
      isWritable: acc.role === 1 || acc.role === 3,
    })),
    programId: toPublicKey(kitInstruction.programAddress),
    data: Buffer.from(kitInstruction.data),
  });
}
