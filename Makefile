example := dev
qemu_display := gtk

iso := build/os.iso
isodir := build/isodir
kernel := build/kernel.bin
rust_kernel := target/target/debug/examples/lib$(example).a

run: $(iso)
	qemu-system-x86_64 -cdrom $(iso) -display $(qemu_display) -serial stdio -device isa-debug-exit

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
	cargo build --example $(example)

clean:
	@cargo clean
	@rm -rf build

.PHONY: clean run iso kernel cargo
