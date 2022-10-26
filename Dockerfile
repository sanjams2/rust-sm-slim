FROM alpine:3

LABEL com.amazonaws.sagemaker.capabilities.accept-bind-to-port=true

RUN apk add --no-cache libgcc gcompat
RUN mkdir -p /sagemaker/bin
COPY target/release/rust-sm-slim /sagemaker/bin/rust-sm-slim

ENTRYPOINT ["/sagemaker/bin/rust-sm-slim"]