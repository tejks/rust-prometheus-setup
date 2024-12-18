# Stage 1: Build Stage
FROM rust:latest as build

# Create a new shell project
RUN USER=root cargo new --bin prome-test
WORKDIR /prome-test

# Copy over Cargo.toml and Cargo.lock to ensure they are up-to-date
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs  # Remove initial Rust source files, keeping build clean

# Copy the rest of the source
COPY ./src ./src

# Build the release version of the app
RUN cargo clean --release  # Ensure clean build directory
RUN cargo build --release

# Stage 2: Final Stage (minimal base image)
FROM rust:slim

# Copy the built binary from the previous stage
COPY --from=build /prome-test/target/release/prome-test /usr/local/bin/prome-test

# Expose the port on which your app runs
EXPOSE 9898

# Set the entry point
CMD ["prome-test"]