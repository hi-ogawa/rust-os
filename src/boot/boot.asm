; Multiboot header (cf. https://www.gnu.org/software/grub/manual/multiboot/multiboot.html)
section .multiboot_header
align 4
dd 0x1BADB002
dd 1 << 0 | 1 << 1
dd - (0x1BADB002 + (1 << 0 | 1 << 1))


; Stack
section .kernel_stack
stack_bottom:
  resb 1 << 17
stack_top:


; Page table
section .bss
align 1 << 12
p4_table:
  resb 1 << 12
p3_table:
  resb 1 << 12
p2_table:
  resb 1 << 12
p1_tables: ; p1_table x 2^9
  resb 1 << 21

; GDT
section .rodata
gdt64:
  dq 0
.code: equ $ - gdt64
  dq (1<<43) | (1<<44) | (1<<47) | (1<<53)
.pointer:
  dw $ - gdt64 - 1
  dq gdt64


; Kernel entry point
section .text
bits 32
global start
start:
  ; Initialize stack pointer
  mov esp, stack_top

  ; Check boot loader and cpu
  call check_multiboot
  call check_cpuid
  call check_long_mode

  ; Paging
  call setup_page_tables
  call enable_paging

  ; Load GDT
  lgdt [gdt64.pointer]
  jmp gdt64.code:start_long_mode


; Print error with "al" register as status
print_error:
  mov dword [0xb8000], 0x4f524f45 ; "ER"
  mov dword [0xb8004], 0x4f3a4f52 ; "R:"
  mov dword [0xb8008], 0x4f204f20 ; "  "
  mov byte  [0xb800a], al
  hlt


; Verify multiboot loader has loaded this kernel by checking eax register
check_multiboot:
  cmp eax, 0x2BADB002
  jne .check_multiboot_fail
  ret
.check_multiboot_fail:
  mov al, "0"
  call print_error


; Veryfy cpuid instruction is supported by probing FLAGS register (cf. https://wiki.osdev.org/CPUID)
check_cpuid:
  ; Save original FLAGS on "ecx"
  pushfd
  pop ecx
  ; Try to flip ID bit and put result in "eax"
  mov eax, ecx
  xor eax, 1 << 21
  push eax
  popfd
  pushfd
  pop eax
  ; Restore original FLAGS
  push ecx
  popfd
  ; Flip failed <=> eax == ecx
  xor eax, ecx
  jz .check_cpuid_fail
  ret
.check_cpuid_fail:
  mov al, "1"
  call print_error


; Verify long mode is supported by cpuid instruction (cf. https://wiki.osdev.org/Setting_Up_Long_Mode)
check_long_mode:
  mov eax, 0x80000000
  cpuid
  cmp eax, 0x80000001
  jb .check_long_mode_fail
  mov eax, 0x80000001
  cpuid
  test edx, 1 << 29
  jz .check_long_mode_fail
  ret
.check_long_mode_fail:
  mov al, "2"
  call print_error


; Setup 1GB identity map
; 1 x 1 x 2^9 x 2^9 = 2^18 pages = 2^18 x 2^12 bytes
setup_page_tables:
  ; Chain first entry p4 -> p3 -> p2
  mov eax, p2_table
  or eax, (1 << 0 | 1 << 1) ; flag (writable + present)
  mov [p3_table], eax

  mov eax, p3_table
  or eax, (1 << 0 | 1 << 1)
  mov [p4_table], eax

  ; loop p2 entries
  mov ecx, 0
.p2_loop:
  ; eax = (p1_tables + 2^12 * ecx) | 0b11
  mov eax, 1 << 12
  mul ecx
  add eax, p1_tables
  or eax, (1 << 0 | 1 << 1)
  mov [p2_table + 8 * ecx], eax
  add ecx, 1
  cmp ecx, (1 << 9)
  jne .p2_loop

  ; loop p1 entries
  mov ecx, 0
.p1_loop:
  ; eax = 2^12 * ecx | 0b11
  mov eax, 1 << 12
  mul ecx
  or eax, (1 << 0 | 1 << 1)
  mov [p1_tables + 8 * ecx], eax
  add ecx, 1
  cmp ecx, (1 << 18)
  jne .p1_loop

  ret


; Enable paging (cf. https://en.wikipedia.org/wiki/Control_register)
enable_paging:
  ; cr3 = p4
  mov eax, p4_table
  mov cr3, eax

  ; PAE bit on cr4
  mov eax, cr4
  or eax, 1 << 5
  mov cr4, eax

  ; LME bit on EFER
  mov ecx, 0xC0000080
  rdmsr
  or eax, 1 << 8
  wrmsr

  ; PG bit on cr0
  mov eax, cr0
  or eax, 1 << 31
  mov cr0, eax
  ret


; Long mode entry point
section .text
bits 64
global start_long_mode
start_long_mode:
  ; Clear data segment registers
  mov ax, 0
  mov ss, ax ; TODO: yasm says `ss` is ignored in 64bits mode, but ISR doesn't work without this
  mov fs, ax
  mov gs, ax

  ; Call Rust entrypoint
  extern kernel_main
  call kernel_main

; For debugging
global print_error64
print_error64:
  mov dword [0xb8000], 0x4f524f45 ; "ER"
  mov dword [0xb8004], 0x4f3a4f52 ; "R:"
  mov dword [0xb8008], 0x4f204f20 ; "  "
  mov byte  [0xb800a], al
  hlt
