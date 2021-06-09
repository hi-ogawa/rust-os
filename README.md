OS

```
make run
```

Misc

```
# Rust toolchain
$ rustc --version
rustc 1.54.0-nightly (c79419af0 2021-06-04)

# Run in docker
$ docker build -t hiogawa/rust-os .
$ docker run -it --rm hiogawa/rust-os make run example=uart qemu_options="-display none"
```

References

- https://os.phil-opp.com
- https://os.phil-opp.com/edition-1
- https://www.gnu.org/software/grub/manual/multiboot/multiboot.html
- https://www.csie.ntu.edu.tw/~comp03/nasm/nasmdoci.html
- https://github.com/hi-ogawa/bare-bones-os
