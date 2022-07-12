const config = require('./.env')['testnet'];

console.log(config)

const TronWeb = require('tronweb')
const HttpProvider = TronWeb.providers.HttpProvider;
const fullNode = new HttpProvider(config.NETWORK);
const solidityNode = new HttpProvider(config.NETWORK);
const eventServer = new HttpProvider(config.NETWORK);
const privateKey = config.KEY;
const tronWeb = new TronWeb(fullNode,solidityNode,eventServer,privateKey);
tronWeb.setHeader({"TRON-PRO-API-KEY": config.API_KEY});

async function getContract(){
    let res = await tronWeb.contract().at(config.CONTRACT_ADDRESS);
    console.log(await res.transfer(config.TARGET, config.NUM).send());

}
getContract();// Execute the function