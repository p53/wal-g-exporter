ARG HOMEDIR=/opt/app
ARG WALG_TARGET=pg
ARG USER=101
ARG GROUP=103
# Build the manager binary
FROM rust:1.70.0-alpine3.18 AS builder

ARG WALG_TARGET
ARG WALG_VERSION
ARG APP_NAME
ARG APP_VERSION
ARG LOG_PKG_PATH
ARG APP_PKG_PATH
ARG WALG_DOWNLOAD_URL=https://github.com/wal-g/wal-g/releases/download
ARG HOMEDIR

WORKDIR /workspace

COPY src/ src/
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY log4rs.yml log4rs.yml

RUN ls -al /workspace/src

# Build
RUN apk add --no-cache musl-dev && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --locked --target=x86_64-unknown-linux-musl

# Download wal-g binary
RUN wget -qO- ${WALG_DOWNLOAD_URL}/${WALG_VERSION}/wal-g-${WALG_TARGET}-ubuntu-20.04-amd64.tar.gz | tar xvz && \
        mv wal-g-${WALG_TARGET}-ubuntu-20.04-amd64 target/x86_64-unknown-linux-musl/release/wal-g-postgres

RUN mkdir -p ${HOMEDIR} && \
    echo "app:x:${USER}:app" >> /etc/group && \
    echo "app:x:${USER}:${GROUP}:app user:${HOMEDIR}:/sbin/nologin" >> /etc/passwd && \
    chown app:app target/x86_64-unknown-linux-musl/release/wal-g-exporter && \
    chown app:app target/x86_64-unknown-linux-musl/release/wal-g-postgres && \
    chown -R app:app ${HOMEDIR} && \
    chmod -R g+rw ${HOMEDIR} && \
    chmod u+rwx target/x86_64-unknown-linux-musl/release/wal-g-exporter && \
    chmod u+rwx target/x86_64-unknown-linux-musl/release/wal-g-postgres
    
# Use distroless as minimal base image to package the manager binary
# Refer to https://github.com/GoogleContainerTools/distroless for more details
FROM gcr.io/distroless/base:latest
ARG HOMEDIR
ARG WALG_TARGET
ARG USER
ARG GROUP

COPY --chown=${USER}:${GROUP} --from=builder ${HOMEDIR} ${HOMEDIR}
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /workspace/target/x86_64-unknown-linux-musl/release/wal-g-exporter /usr/bin/wal-g-exporter
COPY --from=builder /workspace/target/x86_64-unknown-linux-musl/release/wal-g-postgres /usr/bin/wal-g-${WALG_TARGET}
COPY --from=builder /workspace/log4rs.yml ${HOMEDIR}/

WORKDIR ${HOMEDIR}
USER ${USER}

ENTRYPOINT ["/usr/bin/wal-g-exporter"]