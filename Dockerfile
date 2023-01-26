# guide from https://dev.to/rogertorres/first-steps-with-docker-rust-30oi

# use official rust image
FROM rust:latest

# copy the files from my machine to docker image
COPY ./ ./

# build program in debug mode
RUN cargo build

# test
# RUN cargo test

# documentation
RUN cargo doc --open
