# EIP-7702 Playground

## Overview

This repository provides a playground for experimenting with EIP-7702, an Ethereum Improvement Proposal that allows Externally Owned Accounts (EOAs) to utilize smart contract code for a single transaction. This functionality is scheduled for inclusion in the upcoming **Pectra** upgrade in April 2025.

## Features

- **EIP-7702 Transactions**: Explore how EOAs can delegate their code to smart contracts.
- **Storage Management**: Understand the implications of storage when delegating EOAs to contracts.
- **Alloy Integration**: Use Alloy (Rust) scripts to interact with the Ithaca test network, a staging ground for EIP-7702.

## Getting Started

### Prerequisites

- Ensure you have Rust and Cargo installed on your machine.
- Clone this repository to your local environment.

### Environment Setup

1. Create a `.env` file in the root of the project directory.
2. Add your private key and RPC URL to the `.env` file:

   ```
   PRIVATE_KEY=your_private_key_here
   RPC_URL
