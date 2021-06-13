// cf. https://www.gnu.org/software/grub/manual/multiboot/multiboot.html#Boot-information-format

const FLAG_ELF_SECTION: u32 = 1 << 5;
const FLAG_MEMORY_MAP: u32 = 1 << 6;

// 116 bytes
#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct BootInfo {
    pub flags: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub syms: [u32; 4],
    pub mmap_length: u32,
    pub mmap_addr: u32,
    pub drives_length: u32,
    pub drives_addr: u32,
    pub config_table: u32,
    pub boot_loader_name: u32,
    pub apm_table: u32,
    pub vbe_control_info: u32,
    pub vbe_mode_info: u32,
    pub vbe_mode: u16,
    pub vbe_interface_seg: u16,
    pub vbe_interface_off: u16,
    pub vbe_interface_len: u16,
    pub framebuffer_addr: u64,
    pub framebuffer_pitch: u32,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_bpp: u8,
    pub framebuffer_type: u8,
    pub color_info: [u8; 6],
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct MemoryMapInfo {
    pub size: u32,
    pub addr: u64,
    pub length: u64,
    pub type_: u32, // `type = 1` is usable memory
}

pub struct MemoryMapInfoIterator {
    addr: u32,
    length: u32,
    offset: u32,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct SectionHeadersInfo {
    pub num: u32,
    pub size: u32,
    pub addr: u32,
    pub shndx: u32,
}

// cf. https://en.wikipedia.org/wiki/Executable_and_Linkable_Format#Section_header
#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone, Default)]
pub struct SectionHeader {
    pub name: u32,
    pub type_: u32,
    pub flags: u64,
    pub addr: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub info: u32,
    pub addralign: u64,
    pub entsize: u64,
}

pub struct SectionHeaderIterator {
    info: SectionHeadersInfo,
    count: u32,
}

impl BootInfo {
    pub fn memory_usage(&self) -> (u64, u64) {
        let x0 = self as *const _ as usize;
        let x1 = x0 + core::mem::size_of::<BootInfo>();
        let y0 = self.mmap_addr;
        let y1 = y0 + self.mmap_length;
        let z0 = self.syms[2];
        let z1 = z0 + self.syms[0] * self.syms[1];
        (
            *[x0 as u64, y0 as u64, z0 as u64].iter().min().unwrap(),
            *[x1 as u64, y1 as u64, z1 as u64].iter().max().unwrap(),
        )
    }

    pub fn memory_maps(&self) -> MemoryMapInfoIterator {
        assert!(self.flags & FLAG_MEMORY_MAP != 0);
        MemoryMapInfoIterator {
            addr: self.mmap_addr,
            length: self.mmap_length,
            offset: 0,
        }
    }

    pub fn section_headers(&self) -> SectionHeaderIterator {
        assert!(self.flags & FLAG_ELF_SECTION != 0);
        let info = unsafe { *(&{ self.syms } as *const _ as *const SectionHeadersInfo) };
        assert!(info.size as usize == core::mem::size_of::<SectionHeader>());
        SectionHeaderIterator { info, count: 0 }
    }
}

impl Iterator for MemoryMapInfoIterator {
    type Item = MemoryMapInfo;

    fn next(&mut self) -> Option<Self::Item> {
        while self.offset < self.length {
            let mmap = unsafe { *((self.addr + self.offset) as *const MemoryMapInfo) };
            self.offset += mmap.size + 4;
            return Some(mmap);
        }
        return None;
    }
}

impl Iterator for SectionHeaderIterator {
    type Item = SectionHeader;

    fn next(&mut self) -> Option<Self::Item> {
        while self.count < self.info.num {
            let header = unsafe { *((self.info.addr + self.count * 64) as *const SectionHeader) };
            self.count += 1;
            return Some(header);
        }
        return None;
    }
}
