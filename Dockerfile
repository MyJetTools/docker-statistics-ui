FROM ubuntu:22.04

COPY ./target/release/docker-statistics-ui ./target/release/docker-statistics-ui
COPY ./files ./files
ENTRYPOINT ["./target/release/docker-statistics-ui"]