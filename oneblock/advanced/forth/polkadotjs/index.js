const {ApiPromise, WsProvider, Keyring } = require('@polkadot/api');

async function queryLocalStorage(blockNumber) {
    // network
    const provider = new WsProvider("ws://127.0.0.1:9944");
    const api = await ApiPromise.create({provider});
    console.log(api.rpc.offchain);
    let bnU32Array = new Uint32Array(1);
    bnU32Array[0] = blockNumber;
    // let key = Buffer.concat([Buffer.from('node-template::storage::'), Buffer.from(bnU32Array.buffer)]);
    let key = Buffer.from(bnU32Array.buffer).toString('hex');
    console.log('key', key);
    let value = await api.rpc.offchain.localStorageGet('PERSISTENT', '0x' + key);
    console.log('value', value.toHuman());
}

queryLocalStorage(666)