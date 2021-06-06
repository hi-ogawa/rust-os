iso := build/os.iso
isodir := build/isodir
kernel := build/kernel.bin

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) src/boot/grub.cfg
	@mkdir -p $(isodir)/boot/grub
	@cp -f src/boot/grub.cfg $(isodir)/boot/grub
	@cp -f $(kernel) $(isodir)/boot
	@grub-mkrescue -o $(iso) $(isodir)

kernel: $(kernel)

$(kernel): src/boot/linker.ld src/boot/boot.asm
	@mkdir -p build/boot
	@yasm -f elf64 src/boot/boot.asm -o build/boot/boot.o
	@ld -n -o $(kernel) -T src/boot/linker.ld build/boot/boot.o

all: kernel

clean:
	@rm -rf build

.PHONY: all clean run iso kernel
