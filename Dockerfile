FROM ubuntu:22.04

COPY ./target/release/docker-statistics-ui ./target/release/docker-statistics-ui
COPY ./dist ./dist
ENTRYPOINT ["./target/release/docker-statistics-ui"]