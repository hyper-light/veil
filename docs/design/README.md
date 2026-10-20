# Veil's design

The target, from track T8 of kiln's critical path (kiln plan `01a157fe1045705180b43d9dcb442a2e`). Each decision below is
made by the goal named beside it, recorded here with its sources, its evidence and what would
falsify it (CLAUDE.md §5).

## What it is

Veil injects, generates, rotates and controls secrets just in time. It holds:
- `veil-core`: keys, sealing, versions, leases and policy, sans-IO;
- `veil`: the service;
- `veil-sm`: the Secrets Manager API;
- `veil-inject`: the injector at egress;
- `veil-api`: the traits others use, with a simulator implementation;
- the `veil` binary.

It keeps its records in Amphora, its audit archive, encrypted backups and large secrets on Mantle,
and its events on Quiver; it is Mantle's key manager. Kiln vendors it at `vendor/veil` and reaches
it only through kiln-seams' `Secrets`.

## The target

- **Keys.** A root key sealed by the platform where it can be (TPM 2.0 on Linux and Windows, the
  Secure Enclave or the Keychain on macOS), otherwise a cloud KMS the operator names, otherwise
  unsealed from shares (Shamir, as Vault does it). Under it, a key per namespace, then a data key
  per version; AEAD through AWS-LC as Mantle vendors it; every key rotates and re-wraps online.
- **The model, after Vault and Secrets Manager:** namespaces and paths; secrets with versions and
  the stages AWSCURRENT, AWSPREVIOUS and AWSPENDING; leases for dynamic secrets; identities for
  people, agents, runners and Hex workloads; path policies and resource policies; approvals where
  a person must say yes.
- **Just in time.** `veil-inject` sits at each workload's egress and substitutes the real
  credential for a placeholder on granted hosts, terminating TLS with a CA only that workload
  trusts. Tokens are minted per run. Files and environment variables only where nothing else
  works: on tmpfs, revoked when the run ends.
- **Leak prevention:** live values and their encodings redacted from what kiln keeps and shows;
  secret-shaped strings caught before a push; canary tokens.
- **Compared with** HashiCorp Vault or OpenBao on the same hardware, and Secrets Manager.

## Decisions

| Decision | Made by | Status |
|---|---|---|
| Root key sealing per platform, and the key hierarchy | CP-T8.1 | open |
| The Secrets Manager protocol and SigV4 from Mantle's `mantle-s3` | CP-T8.2 | open |
| The injector: placeholders, the per-workload CA, OAuth refresh outside | CP-T8.3 | open |
| Dynamic secrets and leases | CP-T8.4 | open |
| Rotation through staged versions | CP-T8.5 | open |
| Policies, segmentation and approvals | CP-T8.6 | open |
| Leak prevention | CP-T8.7 | open |
| Veil and Mantle, both ways | CP-T8.8 | open |
| Replication and regions | CP-T8.9 | open |
