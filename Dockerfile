FROM rust:latest as build
# Install nightly and depedencies.
RUN rustup default nightly-gnu
# Prepare depedency build.
WORKDIR /
RUN cargo new purpura_oazo
WORKDIR /purpura_oazo
COPY ./Cargo.toml /purpura_oazo/Cargo.toml
COPY ./Cargo.lock /purpura_oazo/Cargo.lock
RUN cargo build -r
# Build actual project.
COPY ./src ./
RUN cargo build -r

FROM archlinux:latest
# Install runtime depedencies
RUN pacman -Syu --noconfirm
COPY --from=build /purpura_oazo/target/release/purpura_oazo ./
CMD [ "./purpura_oazo" ]
