// cf. https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct BootInfo {
    total_size: u32,
    reserved: u32,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct Tag {
    type_: u32,
    size: u32,
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum TagType {
    End = 0,
    MemoryMap = 6,
    Framebuffer = 8,
    SectionHeaderTable = 9,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct MemoryMapTag {
    type_: u32,
    size: u32,
    entry_size: u32,
    entry_version: u32,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct SectionHeaderTableTag {
    type_: u32,
    size: u32,
    // Be careful about the wrong spec https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html#ELF_002dSymbols
    num: u32,
    entsize: u32,
    shndx: u32,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct FramebufferTag {
    type_: u32,
    size: u32,
    addr: u64,
    pitch: u32,
    width: u32,
    height: u32,
    bpp: u8,
    type2_: u8,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct MemoryMapEntry {
    pub base_addr: u64,
    pub length: u64,
    pub type_: u32, // `type = 1` is usable memory
    reserved: u32,
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

pub struct TagIterator {
    address: u32,
    offset: u32,
}

impl Iterator for TagIterator {
    type Item = (Tag, u32);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let address = self.address + self.offset;
            let tag = unsafe { *(address as *const Tag) };
            if tag.type_ == TagType::End as _ {
                break;
            }
            self.offset += tag.size;
            self.offset += (8 - self.offset % 8) % 8;
            return Some((tag, address));
        }
        return None;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MemoryMapIterator {
    tag: MemoryMapTag,
    address: u32,
    offset: u32,
}

impl Iterator for MemoryMapIterator {
    type Item = MemoryMapEntry;

    fn next(&mut self) -> Option<Self::Item> {
        while self.offset < self.tag.size {
            let item = unsafe { *((self.address + self.offset) as *const MemoryMapEntry) };
            self.offset += self.tag.entry_size;
            return Some(item);
        }
        return None;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SectionHeaderIterator {
    tag: SectionHeaderTableTag,
    address: u32,
    offset: u32,
}

impl Iterator for SectionHeaderIterator {
    type Item = SectionHeader;

    fn next(&mut self) -> Option<Self::Item> {
        while self.offset < self.tag.size {
            let item = unsafe { *((self.address + self.offset) as *const SectionHeader) };
            self.offset += self.tag.entsize;
            return Some(item);
        }
        return None;
    }
}

impl BootInfo {
    pub fn tags(&self) -> TagIterator {
        TagIterator {
            address: self as *const _ as u32,
            offset: 8,
        }
    }

    pub fn find_tag<T: Copy>(&self, tag_type: TagType) -> Option<(T, u32)> {
        let (_, address) = self.tags().find(|(tag, _)| tag.type_ == tag_type as _)?;
        let tag = unsafe { *(address as *const T) };
        Some((tag, address))
    }

    pub fn framebuffer(&self) -> Option<FramebufferTag> {
        let (tag, _) = self.find_tag::<FramebufferTag>(TagType::Framebuffer)?;
        Some(tag)
    }

    pub fn memory_map(&self) -> Option<MemoryMapIterator> {
        let (tag, address) = self.find_tag::<MemoryMapTag>(TagType::MemoryMap)?;
        let offset = core::mem::size_of::<MemoryMapTag>();
        Some(MemoryMapIterator {
            tag,
            address,
            offset: offset as _,
        })
    }

    pub fn section_headers(&self) -> Option<SectionHeaderIterator> {
        let (tag, address) = self.find_tag::<SectionHeaderTableTag>(TagType::SectionHeaderTable)?;
        let offset = core::mem::size_of::<SectionHeaderTableTag>();
        Some(SectionHeaderIterator {
            tag,
            address,
            offset: offset as _,
        })
    }

    pub fn occupied_memory(&self) -> impl Iterator<Item = (u64, u64)> + Clone {
        let x0 = self as *const _ as u64;
        let x1 = x0 + (self.total_size as u64);
        let section_headers = self.section_headers().unwrap().filter(|m| m.size > 0);
        let y0 = section_headers.clone().map(|m| m.addr).min().unwrap();
        let y1 = section_headers
            .clone()
            .map(|m| m.addr + m.size)
            .max()
            .unwrap();
        crate::util::OwnedArrayIterator::new([(x0, x1), (y0, y1)])
    }

    pub fn usable_memory(&self) -> impl Iterator<Item = (u64, u64)> + Clone {
        self.memory_map()
            .unwrap()
            .filter(|m| m.type_ == 1)
            .map(|m| (m.base_addr, m.base_addr + m.length))
    }
}
