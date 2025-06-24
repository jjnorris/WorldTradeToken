# wtt-governance

Minimal Rust crate implementing a simple proposal and voting system for
WorldTradeToken. Proposals transition from `Pending` to `Active` and finally to
`Passed` or `Rejected` based on the outcome of votes and a quorum check.

```bash
# run tests
cargo test --quiet
```

This placeholder will evolve into the on-chain DAO logic where 1 WTT equals one
vote.
