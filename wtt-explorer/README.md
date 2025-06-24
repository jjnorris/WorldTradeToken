# wtt-explorer

Prototype backend for the WorldTradeToken blockchain explorer. It exposes a simple Express API and a WebSocket server that broadcasts dummy block events every second. The `/supply` endpoint returns the projected total supply based on these dummy blocks.

```
npm install
npm start
```

Query the current supply:

```bash
curl http://localhost:8080/supply
```

Or follow blocks via WebSocket at `ws://localhost:8080`.
