#!/bin/bash

# Set the build directory and run the release build
cargo build --release
if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1  # Exit with a non-zero code to indicate failure
fi

# Get the executable path
executable_path="target/release/nepdate-cli"

# Display the path of the executable
echo "nepdate-cli executable built at: $executable_path"
sleep 2

# Check if the executable exists and is runnable
if [ -f "$executable_path" ]; then
    # Run the executable
    echo "Running nepdate-cli..."
    $executable_path
else
    echo "Error: Executable not found at $executable_path"
fi
sleep 2