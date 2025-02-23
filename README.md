# Zero-Knowledge Proofs Exploration in Rust

This project is an ongoing exploration of zero-knowledge proofs (ZKPs) using Rust. It is structured into multiple stages, each building upon the previous, to deepen understanding and implementation skills in ZKPs.

## 📚 Project Stages

### Stage 1: Schnorr Key Signatures

- **Objective**: Understand and implement Schnorr key signatures in Rust.
- **Status**: Completed
- **Description**: 
  - This stage involves implementing the Schnorr protocol, a digital signature scheme known for its simplicity and efficiency. The implementation focuses on key generation, signing, and verification processes.
  - **Key Concept**: Understanding the discrete logarithm problem is crucial. The security of Schnorr signatures relies on the difficulty of solving the discrete logarithm problem, which is the basis for many cryptographic protocols. By grasping this concept, you gain insight into how zero-knowledge proofs are constructed, as they often involve proving knowledge of a secret without revealing the secret itself.

### Stage 2: Simple Arithmetic Circuits with Halo2

- **Objective**: Implement simple arithmetic circuits using the Halo2 library from Zcash.
- **Status**: Upcoming
- **Description**: Halo2 is a library for constructing zero-knowledge proofs. This stage will involve creating basic arithmetic circuits to understand how they work at a low level, and how they can be used in conjunction with arithmetization and polynomial algebra to prove computations without revealing inputs.

### Stage 3: Simple Accounting System with Artworks-rs

- **Objective**: Implement a simple accounting system using zero-knowledge proofs with the artworks-rs library.
- **Status**: Upcoming
- **Description**: This stage will explore the use of ZKPs in financial systems, focusing on privacy-preserving transactions and balance proofs.

### Stage 4: Simple L2 Rollup

- **Objective**: Implement a simple Layer 2 rollup solution.
- **Status**: Upcoming
- **Description**: 
  - This stage involves aggregating a small batch of transactions from the accounting system developed in Stage 3. The goal is to use a zero-knowledge proof to verify that these transactions were processed correctly, without revealing the transaction details.
  - **Contrast with Stage 5**: While Stage 4 focuses on a specific use case of transaction aggregation and verification, Stage 5 will apply these concepts in a broader, real-world blockchain context, deploying the solution onto an Avalanche subnet.

### Stage 5: Capstone Project - Deploy L2 Solution on Avalanche Subnet

- **Objective**: Deploy a simple L2 solution onto a Layer 1 Avalanche subnet.
- **Status**: Upcoming
- **Description**: The final stage will involve deploying the developed L2 rollup solution onto an Avalanche subnet, demonstrating the practical application of ZKPs in a real-world blockchain environment.

## 🛠️ Development Setup

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/nick-stebbings/zkp
   cd zkp
   ```

2. **Install Rust**: Follow the [official Rust installation guide](https://www.rust-lang.org/tools/install).

3. **Build the Project**:
   ```bash
   cargo build
   ```

4. **Run Tests**:
   ```bash
   cargo test
   ```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 🔗 Links

- [ZCash Halo2 Book]([https://github.com/nick-stebbings/zkp](https://zcash.github.io/halo2/index.html))
- [Constructing zkSNARK circuits in Rust](https://jtriley.substack.com/p/constructing-zk-snark-circuits)
- [Why do zkSNARKS run on arithmetic circuits?](https://crypto.stackexchange.com/questions/112545/why-do-snarks-operate-on-arithmetic-circuits)
