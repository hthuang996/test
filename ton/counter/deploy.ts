import { SendMode, InternalMessage, toNano, CommonMessageInfo, CellMessage, StateInit } from "ton";
import { mnemonicToWalletKey } from "ton-crypto";
import { TonClient, WalletContract, WalletV3R2Source } from "ton";
import { beginCell } from "ton";
import fs from "fs";
import { contractAddress, Cell } from "ton";

import { TupleSlice } from "ton";

let key:any = null;
let client:any = null;
let wallet:any = null;
let initDataCell:any = null;
let initCodeCell:any = null;
let newContractAddress:any = null;

async function main() {
    const mnemonic = "student spirit beyond pistol autumn album box ritual onion virus lady network sugar curtain seat true famous ball balcony forget solve weasel nephew brick"; // your 24 secret words
    key = await mnemonicToWalletKey(mnemonic.split(" "));

    client = new TonClient({ endpoint: "https://testnet.toncenter.com/api/v2/jsonRPC" });
    wallet = WalletContract.create(client, WalletV3R2Source.create({ publicKey: key.publicKey, workchain: 0 }));

    initDataCell = initData(); // the function we've implemented just now
    initCodeCell = Cell.fromBoc(fs.readFileSync("counter.cell"))[0]; // compilation output from step 5
    
    newContractAddress = contractAddress({ workchain: 0, initialData: initDataCell, initialCode: initCodeCell });
    console.log("newContractAddress", newContractAddress);
}

async function sendMessage() {
    await main();
    const messageBody = beginCell().storeUint(1, 32).storeUint(0, 64).endCell(); // op with value 1 (increment)
    
    const seqno = await wallet.getSeqNo(); // get the next seqno of sender wallet

    const m = {
        to: newContractAddress, // newContractAddress from deploy
        value: toNano(0.02), // pay 0.02 TON as gas
        bounce: false,
        body: new CommonMessageInfo({
          body: new CellMessage(messageBody),
        }),
      };
    
      console.log('m', m);
    const transfer = wallet.createTransfer({
      secretKey: key.secretKey, // from the secret mnemonic of the sender wallet
      seqno: seqno,
      sendMode: SendMode.PAY_GAS_SEPARATLY + SendMode.IGNORE_ERRORS,
      order: new InternalMessage(m),
    });
    
    await client.sendExternalMessage(wallet, transfer);
  }

async function callGetter() {
    await main()
  const call = await client.callGetMethod(newContractAddress, "counter"); // newContractAddress from deploy
  const counterValue = new TupleSlice(call.stack).readBigNumber();
  console.log(`counter value is ${counterValue.toString()}`);
}

function initData() {
  const initialCounterValue = 17;
  return beginCell().storeUint(initialCounterValue, 64).endCell();
}

async function deploy() {
  const seqno = await wallet.getSeqNo(); // get the next seqno of our wallet
  
  const transfer = wallet.createTransfer({
    secretKey: key.secretKey, // from the secret mnemonic of the deployer wallet
    seqno: seqno,
    sendMode: SendMode.PAY_GAS_SEPARATLY + SendMode.IGNORE_ERRORS,
    order: new InternalMessage({
      to: newContractAddress, // calculated before
      value: toNano(0.02), // fund the new contract with 0.02 TON to pay rent
      bounce: false,
      body: new CommonMessageInfo({
        stateInit: new StateInit({ data: initDataCell, code: initCodeCell }), // calculated before
        body: null,
      }),
    }),
  });
  
  await client.sendExternalMessage(wallet, transfer);
}

// deploy()
// callGetter()
sendMessage()