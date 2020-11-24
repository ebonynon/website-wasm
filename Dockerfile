FROM debian:buster-slim

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        gcc \
        libc6-dev \
        wget \
        curl \
        ; \
    \
    url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain nightly; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version; \
    \
    apt-get remove -y --auto-remove \
        wget \
        ; \
    rm -rf /var/lib/apt/lists/*;

RUN mkdir /usr/app
COPY . /usr/app

# Create app directory
WORKDIR /usr/app

# Install dependencies, build n run
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 
RUN cargo +nightly install miniserve
RUN build/build.sh

# New
EXPOSE 8080
#CMD [ "bash", "run.sh" ]
ENTRYPOINT ["/bin/bash", "/usr/app/run.sh"]