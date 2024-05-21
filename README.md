# Getter Setter Assignment

This project involves deploying and interacting with Solana smart contracts on a local test validator using the Solana CLI. It includes two programs: `getter_setter` for managing access control and value setting, and `interacting_program` for interacting with the `getter_setter` contract.

## Table of Contents

- [Installation](#installation)
- [Environment Setup](#environment-setup)
- [Code Explanation](#code-explanation)
  - [getter_setter Program](#getter_setter-program)
  - [interacting_program Program](#interacting_program-program)
- [Verification](#verification)

## Installation

1. **Install Rust and Cargo**: Follow the official instructions to install Rust and Cargo: [Rust Installation](https://www.rust-lang.org/learn/get-started)

2. **Install Solana CLI**: Follow the official instructions to install the Solana CLI: [Solana CLI Installation](https://docs.solana.com/cli/install-solana-cli-tools)

3. **Install Anchor CLI**:
    ```bash
    cargo install --git https://github.com/project-serum/anchor --locked anchor-cli
    ```

## Environment Setup

1. **Start Solana Test Validator**:
    ```bash
    solana-test-validator
    ```

2. **Build and Deploy Programs**:
    ```bash
    anchor build
    anchor deploy
    ```

3. **Set Solana CLI to Localnet**:
    ```bash
    solana config set --url localhost
    ```

## Code Explanation

### getter_setter Program

The `getter_setter` program manages access control and value setting. It includes the following instructions:
- **initialize**: Initializes the base account with an allowed program.
- **grantAccess**: Grants access to the interacting program.
- **revokeAccess**: Revokes access from the interacting program.
- **setValue**: Sets a value if access is granted.

### interacting_program Program

The `interacting_program` interacts with the `getter_setter` contract, specifically calling the `setValue` function if access is granted.

## Verification

Verify the state of your program and accounts using Solana CLI commands:

**Show Program Information**:

```bash
solana program show <PROGRAM_ID> --url localhost
