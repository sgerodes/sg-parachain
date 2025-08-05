# Zombienet Quick Start

Zombienet is a testing framework for Polkadot networks. See the [official GitHub repository](https://github.com/paritytech/zombienet) and [documentation](https://paritytech.github.io/zombienet/) for more information.

## Prerequisites

1. Install zombienet: https://paritytech.github.io/zombienet/install.html#installation
2. Install Polkadot globally: `cargo install polkadot`
3. Build the runtime: `cargo build -r`

## Start Network

Start the parachain zombienet from the root directory of the substrate project
```bash
cd zombienet
zombienet spawn ./zombienet.toml
```

## Running Tests

### Spawn and Test in One Command 
```bash
cd zombienet
zombienet test ./test-minimal.zndsl --provider native
```

**Note:** The `--provider native` flag ensures the test runs with the native provider instead of kubernetes.

**Zombienet DSL Resources**: For detailed configuration options, see the [Zombienet DSL specification](https://paritytech.github.io/zombienet/cli/test-dsl-definition-spec.html).

**Test Examples**: See [zombienet test examples](https://github.com/paritytech/zombienet/tree/master/examples) for more test patterns.

## Fork a chain 

Chopsticks provides a way to [fork chains](https://docs.polkadot.com/tutorials/polkadot-sdk/testing/fork-live-chains/)
