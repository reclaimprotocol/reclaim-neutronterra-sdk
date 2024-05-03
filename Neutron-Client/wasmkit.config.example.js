const neutron_testnet_accounts = [
  {
    name: 'account_0',
    address: '', // your wallet address
    mnemonic: '' // your wallet mnemonic key
  },
];

const neutron_mainnet_accounts = [
  {
    name: 'account_0',
    address: '', // your wallet address
    mnemonic: '' // your wallet mnemonic key
  },
];


const networks = {
  neutron_testnet: {
    endpoint: 'https://rpc-palvus.pion-1.ntrn.tech/',
    chainId: 'pion-1',
    accounts: neutron_testnet_accounts,
    fees: {
      upload: {
        amount: [{ amount: "90000", denom: "untrn" }],
        gas: "4000000",
      },
      init: {
        amount: [{ amount: "20000", denom: "untrn" }],
        gas: "1000000",
      },
      exec: {
        amount: [{ amount: "20000", denom: "untrn" }],
        gas: "1000000",
      }
    },
  },
  neutron_mainnet: {
    endpoint: 'https://rpc-kralum.neutron-1.neutron.org',
    chainId: 'neutron-1',
    accounts: neutron_mainnet_accounts,
    fees: {
      upload: {
        amount: [{ amount: "90000", denom: "untrn" }],
        gas: "4000000",
      },
      init: {
        amount: [{ amount: "20000", denom: "untrn" }],
        gas: "1000000",
      },
      exec: {
        amount: [{ amount: "20000", denom: "untrn" }],
        gas: "1000000",
      }
    },
  },
};

module.exports = {
  networks: {
    default: networks.neutron_mainnet,
    testnet: networks.neutron_testnet,
    mainnet: networks.neutron_mainnet,
  },
  mocha: {
    timeout: 60000
  },
  rust: {
    version: "1.68.0",
  },
  commands: {
    compile: "RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown",
    schema: "cargo run --example schema",
  },
};
