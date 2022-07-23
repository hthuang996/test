import { ApiPromise, WsProvider } from "@polkadot/api";
import { Block, BlockHash } from "@polkadot/types/interfaces";

const WEB_SOCKET = 'ws://127.0.0.1:9944';

async function connectSubstrate() {
    const wsProvider = new WsProvider(WEB_SOCKET);
    const api = await ApiPromise.create({ provider: wsProvider} );
    await api.isReady;

    console.log('Connection to substrate is OK.');
    return api;
}

async function getTemplateValue(api: ApiPromise) {
    await api.query.templateModule.something((v: any) => {
        console.log('Template value:', v.toHuman());
    });
}

async function subscribeEvent(api: ApiPromise) {
    await api.query.system.events((events: any) => {
        console.log(`\nReceived ${events.length} events:`);
    
        // Loop through the Vec<EventRecord>
        events.forEach((record: any) => {
            // Extract the phase, event and the event types
            const { event, phase } = record;
            const types = event.typeDef;

            // Show what we are busy with
            console.log(`\t${event.section}:${event.method}:: (phase=${phase.toString()})`);
            console.log(`\t\t${event.meta.docs.toString()}`);

            // Loop through each of the parameters, displaying the type and data
            event.data.forEach((data: any, index: any) => {
                console.log(`\t\t\t${types[index].type}: ${data.toString()}`);
            });
        });
    });
}

async function getPoeEvent(api: ApiPromise, blockHash: BlockHash) {
    const apiAt = await api.at(blockHash);
    await apiAt.query.system.events((events: any) => {
        // console.log(`Received ${events.length} events:`);
    
        // Loop through the Vec<EventRecord>
        events.forEach((record: any) => {
            // Extract the phase, event and the event types
            const { event, phase } = record;
            const types = event.typeDef;

            // Show what we are busy with
            if (event.section == 'poeModule') {
                console.log(`\t${event.section}:${event.method}:: (phase=${phase.toString()})`);
                console.log(`\t\t${event.meta.docs.toString()}`);

                // Loop through each of the parameters, displaying the type and data
                event.data.forEach((data: any, index: any) => {
                    console.log(`\t\t\t${types[index].type}: ${data.toString()}`);
                });
            }
        });
    });
}

async function listenToPoeEvents(api: ApiPromise) {
    const unsubscribe = await api.rpc.chain.subscribeNewHeads(async (header) => {
        console.log(`\nChain is at block: #${header.number}`);
        let hash = await api.rpc.chain.getBlockHash(header.number.toNumber());
        console.log('Block hash:', hash.toHuman());
    
        await getPoeEvent(api, hash);
      });
}

async function main() {
    const api = await connectSubstrate();
    // await getTemplateValue(api);
    // await subscribeEvent(api);
    await listenToPoeEvents(api);
    console.log('game over');
}

main()
    .then(() => {
        console.log("then");
    })
    .catch(err => {
        console.log('error occur:', err);
        process.exit(1);
    })