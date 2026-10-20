# veil
Agentic just-in-time secrets - local or distributed.

Veil is just-in-time secrets for people and their agents. AWS Secrets Manager's API, Vault-like grouping and segmentation, and no key ever within an agent's
reach: credentials are filled in at a workload's egress, never handed to it.

**Status:** the repository has its bar (`CLAUDE.md`, the gate, the requirement registry); the
`veil` binary says what it is, and nothing more yet. `STATUS.md` says what works and what's next;
`docs/design/` is the design.

```sh
cargo build --release      # target/release/veil
bash scripts/gates.sh      # every gate, as kiln CI runs them
```

MIT licensed (`LICENSE`).
