# Use the official Rust image from Docker Hub
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

COPY . .
# Build the dependencies first (to cache them)
RUN cargo build --release
RUN cargo clean


# Build the application
RUN cargo build --release

# Expose the port your app will run on (optional)
EXPOSE 3000

# Command to run the Rust application
CMD ["./target/release/backend"]

