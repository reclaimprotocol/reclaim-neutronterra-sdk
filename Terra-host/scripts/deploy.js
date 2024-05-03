import { LCDClient, MsgInstantiateContract } from '@terra-money/terra.js';
import { MnemonicKey, MsgStoreCode, isTxError } from '@terra-money/terra.js';
import * as fs from 'fs';
import 'dotenv/config';

const terra = new LCDClient({
  URL: "https://lcd-terra.tfl.foundation/",
  chainID: "phoenix-1",
});

const mk = new MnemonicKey({
    mnemonic: process.env.MNEMONIC,
});
const wallet = terra.wallet(mk);
const storeCode = new MsgStoreCode(
wallet.key.accAddress,
    fs.readFileSync('../artifacts/contracts/terra_host.wasm').toString('base64')
);
const storeCodeTx = await wallet.createAndSignTx({
    msgs: [storeCode],
});
const storeCodeTxResult = await terra.tx.broadcast(storeCodeTx);

console.log(storeCodeTxResult);

if (isTxError(storeCodeTxResult)) {
    throw new Error(
        `store code failed. code: ${storeCodeTxResult.code}, codespace: ${storeCodeTxResult.codespace}, raw_log: ${storeCodeTxResult.raw_log}`
    );
}

const {
    store_code: { code_id },
  } = storeCodeTxResult.logs[0].eventsByType;


console.log(wallet.key.accAddress)
const instantiate = new MsgInstantiateContract(
    wallet.key.accAddress,
    undefined,
    code_id,
    {}, // InitMsg
    { uluna: 10000 },
    "free"
  );
  
  const instantiateTx = await wallet.createAndSignTx({
    msgs: [instantiate],
  });
  const instantiateTxResult = await terra.tx.broadcast(instantiateTx);
  
  console.log(instantiateTxResult);
  
  if (isTxError(instantiateTxResult)) {
    throw new Error(
      `instantiate failed. code: ${instantiateTxResult.code}, codespace: ${instantiateTxResult.codespace}, raw_log: ${instantiateTxResult.raw_log}`
    );
  }