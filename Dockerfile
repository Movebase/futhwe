# Use a Rust base image
FROM rust:latest AS builder

# Set the working directory
WORKDIR /app

# Install additional dependencies
RUN apt-get update && \
    apt-get install -y \
    build-essential \
    protobuf-compiler \
    pkg-config \
    clang\
    cmake \
    && apt-get clean 

# Copy the project files into the container
COPY . .

# Build the project
RUN cargo build --release --bin futhwe

# Use a ubuntu base image
FROM ubuntu:latest

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/futhwe .


# Expose the port
EXPOSE $APP_PORT

# Run the binary
CMD ["./futhwe"]
