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

use core::mem::size_of;

extern "C" {
    static __heap_start: usize;
    static __heap_size: usize;
}

fn get_num_pages() -> usize {
    (unsafe { &__heap_size as *const usize as usize } / crate::common::PAGE_SIZE)
}
fn get_alloc_start() -> usize {
    align(
        unsafe { &__heap_start as *const usize as usize } + get_num_pages() * size_of::<Page>(),
        crate::common::PAGE_ORDER,
    )
}

const fn align(val: usize, order: usize) -> usize {
    (((val - 1) >> order) + 1) << order
}

#[repr(u8)]
pub enum PageBits {
    Empty = 0,
    Taken = 1 << 0,
    Last = 1 << 1,
}

pub struct Page {
    flags: u8,
}

impl Page {
    pub fn test_bits(&self, bits: u8) -> bool {
        self.flags & bits != 0
    }

    pub fn set_bits(&mut self, bits: u8) {
        self.flags = bits;
    }

    pub fn add_bits(&mut self, bits: u8) {
        self.flags = self.flags | bits;
    }
}

pub fn alloc(pages: usize) -> *mut u8 {
    if pages == 0 {
        return 0 as *mut u8;
    }

    unsafe {
        let ptr = &__heap_start as *const usize as *mut Page;
        let mut cur_start: usize = 0;
        let mut found = false;
        for i in 0..get_num_pages() - pages {
            if found && i - cur_start >= pages {
                for j in cur_start..cur_start + pages {
                    (*ptr.add(j)).flags = PageBits::Taken as u8;
                }
                (*ptr.add(cur_start + pages - 1)).add_bits(PageBits::Last as u8);
                return (get_alloc_start() + crate::common::PAGE_SIZE * cur_start) as *mut u8;
            } else if !(*ptr.add(i)).test_bits(PageBits::Taken as u8) {
                if !found {
                    cur_start = i;
                }
                found = true;
            } else {
                found = false;
            }
        }
    }

    0 as *mut u8
}
