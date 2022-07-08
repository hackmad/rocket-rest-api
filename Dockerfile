# ----------------------------------------------------------------------------
# The build container.
FROM rust:1.62 as build

# Create a new empty shell project.
RUN cd / && cargo new --bin rocket-rest-api
WORKDIR /rocket-rest-api

# Cache dependencies.
COPY ./Cargo.toml ./Cargo.toml
COPY ./diesel.toml ./diesel.toml
RUN cargo install --path . --locked
RUN rm src/*.rs
RUN rm ./target/release/deps/rocket_rest_api*

# Build the source.
ADD . ./
RUN cargo install --path . --locked

# ----------------------------------------------------------------------------
# The release container.
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq-dev
COPY --from=build /rocket-rest-api/target/release/rocket-rest-api /
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
EXPOSE 8000
CMD ["/rest-api"]
