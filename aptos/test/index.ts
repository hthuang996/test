/** AptosAccount provides methods around addresses, key-pairs */
import { AptosAccount, TxnBuilderTypes, BCS, MaybeHexString } from "aptos";

/** Wrappers around the Aptos Node and Faucet API */
import { AptosClient, FaucetClient } from "aptos";

// const NODE_URL = 'https://fullnode.devnet.aptoslabs.com/v1';
export const NODE_URL = "https://fullnode.devnet.aptoslabs.com";
export const FAUCET_URL = "https://faucet.devnet.aptoslabs.com";

const client = new AptosClient(NODE_URL);

export async function accountBalance(accountAddress: MaybeHexString): Promise<number | null> {
    const resource = await client.getAccountResource(accountAddress, "0x1::coin::CoinStore<0x1::aptos_coin::AptosCoin>");
    if (resource == null) {
      return null;
    }
  
    return parseInt((resource.data as any)["coin"]["value"]);
  }

  async function transfer(accountFrom: AptosAccount, recipient: MaybeHexString, amount: number): Promise<string> {
    const token = new TxnBuilderTypes.TypeTagStruct(TxnBuilderTypes.StructTag.fromString("0x1::aptos_coin::AptosCoin"));
  
    const entryFunctionPayload = new TxnBuilderTypes.TransactionPayloadEntryFunction(
      TxnBuilderTypes.EntryFunction.natural(
        "0x1::coin",
        "transfer",
        [token],
        [BCS.bcsToBytes(TxnBuilderTypes.AccountAddress.fromHex(recipient)), BCS.bcsSerializeUint64(amount)],
      ),
    );
  
    const [{ sequence_number: sequenceNumber }, chainId] = await Promise.all([
      client.getAccount(accountFrom.address()),
      client.getChainId(),
    ]);
  
    const rawTxn = new TxnBuilderTypes.RawTransaction(
      TxnBuilderTypes.AccountAddress.fromHex(accountFrom.address()),
      BigInt(sequenceNumber),
      entryFunctionPayload,
      1000n,
      1n,
      BigInt(Math.floor(Date.now() / 1000) + 10),
      new TxnBuilderTypes.ChainId(chainId),
    );
  
    const bcsTxn = AptosClient.generateBCSTransaction(accountFrom, rawTxn);
    const pendingTxn = await client.submitSignedBCSTransaction(bcsTxn);
  
    return pendingTxn.hash;
  }

/**
 * https://aptos-labs.github.io/ts-sdk-doc/classes/AptosClient.html#getAccount
 * returns the sequence number and authentication key for an account
 *
 * https://aptos-labs.github.io/ts-sdk-doc/classes/AptosClient.html#getAccountResource
 * returns all resources associated with the account
 */

 const faucetClient = new FaucetClient(NODE_URL, FAUCET_URL);

 async function main() {
    // Create two accounts, Alice and Bob, and fund Alice but not Bob
    const alice = new AptosAccount();
    const bob = new AptosAccount();
  
    console.log("\n=== Addresses ===");
    console.log(`Alice: ${alice.address()}`);
    console.log(`Bob: ${bob.address()}`);
  
    await faucetClient.fundAccount(alice.address(), 5_000);
    await faucetClient.fundAccount(bob.address(), 0);
  
    console.log("\n=== Initial Balances ===");
    console.log(`Alice: ${await accountBalance(alice.address())}`);
    console.log(`Bob: ${await accountBalance(bob.address())}`);
  
    // Have Alice give Bob 1000 coins
    const txHash = await transfer(alice, bob.address(), 1_000);
    await client.waitForTransaction(txHash);
  
    console.log("\n=== Final Balances ===");
    console.log(`Alice: ${await accountBalance(alice.address())}`);
    console.log(`Bob: ${await accountBalance(bob.address())}`);
  }
  
  main();