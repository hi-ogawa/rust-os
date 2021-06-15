pub const PAGE_SIZE: u64 = 1 << 12; // 4096 = 4KB

// For now, just type aliases
pub type Frame = u64;
pub type Page = u64;
pub type PhysicalAddress = u64;
pub type VirtualAddress = u64;

pub fn frame_to_address(frame: Frame) -> PhysicalAddress {
    frame * PAGE_SIZE
}

pub fn adress_to_frame(addr: PhysicalAddress) -> Frame {
    addr / PAGE_SIZE
}

pub trait FrameAllocator {
    fn allocate(&mut self) -> Option<Frame>;
}

pub struct SimpleFrameAllocator<I1, I2> {
    index: Frame,
    usable: I1,
    occupied: I2,
    usable_max: u64,
}

impl<I1, I2> SimpleFrameAllocator<I1, I2>
where
    I1: Iterator<Item = (u64, u64)> + Clone,
    I2: Iterator<Item = (u64, u64)> + Clone,
{
    pub fn new(usable: I1, occupied: I2) -> Self {
        let usable_max = usable.clone().map(|(_, hi)| hi).max().unwrap_or(0);
        Self {
            index: 0,
            usable,
            occupied,
            usable_max,
        }
    }
}

impl<I1, I2> FrameAllocator for SimpleFrameAllocator<I1, I2>
where
    I1: Iterator<Item = (u64, u64)> + Clone,
    I2: Iterator<Item = (u64, u64)> + Clone,
{
    fn allocate(&mut self) -> Option<Frame> {
        loop {
            let index = self.index;
            let addr_lo = frame_to_address(index);
            let addr_hi = frame_to_address(index + 1);
            if addr_lo >= self.usable_max {
                return None;
            }
            self.index += 1;
            let usable = self
                .usable
                .clone()
                .any(|(lo, hi)| lo <= addr_lo && addr_hi <= hi);
            let occupied = self
                .occupied
                .clone()
                .any(|(lo, hi)| lo <= addr_lo && addr_hi <= hi);
            if !usable || occupied {
                continue;
            }
            return Some(index);
        }
    }
}

pub mod paging {
    use crate::memory::{
        frame_to_address, Frame, FrameAllocator, Page, PhysicalAddress, VirtualAddress, PAGE_SIZE,
    };
    use crate::util::address_cast_mut;

    // Page entry flag
    pub const PRESENT: u64 = 1 << 0;
    pub const WRITABLE: u64 = 1 << 1;

    pub const TABLE_SIZE: usize = 1 << 9; // = 512 = 4096 / 8 = PAGE_SIZE / sizeof(Entry)

    pub type Entry = u64;
    pub type Table = [Entry; TABLE_SIZE];

    // --16-- --9-- --9-- --9-- --9-- --12--
    // 177777  777   777   777   777   0000
    const ACTIVE_P4_TABLE_ADDRESS: VirtualAddress = 0o177777_777_777_777_777_0000;

    pub fn get_p4_table<'a>() -> &'a mut Table {
        unsafe { address_cast_mut(ACTIVE_P4_TABLE_ADDRESS as usize) }
    }

    pub fn get_child_table_address<'a>(parent: &'a Table, index: usize) -> VirtualAddress {
        let p_addr = parent as *const _ as u64;
        let c_addr = (p_addr | ((index as u64) << 3)) << 9;
        c_addr
    }

    pub fn get_child_table<'a, 'b>(parent: &'a Table, index: usize) -> Option<&'b mut Table> {
        if parent[index] & PRESENT == 0 {
            return None;
        }
        let addr = get_child_table_address(parent, index);
        Some(unsafe { address_cast_mut(addr as usize) })
    }

    pub fn virtual_to_page(addr: VirtualAddress) -> Page {
        // TODO: Exclude invalid sign extension
        (addr >> 12) & 0o777_777_777_777
    }

    pub fn page_to_virtual(page: Page) -> VirtualAddress {
        assert!(page <= 0o777_777_777_777);
        let extension = if (page >> (4 * 9 - 1)) == 0 {
            0
        } else {
            0o177777
        };
        (page | (extension << (4 * 9))) << 12
    }

    pub fn page_p4_index(page: Page) -> usize {
        ((page >> (9 * 3)) & 0o777) as usize
    }

    pub fn page_p3_index(page: Page) -> usize {
        ((page >> (9 * 2)) & 0o777) as usize
    }

    pub fn page_p2_index(page: Page) -> usize {
        ((page >> (9 * 1)) & 0o777) as usize
    }

    pub fn page_p1_index(page: Page) -> usize {
        ((page >> (9 * 0)) & 0o777) as usize
    }

    pub fn virtual_to_physical(addr: VirtualAddress) -> Option<PhysicalAddress> {
        let offset = addr % PAGE_SIZE;
        let page = virtual_to_page(addr);
        let p4 = get_p4_table();
        let p3 = get_child_table(p4, page_p4_index(page))?;
        let p2 = get_child_table(p3, page_p3_index(page))?;
        let p1 = get_child_table(p2, page_p2_index(page))?;
        let entry = p1[page_p1_index(page)];
        let phys_addr = entry & !(PAGE_SIZE - 1) + offset;
        Some(phys_addr)
    }

    pub fn initialize_page(page: Page) {
        let addr = page_to_virtual(page);
        assert!(virtual_to_physical(addr) != None);
        let page_data = unsafe { address_cast_mut::<[u8; PAGE_SIZE as usize]>(addr as usize) };
        page_data.iter_mut().for_each(|x| *x = 0);
    }

    pub fn get_or_create_child_table<'a, 'b, 'c, A: FrameAllocator>(
        parent: &'a mut Table,
        index: usize,
        allocator: &'b mut A,
    ) -> &'c mut Table {
        if let Some(child) = get_child_table(parent, index) {
            return child;
        }
        let addr = get_child_table_address(parent, index);
        let page = virtual_to_page(addr);
        let frame = allocator.allocate().unwrap();
        parent[index] = frame_to_address(frame) | PRESENT | WRITABLE;
        initialize_page(page);
        get_child_table(parent, index).unwrap()
    }

    pub fn map_page_to_frame<'a, A: FrameAllocator>(
        page: Page,
        frame: Frame,
        allocator: &'a mut A,
    ) {
        let p4 = get_p4_table();
        let p3 = get_or_create_child_table(p4, page_p4_index(page), allocator);
        let p2 = get_or_create_child_table(p3, page_p3_index(page), allocator);
        let p1 = get_or_create_child_table(p2, page_p2_index(page), allocator);
        p1[page_p1_index(page)] = frame_to_address(frame) | PRESENT | WRITABLE;
        initialize_page(page);
    }

    pub fn unmap_page(page: Page) {
        let addr = page_to_virtual(page);
        assert!(virtual_to_physical(addr) != None);
        let p4 = get_p4_table();
        let p3 = get_child_table(p4, page_p4_index(page)).unwrap();
        let p2 = get_child_table(p3, page_p3_index(page)).unwrap();
        let p1 = get_child_table(p2, page_p2_index(page)).unwrap();
        p1[page_p1_index(page)] = 0;
    }
}
