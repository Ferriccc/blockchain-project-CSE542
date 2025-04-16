# Blockchain-Based Decentralized Storage System

## Project Overview
A Rust-based blockchain implementation providing decentralized storage with proof-of-storage validation and monetary incentives.

## Core Components

### 🔷 Node System (`src/node.rs`)
- **Implementation**: Ed25519 cryptography-based network participants
- **Features**:
  - Unique identifier
  - Public/private keypair for signing
  - Message validation and signature verification
  - Peer discovery support

### 🔶 Blockchain Structure

#### Block (`src/block.rs`)
- **Core Components**:
  - Previous block hash (SHA-256)
  - UTC timestamp
  - Dual transaction support:
    - Storage Transaction (STX)
    - Monetary Transaction (MTX)
  - Content-derived block hash
- **Capabilities**:
  - Serialization/deserialization
  - Validation logic

#### Blockchain (`src/blockchain.rs`)
- **Management**:
  - Block chain (`Vec<Block>`)
  - File storage mapping (`HashMap<String, Vec<String>>`)
  - Node balances (`HashMap<String, f64>`)
- **Core Functions**:
  ```rust
  - new_with_genesis_block()
  - add_block()
  - search_transaction()
