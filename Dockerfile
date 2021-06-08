FROM archlinux:base-devel-20210530.0.24217

# Dependencies
RUN pacman --noconfirm -Syu
RUN pacman --noconfirm -S \
  rustup \
  grub libisoburn mtools \
  yasm \
  qemu-headless

# Rust toolchain
RUN rustup toolchain install nightly-2021-06-04
RUN rustup component add rust-src

# Copy repository
RUN mkdir -p /app
WORKDIR /app

# Copy the main source
COPY . ./

# Build
RUN cargo build --examples
