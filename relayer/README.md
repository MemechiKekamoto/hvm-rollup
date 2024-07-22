# HVM offchain relayer with QuantumFusion node

## Overview

The HVM offchain Relayer acts as a bridge between a QuantumFusion node and an offchain sequencer. This relayer receives HVM WASM binary encoded via HTTP, converts it into calldata, and sends it as a parameter to a pallet function in our node.

## Prerequisites

Before setting up the project, ensure you have the following installed:

- [Rust](https://doc.rust-lang.org/)
- [Substrate](https://docs.substrate.io/)

## Getting Started

1. Clone the repository:
```bash
git clone https://github.com/MemechiKekamoto/hvm-rollup.git
cd hvm-rollup/relayer
```
2. Build the project:
```bash
cargo build
```
3. Ensure our node is running:
4. Run the relayer:
```bash
cargo run -- --substrate-url ws://127.0.0.1:9944
```
5. Send data via http request:
```bash
curl -X POST -H "Content-Type: application/json" -d '{"data":<"testdata">}' http://127.0.0.1:3030/receive
```

## Contributing

We welcome contributions from the community. If you wish to contribute, please fork the repository and submit a pull request.

## Join Us

Join us in pioneering the future of parallel computing, driving innovation, and transforming technology across industries. For more information, visit our [website](https://quantumfusion.pro/) and follow us on [GitHub](https://github.com/MemechiKekamoto/hvm-rollup).