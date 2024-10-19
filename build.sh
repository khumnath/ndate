#!/bin/bash

# Set the build directory and run the release build
cargo build --release

# Get the executable path (replace 'your_project_name' with your actual project name)
executable_path="target/release/ndate"

# Display the path of the executable
echo "ndate executable built at: $executable_path"
sleep 2

# Check if the executable exists and is runnable
if [ -f "$executable_path" ]; then
    # Run the executable
    echo "Running ndate..."
    $executable_path
else
    echo "Error: Executable not found at $executable_path"
fi
