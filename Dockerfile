FROM rust:1.67.0-slim-buster AS builder
WORKDIR /usr/src/quote-service
COPY . .
RUN cargo fetch
RUN cargo install --path quote_server --bins --root /usr/local/
RUN cargo install --path quote_client --bins --root /usr/local/


FROM debian:bullseye-slim
WORKDIR /quote-service

COPY --from=builder /usr/src/quote-service/log4rs.yml \
                    /usr/src/quote-service/Words-of-Wisdom.txt \
                    /usr/src/quote-service/config/config.yml  ./
COPY --from=builder /usr/local/bin/quote_server /usr/bin/
COPY --from=builder /usr/local/bin/quote_client /usr/bin/
EXPOSE 8081
