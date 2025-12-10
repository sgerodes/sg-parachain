# Bridge Pallet POC

This is a minimal Proof of Concept (POC) demonstrating cross-chain communication between two parachains using the bridge pallet.

## Overview

The bridge pallet provides basic functionality for:
- Sending messages between parachains
- Executing cross-chain function calls
- Tracking message status and delivery

## Architecture

```
┌─────────────────┐    ┌─────────────────┐
│   Parachain     │    │   Parachain     │
│     2000        │    │     2001        │
│                 │    │                 │
│ ┌─────────────┐ │    │ ┌─────────────┐ │
│ │Bridge Pallet│ │    │ │Bridge Pallet│ │
│ └─────────────┘ │    │ └─────────────┘ │
└─────────────────┘    └─────────────────┘
         │                       │
         └───────────────────────┘
                   │
         ┌─────────────────┐
         │   Relay Chain   │
         │   (Rococo)      │
         └─────────────────┘
```

## Prerequisites

1. **Polkadot binary**: Download `polkadot` binary for your platform
2. **Node.js**: For running the test script
3. **Dependencies**: Install required npm packages

## Setup

### 1. Install Dependencies

```bash
npm install @polkadot/api @polkadot/keyring
```

### 2. Build the Project

```bash
cargo build --release
```

### 3. Download Polkadot

Download the latest `polkadot` binary for your platform and place it in the project root.

## Running the POC

### 1. Start the Network

```bash
zombienet spawn zombienet/poc_bridge.toml
```

This will start:
- 4 relay chain validators (Alice, Bob, Charlie, Dave)
- 2 parachain collators (Parachain 2000 and 2001)
- HRMP channels between the parachains

### 2. Run the Test Script

```bash
node test_bridge_poc.js
```

The test script will:
- Connect to both parachains
- Send messages between them using the bridge pallet
- Execute cross-chain function calls
- Demonstrate the basic functionality

## What the POC Demonstrates

1. **Message Sending**: Basic message passing between parachains
2. **Cross-chain Calls**: Execution of function calls across chains
3. **Message Tracking**: Status tracking for sent/received messages
4. **Event System**: Events emitted for cross-chain activities

## Current Limitations

- **No XCM Integration**: This is a simplified implementation without actual XCM message passing
- **Local Testing**: Messages are stored locally but not actually transmitted between chains
- **Basic Functionality**: Minimal feature set for demonstration purposes

## Next Steps for Production

1. **XCM Integration**: Implement actual XCM message sending
2. **Message Validation**: Add proper message validation and security
3. **Error Handling**: Enhance error handling and recovery mechanisms
4. **Performance**: Optimize for production workloads
5. **Testing**: Add comprehensive test coverage

## Troubleshooting

### Common Issues

1. **Port Conflicts**: Ensure ports 9944, 9945, 8545, 8546 are available
2. **Binary Paths**: Verify `polkadot` and `parachain-template-node` are in the correct locations
3. **Build Errors**: Ensure all dependencies are properly installed

### Logs

Check the zombienet output for detailed logs from both parachains and the relay chain.

## Files

- `pallets/bridge/src/lib.rs` - Bridge pallet implementation
- `zombienet/poc_bridge.toml` - Zombienet configuration
- `test_bridge_poc.js` - Test script
- `runtime/src/configs/mod.rs` - Runtime configuration

## Support

This is a minimal POC for demonstration purposes. For production use, additional development and testing is required.
