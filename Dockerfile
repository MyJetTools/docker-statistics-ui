FROM ubuntu:22.04

COPY ./target/release/docker-statistics-ui ./target/release/docker-statistics-ui
COPY ./dist ./dist
RUN chmod +x ./target/release/docker-statistics-ui

ENTRYPOINT ["./target/release/docker-statistics-ui"]