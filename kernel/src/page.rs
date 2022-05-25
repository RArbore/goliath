/*
 * This file is part of goliath.
 * goliath is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * any later version.
 * goliath is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License
 * along with goliath. If not, see <https://www.gnu.org/licenses/>.
 */

extern "C" {
    static __heap_start: usize;
    static __heap_size: usize;
}

const NUM_PAGES: usize = __heap_size / PAGE_SIZE;

unsafe fn get_heap_start() -> *mut u8 {
    &__heap_start as *const usize as _
}

unsafe fn get_heap_size() -> usize {
    &__heap_size as *const usize as _
}

#[repr(u8)]
pub enum PageBits {
    Empty = 0,
    Taken = 1 << 0,
    Last = 1 << 1,
}

pub struct Page {
    flags: PageBits,
}

impl Page {
    pub fn test_bits(&self, bits: PageBits) {
        self.flags & bits != 0
    }
}

pub fn alloc(pages: usize) -> *mut u8 {
    if pages == 0 {
        0
    }

    let ptr = __heap_start as *mut Page;
    let mut cur_start: usize = 0;
    let mut found = false;
    for i in 0..NUM_PAGES - pages {
        if found && i - cur_start >= pages {
            ptr.add(cur_start)
        } else if !(unsafe { *ptr.add(i) }).test_bits(PageBits::Taken) {
            if !found {
                cur_start = i;
            }
            found = true;
        } else {
            found = false;
        }
    }

    0
}
