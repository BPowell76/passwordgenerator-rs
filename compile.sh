#!/bin/bash

# Linux Build (x86_64-unknown-linux-gnu)
cargo build -r

# Windows Build (GNU)
cargo build -r --target x86_64-pc-windows-gnu

# Generate checksums
cat /dev/null > ./target/passwordgenerator-rs.sha256
cd ./target/release
sha256sum passwordgenerator-rs >> ../passwordgenerator-rs.sha256
cd ../x86_64-pc-windows-gnu/release
sha256sum passwordgenerator-rs.exe >> ../../passwordgenerator-rs.sha256

cd ../..
