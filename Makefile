example := dev
qemu_display := gtk

iso := build/os.iso
isodir := build/isodir
kernel := build/kernel.bin
rust_kernel := target/target/debug/examples/lib$(example).a

run: $(iso)
	qemu-system-x86_64 -cdrom $(iso) -serial stdio -display $(qemu_display)

iso: $(iso)

$(iso): $(kernel) src/boot/grub.cfg
	@mkdir -p $(isodir)/boot/grub
	@cp -f src/boot/grub.cfg $(isodir)/boot/grub
	@cp -f $(kernel) $(isodir)/boot
	grub-mkrescue -o $(iso) $(isodir)

kernel: $(kernel)

$(kernel): src/boot/linker.ld src/boot/boot.asm cargo
	@mkdir -p build/boot
	yasm -f elf64 src/boot/boot.asm -o build/boot/boot.o
	ld -n -o $(kernel) -T src/boot/linker.ld build/boot/boot.o $(rust_kernel)

cargo:
	cargo build --target target.json --example $(example)

cargo-all:
	cargo build --target target.json --examples

clean:
	@cargo clean
	@rm -rf build

.PHONY: clean run iso kernel cargo
