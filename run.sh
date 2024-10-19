#!/bin/bash

set -e # Exit immediately if a command exits with a non-zero status.

# Compile the C runtime
echo "Compiling C runtime..."
gcc -c src/runtime/runtime.c -o runtime.o

# Run the Rust program and capture its output (assembly) to a file
echo "Running Rust program to generate assembly..."
cargo run >output.s

# Check if the assembly file is not empty
if [ ! -s output.s ]; then
  echo "Error: The generated assembly file is empty."
  exit 1
fi

# Assemble the generated assembly
echo "Assembling..."
as output.s -o output.o

# Link everything together
echo "Linking..."
ld -syslibroot $(xcrun -sdk macosx --show-sdk-path) \
   -lSystem \
   runtime.o output.o \
   -o final_executable


echo "Compilation, assembly, and linking completed successfully!"
