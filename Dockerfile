FROM debian:stable

ARG CROSS_TRIPLE=arm-linux-gnueabihf

RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    git \
    file \
    openssh-client \
    gcc \
    libc6-dev \
    gcc-${CROSS_TRIPLE} \
    g++-${CROSS_TRIPLE} \
    libwebkit2gtk-4.0-37:armhf \
    libwebkit2gtk-4.0-dev:armhf

RUN ln -s /usr/bin/${CROSS_TRIPLE}-strip /usr/bin/cross-strip

# Setup Rust for ARMv7 cross-compilation

ARG RUST_VERSION=stable

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain ${RUST_VERSION} -y

ENV PATH=/root/.cargo/bin:$PATH

RUN rustup target add armv7-unknown-linux-gnueabihf

RUN cargo install cargo-deb
RUN cargo install just

ENV PKG_CONFIG_PATH=/usr/share/pkgconfig \
    PKG_CONFIG_ALLOW_CROSS=1 \
    PKG_CONFIG_LIBDIR=/usr/lib/${CROSS_TRIPLE}/pkgconfig/ \
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc \
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_STRIP=arm-linux-gnueabihf-strip