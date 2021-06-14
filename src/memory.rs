pub const PAGE_SIZE: u64 = 1 << 12; // 4096 = 4KB

pub struct FrameAllocator<I1, I2> {
    index: u64,
    usable: I1,
    occupied: I2,
    usable_max: u64,
}

impl<I1, I2> FrameAllocator<I1, I2>
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

    pub fn allocate(&mut self) -> Option<u64> {
        loop {
            let index = self.index;
            let addr_lo = PAGE_SIZE * index;
            let addr_hi = addr_lo + PAGE_SIZE;
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

    pub fn deallocate(&mut self) {
        todo!()
    }
}
