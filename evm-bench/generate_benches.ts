import { utils } from 'ethers';
import { writeFileSync } from 'fs';

const benches = {
    empty_deploy: {
        contract: 'Empty',
        from: '0000000000000000000000000000000000000003',
        used_gas: 76975
    },
    empty_noop: {
        contract: 'Empty',
        from: '0000000000000000000000000000000000000003',
        call: ['noop'],
        used_gas: 21186
    },
    erc20_deploy: {
        contract: 'ERC20Token',
        from: '0000000000000000000000000000000000000003',
        used_gas: 1338700
    },
    erc20_approve: {
        contract: 'ERC20Token',
        from: '0000000000000000000000000000000000000003',
        call: ['approve', '0000000000000000000000000000000000000004', 10],
        used_gas: 46593,
        output: '0000000000000000000000000000000000000000000000000000000000000001'
    },
    erc20_approve_many: {
        contract: 'ERC20Token',
        from: '0000000000000000000000000000000000000003',
        call: ['approve_many', 100, 1],
        used_gas: 2511136,
        output: '0000000000000000000000000000000000000000000000000000000000000001'
    },
    erc20_transfer: {
        contract: 'ERC20Token',
        from: '0000000000000000000000000000000000000003',
        call: ['transfer', '4000000000000000000000000000000000000004', 20],
        used_gas: 52082,
        output: '0000000000000000000000000000000000000000000000000000000000000001'
    },
    erc20_transfer_many: {
        contract: 'ERC20Token',
        from: '0000000000000000000000000000000000000003',
        call: ['transfer_many', 100, 1],
        used_gas: 2856492,
        output: '0000000000000000000000000000000000000000000000000000000000000001'
    },
    storage_deploy: {
        contract: 'Storage',
        from: '0000000000000000000000000000000000000003',
        used_gas: 187075
    },
    storage_store: {
        contract: 'Storage',
        from: '0000000000000000000000000000000000000003',
        call: ['store', 1],
        used_gas: 43724
    },
    storage_store_many: {
        contract: 'Storage',
        from: '0000000000000000000000000000000000000003',
        call: ['store_many', 100, 1],
        used_gas: 2581277
    },
    ballot_deploy: {
        contract: 'Ballot',
        from: '0000000000000000000000000000000000000003',
        used_gas: 1060480
    },
    ballot_delegate: {
        contract: 'Ballot',
        from: '0000000000000000000000000000000000000003',
        call: ['delegate', '0000000000000000000000000000000000000004'],
        used_gas: 74340
    },
    ballot_vote: {
        contract: 'Ballot',
        from: '0000000000000000000000000000000000000003',
        call: ['vote', 0],
        used_gas: 75812
    }
}

for (const name in benches) {
    const bench = benches[name];
    const { bytecode, abi } = require(`${__dirname}/build/${bench.contract}.json`);
    bench['code'] = bytecode;
    delete bench['contract'];

    if (bench['call']) {
        const [method, ...args] = bench['call'];
        let contact = new utils.Interface(abi);
        const input = contact.encodeFunctionData(method, args).slice(2);
        bench['input'] = input;
        delete bench['call'];
    }
}

writeFileSync(`${__dirname}/build/benches.json`, JSON.stringify(benches, null, 2));    
