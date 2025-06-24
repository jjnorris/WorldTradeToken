# wtt-wallet

Simple TypeScript CLI wallet used for development and testing. Keys are stored in `wallet.json` in the current directory. Commands include key generation, balance lookup and sending tokens. This is only a placeholder implementation.

```bash
npm install
npm run build
node dist/index.js generate
node dist/index.js balance            # uses wallet.json
node dist/index.js balance <address>
node dist/index.js send <to> <amount>
node dist/index.js supply            # query explorer
node dist/index.js sync              # listen for new blocks
```
