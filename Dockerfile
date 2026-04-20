FROM ghcr.io/myjettools/dioxus-docker:0.7.5

ENV PORT=9001
ENV IP=0.0.0.0

COPY ./target/dx/my-logger-ui/release/web /target/dx/my-logger-ui/release/web

RUN chmod +x /target/dx/my-logger-ui/release/web/server
WORKDIR /target/dx/my-logger-ui/release/web/
ENTRYPOINT ["./server"]
