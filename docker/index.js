const config = require('config');

function main() {
    console.log('config name: ' + config.get('name'));
}

main()