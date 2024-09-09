# Use an official Rust image as a base
FROM rust:latest

# Install neovim, tmux, and other necessary dependencies
RUN apt-get update && apt-get install -y \
    neovim \
    tmux \
    curl \
    git

# Set the working directory inside the container
WORKDIR /usr/src/txr

# Copy the current directory into the container
COPY . .

# Build the Rust project
#RUN cargo build --release

# Command to keep the container running for tmux session
CMD ["tail", "-f", "/dev/null"]

