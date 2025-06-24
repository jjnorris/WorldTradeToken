# Tokenomics

WorldTradeToken (WTT) has a fixed supply cap of 42,000,000 tokens. Distribution occurs exclusively via mining rewards using the custom WTT-X proof-of-work algorithm.

All amounts are tracked with 18 decimal places to ensure precision when sending very small values.

## Emission Curve

Rewards follow a logarithmic decay:

```
reward = 50 / sqrt(block_height / 5_000_000 + 1)
```

This emits roughly 50% of the total supply in the first 20–25 years with the remainder mined over the following decades.

Approximate cumulative supply projection:

| Year | Mined Supply (M WTT) |
|----|----|
|10|14|
|20|21|
|30|28|
|40|34|
|50|40|
|60|42|

## Founder Vesting

2.1 million WTT are locked in a vesting contract with a 6‑month cliff and 4‑year linear release schedule. A Rust implementation lives in `wtt-vesting/` and will later be moved on-chain. Unlock status will be visible in the explorer and wallet.
