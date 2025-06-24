import express from 'express';
import { WebSocketServer } from 'ws';

const app = express();
const port = process.env.PORT || 9000;

app.use(express.json());

let workers = {};

app.post('/submit', (req, res) => {
  const { worker, hashrate } = req.body;
  if (worker && typeof hashrate === 'number') {
    workers[worker] = hashrate;
    res.json({ status: 'ok' });
  } else {
    res.status(400).json({ error: 'invalid payload' });
  }
});

app.get('/stats', (_req, res) => {
  const totalHashrate = Object.values(workers).reduce((a, b) => a + b, 0);
  res.json({ workers, totalHashrate });
});

const server = app.listen(port, () => {
  console.log(`Pool server listening on ${port}`);
});

const wss = new WebSocketServer({ server });

function broadcast() {
  const totalHashrate = Object.values(workers).reduce((a, b) => a + b, 0);
  const data = JSON.stringify({ workers, totalHashrate });
  for (const client of wss.clients) {
    if (client.readyState === 1) {
      client.send(data);
    }
  }
}

setInterval(broadcast, 5000);
