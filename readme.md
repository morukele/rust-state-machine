# Rust State Machine Implementation

This repository contains an implementation of a basic blockchain state machine in Rust, based on the tutorial from [Shawn Tabrizi's Rust State Machine Guide](https://www.shawntabrizi.com/rust-state-machine/0/index.html).

## Overview

This project demonstrates core blockchain concepts by implementing a simple state machine with the following features:

- **Balance Management**: Track and manage user account balances
- **Proof of Existence**: Create and revoke claims for content
- **Block Processing**: Execute transactions within blocks
- **System State**: Maintain system-wide state including block numbers and nonces

## Project Structure

The project is organized into several modules:

- `balances`: Handles account balance operations
- `proof_of_existence`: Manages content claims and revocations
- `support`: Contains core types and traits for the state machine
- `system`: Manages system-wide state
- `types`: Defines core type aliases used throughout the system

## Key Features

### Balance System
- Transfer tokens between accounts
- Check and maintain account balances
- Prevent invalid transfers (insufficient funds)

### Proof of Existence
- Create claims for content
- Revoke existing claims
- Prevent duplicate claims

### Runtime System
- Block-by-block execution
- Transaction (extrinsic) processing
- Account nonce tracking
- Block number management

## Usage

The main example in `main.rs` demonstrates:
1. Creating a new runtime instance
2. Setting up user accounts (Alice, Bob, Charlie)
3. Executing multiple blocks containing various transactions
4. Processing balance transfers
5. Managing proof of existence claims

### Example

```rust
let mut runtime = Runtime::new();

// Initialize accounts
let alice = String::from("alice");
let bob = String::from("bob");

// Set initial balance
runtime.balances.set_balance(&alice, 100);

// Create and execute a block
let block = types::Block {
    header: support::Header { block_number: 1 },
    extrinsics: vec![
        support::Extrinsic {
            caller: alice.clone(),
            call: RuntimeCall::Balances(balances::Call::Transfer {
                to: bob.clone(),
                amount: 30,
            }),
        },
    ],
};

runtime.execute_block(block).expect("invalid block");
```

## Dependencies

- `num`: Version 0.4.1 - Provides numeric type utilities

## Building and Running

To build and run the project:

```bash
cargo build
cargo run
```

## License

This project is open source and available for learning and educational purposes.

## Acknowledgments

This implementation is based on the educational materials provided by Shawn Tabrizi in his Rust State Machine tutorial series.
