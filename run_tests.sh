#!/bin/bash

# Set the script to exit immediately if any command fails
set -e

echo "Running tests for all packages in the workspace..."
echo "=================================================="

# Run tests for all packages in the workspace
cargo test --workspace

echo "=================================================="
echo "All tests completed successfully!" 