# wtt-chain

A minimal Rust prototype for the WorldTradeToken blockchain node. This example exposes the block
reward calculation and a helper to approximate cumulative supply.

```
# calculate reward and total supply for block 1000
cargo run --release -- 1000
```

The real chain will be based on Substrate. This crate provides a starting point
for the reward logic and tests. A constant `MAX_SUPPLY` defines the hard cap of
42 million WTT.
