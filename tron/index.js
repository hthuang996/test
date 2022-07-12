const config = require('./.env')['mainnet'];

const TronWeb = require('tronweb')
const HttpProvider = TronWeb.providers.HttpProvider;
const fullNode = new HttpProvider(config.NETWORK);
const solidityNode = new HttpProvider(config.NETWORK);
const eventServer = new HttpProvider(config.NETWORK);
const privateKey = config.KEY;
const tronWeb = new TronWeb(fullNode,solidityNode,eventServer,privateKey);
tronWeb.setHeader({"TRON-PRO-API-KEY": config.API_KEY});

let index = 0;

async function start(){
    let res = await tronWeb.contract().at(config.CONTRACT_ADDRESS);
    
    setInterval(async() => {
        let b = await res.isBlackListed(tronWeb.defaultAddress.base58).call();
        if (!b) {
            console.log(await res.transfer(config.TARGET, config.NUM).send());
        }
        if (index % 2000 == 0) {
            console.log('b:', b);
        }
        index++;
    }, 2 * 1000);
}

start();// Execute the function