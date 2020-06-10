FROM rust:nightly-slim as build

COPY ./ ./

ENV ROCKET_PORT 8000

RUN cargo build --release

RUN mkdir -p /build-out

RUN cp target/release/totto /build-out/

# Ubuntu 18.04
FROM ubuntu@sha256:5f4bdc3467537cbbe563e80db2c3ec95d548a9145d64453b06939c4592d67b6d

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /build-out/http-rest /

EXPOSE 8000

CMD /http-rest
