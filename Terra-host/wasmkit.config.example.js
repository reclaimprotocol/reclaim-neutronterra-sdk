const terra_mainnet_accounts = [
  {
    name: '',
    address: '',
    mnemonic: ''
  },
];
const networks = {
  terra_mainnet: {
    endpoint: 'https://phoenix-rpc.terra.dev/',
    chainId: 'phoenix-1',
    accounts: terra_mainnet_accounts,
    fees: {
      upload: {
        amount: [{ amount: "0", denom: "uluna" }],
        gas: "20000",
      },
      init: {
        amount: [{ amount: "0", denom: "uluna" }],
        gas: "500",
      },
      exec: {
        amount: [{ amount: "0", denom: "uluna" }],
        gas: "500",
      }
    },
  }
};

module.exports = {
  networks: {
    default: networks.terra_mainnet,
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
