/**
 * SATI Standard: User-Agent Delegation Schema v1
 * Compatible with Solana Attestation Service (SAS)
 *
 * This file defines the standard delegation schema for SATI.
 * Actual SAS attestation creation requires sas-lib SDK.
 */

/**
 * SATI Delegation Schema v1 Layout
 *
 * Maps to SAS SchemaDataTypes:
 * - String: user_identifier
 * - VecU8: agent_pubkey (32 bytes)
 * - I64: delegated_at
 * - I64: expires_at
 * - U64: max_spend_lamports
 * - VecString: allowed_capabilities
 * - VecString: allowed_merchants
 * - String: additional_constraints (JSON)
 */
export const SATI_DELEGATION_SCHEMA_V1_LAYOUT = [
  0,  // String: user_identifier
  6,  // VecU8: agent_pubkey
  10, // I64: delegated_at
  10, // I64: expires_at
  9,  // U64: max_spend_lamports
  7,  // VecString: allowed_capabilities
  7,  // VecString: allowed_merchants
  0,  // String: additional_constraints
];

export const SATI_DELEGATION_SCHEMA_V1_FIELDS = [
  "user",
  "agent",
  "from",
  "until",
  "max_spend",
  "capabilities",
  "merchants",
  "constraints",
];

export const SATI_DELEGATION_SCHEMA_NAME = "sati_user_agent_delegation_v1";
export const SATI_DELEGATION_SCHEMA_DESCRIPTION =
  "User authorizes agent to act with specific capabilities and spending limits";
export const SATI_DELEGATION_SCHEMA_VERSION = 1;

/**
 * Delegation data structure
 */
export interface DelegationData {
  user_identifier: string;
  agent_pubkey: Uint8Array; // 32 bytes
  delegated_at: number; // Unix timestamp
  expires_at: number; // Unix timestamp, 0 = never
  max_spend_lamports: number; // 0 = unlimited
  allowed_capabilities: string[];
  allowed_merchants: string[];
  additional_constraints: string; // JSON
}

/**
 * Example delegation data
 */
export const exampleDelegation: DelegationData = {
  user_identifier: "alice@example.com",
  agent_pubkey: new Uint8Array(32), // Replace with actual pubkey bytes
  delegated_at: Math.floor(Date.now() / 1000),
  expires_at: Math.floor(Date.now() / 1000) + 86400, // 24 hours
  max_spend_lamports: 1_000_000_000, // 1 SOL
  allowed_capabilities: ["shopping", "payments"],
  allowed_merchants: [], // All merchants
  additional_constraints: JSON.stringify({
    geographic_restrictions: ["US", "CA"],
    time_restrictions: { start: "09:00", end: "17:00" },
  }),
};
