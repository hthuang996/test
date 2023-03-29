const http = require('http');
http.createServer(function (req, res) {
    res.write('Hello World!'); //write a response to the client
    res.end(); //end the response
  }).listen(8080); //the server object listens on port 8080

//   setInterval(() => { console.log('still running'); throw 'asdf'}, 500);

async function sleep(seconds) {
    await new Promise((resolve) => {
        setTimeout(() => {
        resolve();
        }, seconds * 1000);
    });
}

async function tr() {
    throw 'asdf'
}

function tr2() {
    throw 'asdf'
}

async function main() {
    // throw 'async in setTimeout'
    while (true) {
        await tr();
        await sleep(1);
        console.log('while1');
    }
    console.log('while11');
}

async function main2() {
    console.log('while23');
    new Error('asdf');
    return;
    while (true) {
        await sleep(1);
        console.log('while2');
    }
    console.log('while22');
}
try {
main2()
}
catch {
    console.log('catch')
}
console.log('over')

process.on('unhandledRejection', (err) => {
    console.log('unhandledRejection');
	throw err;
})

process.on('uncaughtException', (err) => {
    console.log('uncaughtException');
    process.exitCode = 1;
    process.exit();
	// throw err;
})