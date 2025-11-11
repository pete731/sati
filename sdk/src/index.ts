/**
 * SATI SDK - Solana Agent Trust Infrastructure
 *
 * TypeScript SDK for interacting with SATI registries:
 * - Identity Registry: Agent identity with ZK compression
 * - Reputation Registry: Pre-authorized client feedback
 * - Mandate Registry: AP2 mandate lifecycle management
 *
 * @packageDocumentation
 */

export * from "./helpers";
export * from "./identity";
export * from "./reputation";
export * from "./mandate";

// Re-export types for convenience
export type { Address } from "@solana/kit";
export { address } from "@solana/kit";
