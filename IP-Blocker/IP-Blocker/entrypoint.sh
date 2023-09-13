#!/bin/sh

# Debugging: List contents of the release directory
cd xdp-drop
pwd
ls -R .

# Start xdp-drop
./target/release/xdp-drop
cd ..
# Start the server
./Blocker-API/server/server &

# Start the client
./Blocker-API/client/client &

# Start the main application
./Blocker-API/main
