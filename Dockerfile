FROM rustembedded/cross:aarch64-unknown-linux-gnu

RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install -t xenial-backports --assume-yes libgtk-3-dev:arm64 x11proto-core-dev:arm64

ENV PKG_CONFIG_LIBDIR=/usr/local/lib/aarch64-linux-gnu/pkgconfig:/usr/lib/aarch64-linux-gnu/pkgconfig:/usr/share/pkgconfig/
ENV PKG_CONFIG_ALLOW_CROSS=1
