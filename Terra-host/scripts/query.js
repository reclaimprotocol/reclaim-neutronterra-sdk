import { LCDClient, MnemonicKey } from '@terra-money/terra.js';
import 'dotenv/config';

const terra = new LCDClient({
  URL: "https://phoenix-lcd.terra.dev",
  chainID: "phoenix-1", 
});

const mk = new MnemonicKey({
    mnemonic: process.env.MNEMONIC,
});

const wallet = terra.wallet(mk);

console.log(wallet.key.accAddress)

const identifier = "0x531322a6c34e5a71296a5ee07af13f0c27b5b1e50616f816374aff6064daaf55"

const queryResult = await terra.wasm.contractQuery(
  "terra13z58nvwxf5hjx0l7pwd72tt4dd2ycvj3zp3n5m5vtr3wnfw2jdzss35maw",
  { query_result: { identifier: identifier} }
);

console.log(queryResult); 