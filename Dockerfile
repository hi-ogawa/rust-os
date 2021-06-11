FROM archlinux:base-devel-20210530.0.24217

# Dependencies
RUN pacman --noconfirm -Sy && \
    pacman --noconfirm -S \
  rustup \
  grub libisoburn mtools \
  yasm \
  qemu-headless \
  python python-yaml

# Rust toolchain
RUN rustup toolchain install nightly-2021-06-09
RUN rustup component add rust-src

RUN mkdir -p /app
WORKDIR /app
