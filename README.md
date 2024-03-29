OS

```
# Run example
make run example=dev

# Testing
python test.py -k dev -v

# Rust toolchain
$ rustup default
nightly-2021-06-09-x86_64-unknown-linux-gnu (default)

# Run in docker
# -- build base image --
$ docker build -t hiogawa/rust-os-deps - < Dockerfile
$ docker push hiogawa/rust-os-deps
# -- run --
$ docker-compose run dev make run example=dev qemu_options='-display none'
```

References

- https://os.phil-opp.com
- https://os.phil-opp.com/edition-1
- https://www.gnu.org/software/grub/manual/multiboot/multiboot.html
- https://www.csie.ntu.edu.tw/~comp03/nasm/nasmdoci.html
- https://github.com/hi-ogawa/bare-bones-os
