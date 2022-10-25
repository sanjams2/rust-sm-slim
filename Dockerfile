FROM alpine:3

RUN apk add --no-cache libgcc gcompat

COPY target/release/rust-sm-slim /rust-sm-slim

ENTRYPOINT ["/rust-sm-slim"]