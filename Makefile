# Options
example := dev
cargo_options := # e.g. --features os-test
qemu_options := # e.g. -display none -d int -no-reboot
qemu_success := 123

iso := build/os.iso
isodir := build/isodir
kernel := build/kernel.bin
rust_kernel := target/target/debug/examples/lib$(example).a

run: $(iso)
	qemu-system-x86_64 -cdrom $(iso) -serial stdio -device isa-debug-exit $(qemu_options); test "$$?" = "$(qemu_success)"

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
	cargo rustc --example $(example) $(cargo_options)

clean:
	@cargo clean
	@rm -rf build

.PHONY: clean run iso kernel cargo
