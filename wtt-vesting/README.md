# wtt-vesting

Rust helper crate implementing the founder vesting schedule for WorldTradeToken.
Tokens are locked for a 6‑month cliff followed by a 4‑year linear release.

```bash
# calculate vested amount one year after start
time cargo run --example usage
```

This logic will later be integrated into the on-chain runtime.
