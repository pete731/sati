/**
 * SATI Standard: TEE Validation Schema v1
 * Compatible with Solana Attestation Service (SAS)
 *
 * This file defines the standard validation schema for SATI.
 * Actual SAS attestation creation requires sas-lib SDK.
 */

/**
 * SATI TEE Validation Schema v1 Layout
 *
 * Maps to SAS SchemaDataTypes:
 * - Hash32: agent_id_hash
 * - Hash32: work_hash
 * - String: enclave_type
 * - VecU8: measurement
 * - VecU8: signature
 * - Hash32: output_hash
 * - I64: validated_at
 * - String: metadata (JSON)
 */
export const SATI_TEE_VALIDATION_SCHEMA_V1_LAYOUT = [
  4,  // Hash32: agent_id_hash
  4,  // Hash32: work_hash
  0,  // String: enclave_type
  6,  // VecU8: measurement
  6,  // VecU8: signature
  4,  // Hash32: output_hash
  10, // I64: validated_at
  0,  // String: metadata
];

export const SATI_TEE_VALIDATION_SCHEMA_V1_FIELDS = [
  "agent",
  "work",
  "enclave_type",
  "measurement",
  "signature",
  "output",
  "timestamp",
  "metadata",
];

export const SATI_TEE_VALIDATION_SCHEMA_NAME = "sati_tee_attestation_v1";
export const SATI_TEE_VALIDATION_SCHEMA_DESCRIPTION =
  "TEE-based work validation with enclave measurements";
export const SATI_TEE_VALIDATION_SCHEMA_VERSION = 1;

/**
 * TEE validation data structure
 */
export interface TEEValidationData {
  agent_id_hash: Uint8Array; // 32 bytes
  work_hash: Uint8Array; // 32 bytes
  enclave_type: string; // "intel_tdx", "amd_sev", "phala", "aws_nitro", "azure_cc"
  measurement: Uint8Array;
  signature: Uint8Array;
  output_hash: Uint8Array; // 32 bytes
  validated_at: number; // Unix timestamp
  metadata: string; // JSON
}

/**
 * Example TEE validation data
 */
export const exampleTEEValidation: TEEValidationData = {
  agent_id_hash: new Uint8Array(32),
  work_hash: new Uint8Array(32),
  enclave_type: "intel_tdx",
  measurement: new Uint8Array(64), // Example measurement bytes
  signature: new Uint8Array(64), // Example signature bytes
  output_hash: new Uint8Array(32),
  validated_at: Math.floor(Date.now() / 1000),
  metadata: JSON.stringify({
    node_id: "validator_1",
    tee_version: "2.0",
  }),
};
