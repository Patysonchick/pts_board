FROM rust:1.91.1-alpine3.22 AS builder

RUN apk add --no-cache build-base

WORKDIR /pts_board

COPY Cargo.toml Cargo.lock ./
COPY migration/Cargo.toml ./migration/Cargo.toml
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN mkdir -p migration/src && echo "" > migration/src/lib.rs
RUN cargo build --release

COPY src ./src
COPY migration ./migration
COPY templates ./templates
COPY static ./static
RUN touch src/main.rs migration/src/lib.rs
RUN cargo build --release --locked

FROM alpine:3.22.2 AS runner

WORKDIR /pts_board

RUN addgroup -S pts_board && adduser -S -G pts_board pts_board
USER pts_board

COPY --from=builder --chown=pts_board:pts_board /pts_board/static ./static
COPY --from=builder /pts_board/target/release/pts_board ./

CMD ["./pts_board"]
