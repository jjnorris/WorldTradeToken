# wtt-pool

Simple mining pool backend prototype using Node.js. Workers submit hashrate via
`POST /submit` and the server broadcasts aggregated stats over WebSocket.

```bash
npm install
npm start
```

The `/stats` endpoint returns current worker hashrates and the total pool
hashrate.
