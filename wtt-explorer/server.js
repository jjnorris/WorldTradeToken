import express from 'express';
import { WebSocketServer } from 'ws';

const app = express();
const port = process.env.PORT || 8080;

function reward(h) {
  return 50 / Math.sqrt(h / 5_000_000 + 1);
}

app.get('/', (_req, res) => {
  res.send('WTT Explorer API');
});

app.get('/supply', (_req, res) => {
  res.json({ height, supply });
});

const server = app.listen(port, () => {
  console.log(`Explorer listening on ${port}`);
});

const wss = new WebSocketServer({ server });
let height = 0;
let supply = 0;
wss.on('connection', (ws) => {
  ws.send(JSON.stringify({ height, hash: '0x0', supply }));
});

setInterval(() => {
  height += 1;
  supply += reward(height);
  const block = { height, hash: `0x${height.toString(16)}`, supply };
  wss.clients.forEach((client) => {
    if (client.readyState === 1) {
      client.send(JSON.stringify(block));
    }
  });
}, 1000);
