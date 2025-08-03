# Zombienet Quick Start

Zombienet is a testing framework for Polkadot networks. See the [official GitHub repository](https://github.com/paritytech/zombienet) and [documentation](https://paritytech.github.io/zombienet/) for more information.

## Prerequisites

1. Install zombienet: https://paritytech.github.io/zombienet/install.html#installation
2. Install Polkadot globally: `cargo install polkadot`
3. Build the runtime: `cargo build -r`

## Start Network

Start the parachain zombienet from the root directory of the substrate project
```bash
zombienet spawn ./zombienet/zombienet.toml
```

## Running Tests

### Option 1: Spawn and Test Separately
```bash
# 1. Spawn the network first
zombienet spawn ./zombienet/zombienet.toml

# 2. Then run tests against the running network
zombienet test ./zombienet/test-parachain.zndsl
```

### Option 2: Spawn and Test in One Command (Recommended)
```bash
# From the zombienet directory
cd zombienet
zombienet test ./test-local.zndsl --provider native
```

**Note:** The `--provider native` flag ensures the test runs with the native provider instead of kubernetes.

**Zombienet DSL Resources**: For detailed configuration options, see the [Zombienet DSL specification](https://paritytech.github.io/zombienet/cli/test-dsl-definition-spec.html).

**Test Examples**: See [zombienet test examples](https://github.com/paritytech/zombienet/tree/master/examples) for more test patterns.
