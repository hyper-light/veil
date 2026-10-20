# Status

Updated 2026-10-20. A piece is done when its tests pass in the gate (`scripts/gates.sh`: Linux, with the
macOS and Windows lints) and the evidence its requirement names is recorded.

## Implemented

- **The bar** (kiln's CP-M1): a Cargo workspace whose lints deny panics, this repository's rules
  (`CLAUDE.md`), the requirement registry (`docs/requirements.toml`), the design's starting point
  (`docs/design/README.md`), the gate (`scripts/gates.sh`, run by kiln CI from `.kiln/ci.yaml`), and the
  vendoring tool (`scripts/vendor.py`, kiln's).
- **The `veil` binary** says what it is (`veil --version`, `veil --help`), and nothing more yet.

## Next

- **CP-T8.1**: `veil-core` (a platform-sealed root, keys per namespace and version, versions and stages, the audit chain) as kiln's secrets backend, every version migrated.
- **CP-T8.2**: The AWS Secrets Manager-compatible API.
- **CP-T8.3**: Just-in-time injection for agents at egress.

## Open

Everything else the track's plan names (kiln plan `01a157fe1045705180b43d9dcb442a2e`): each goal, its criteria and how each is
measured are in `docs/requirements.toml`.
