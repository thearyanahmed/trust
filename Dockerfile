# Use a small Ubuntu base image
FROM ubuntu:20.04

# Install necessary tools for networking and Rust development
RUN apt-get update && \
    apt-get install -y iputils-ping net-tools iproute2 tcpdump curl build-essential && \
    rm -rf /var/lib/apt/lists/*

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set up the environment variables for Rust
ENV PATH="/root/.cargo/bin:${PATH}"

# Create a work directory and set it as the working directory
WORKDIR /app

# Copy the setup script
COPY loop /app/

# Mount the current directory as a volume
VOLUME /app

RUN cargo install cargo-watch

# Set up the entry point to run the setup script
CMD ["./loop"]
