# Use a small Ubuntu base image
FROM ubuntu:20.04

ENV TZ=America/New_York

# Refresh the timezone
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Install necessary tools for networking and Rust development
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt install -y iputils-ping net-tools iproute2 tshark netcat tcpdump curl build-essential && \
    rm -rf /var/lib/apt/lists/*

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set up the environment variables for Rust
ENV PATH="/root/.cargo/bin:${PATH}"

# Create a work directory and set it as the working directory
WORKDIR /app

# Mount the current directory as a volume
VOLUME /app

CMD ["tail", "-f", "/dev/null"]
