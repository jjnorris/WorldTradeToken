import { Command } from 'commander';
import * as crypto from 'crypto';
import * as fs from 'fs';
import fetch from 'node-fetch';
import WebSocket from 'ws';

const WALLET_PATH = 'wallet.json';
const EXPLORER_API = process.env.EXPLORER_API || 'http://localhost:8080';
const EXPLORER_WS = process.env.EXPLORER_WS || 'ws://localhost:8080';

interface WalletFile {
  privateKey: string;
  address: string;
}

function generate(): void {
  const key = crypto.randomBytes(32).toString('hex');
  const address = crypto.createHash('sha256').update(key).digest('hex').slice(0, 40);
  const data: WalletFile = { privateKey: key, address };
  fs.writeFileSync(WALLET_PATH, JSON.stringify(data, null, 2));
  console.log('Generated address:', address);
}

function loadWallet(): WalletFile {
  const raw = fs.readFileSync(WALLET_PATH, 'utf8');
  return JSON.parse(raw) as WalletFile;
}

function balance(address?: string): void {
  if (!address) {
    const w = loadWallet();
    address = w.address;
  }
  console.log(`Balance for ${address}: 0 WTT (placeholder)`);
}

function send(to: string, amount: string): void {
  const w = loadWallet();
  console.log(`Sending ${amount} WTT from ${w.address} to ${to} (placeholder)`);
}

async function supplyCmd(): Promise<void> {
  const res = await fetch(`${EXPLORER_API}/supply`);
  const data: any = await res.json();
  console.log(`Height ${data.height} supply ${data.supply.toFixed?.(4) ?? data.supply} WTT`);
}

function sync(): void {
  const ws = new WebSocket(EXPLORER_WS);
  ws.on('message', (msg) => {
    const data = JSON.parse(msg.toString());
    if (data.supply !== undefined) {
      console.log(`Block ${data.height} supply ${data.supply.toFixed?.(4) ?? data.supply}`);
    } else {
      console.log('Block', data);
    }
  });
}

const program = new Command();
program
  .name('wtt-wallet')
  .description('CLI wallet for WorldTradeToken');

program.command('generate').description('generate a new keypair').action(generate);
program
  .command('balance [address]')
  .description('check balance of address or saved wallet')
  .action(balance);
program
  .command('send <to> <amount>')
  .description('send WTT from saved wallet')
  .action(send);

program.command('supply').description('fetch total supply from explorer').action(() => {
  supplyCmd().catch((e) => console.error(e));
});

program.command('sync').description('follow blocks via WebSocket').action(sync);

program.parse(process.argv);
