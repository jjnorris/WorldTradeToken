# WorldTradeToken (WTT)

WorldTradeToken is a mineable cryptocurrency targeting long‑term sustainability.
This repository contains multiple modules that together form the prototype
ecosystem.

## Modules

- **wtt-chain/** – Rust reward and supply helpers
- **wtt-wallet/** – TypeScript CLI wallet
- **wtt-explorer/** – Express backend with WebSocket feed
- **wtt-vesting/** – Founder vesting logic in Rust
- **wtt-governance/** – Minimal governance crate
- **wtt-pool/** – Mining pool backend
- **mine.wtt.money/** – React mining dashboard
- **wtt.money/** – Marketing website built with Next.js

## Quick Start

### Chain example
```bash
cd wtt-chain
cargo run --release -- 0
```

### Wallet CLI
```bash
cd wtt-wallet
npm install
npm run build
node dist/index.js generate
node dist/index.js balance
node dist/index.js supply
node dist/index.js sync
```

### Explorer
```bash
cd wtt-explorer
npm install
npm start
```

### Mining pool
```bash
cd wtt-pool
npm install
npm start
```

### Websites
Both `wtt.money` and `mine.wtt.money` are basic web apps. Install dependencies
and start the dev servers:
```bash
cd wtt.money && npm install && npm run dev
cd ../mine.wtt.money && npm install && npm run dev
```

### Vesting tests
```bash
cd wtt-vesting
cargo test --quiet
```

See individual READMEs for details.
