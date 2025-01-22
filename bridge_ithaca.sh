#!/usr/bin/env bash

# Exit immediately if any command returns a non-zero status.
set -e

# Source the .env file to load environment variables
source .env

# Check that the PRIVATE_KEY and SEPOLIA_RPC_URL are set
if [ -z "$PRIVATE_KEY" ]; then
  echo "Error: PRIVATE_KEY is not set in .env"
  exit 1
fi

# Run the cast send command
cast send 0x9228665c0D8f9Fc36843572bE50B716B81e042BA \
  --value 0.1ether \
  --private-key "$PRIVATE_KEY" \
  --rpc-url sepolia

