#!/bin/bash

# Start or restart the container
docker-compose up -d

# Execute the `n` binary within the container and pass any arguments
docker-compose exec txr cargo run -- "$@"

# Attach to tmux inside the container
docker-compose exec txr tmux attach

