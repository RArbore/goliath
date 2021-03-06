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
    static __clint_addr: usize;
}

unsafe fn get_clint_addr() -> *mut u8 {
    &__clint_addr as *const usize as _
}

pub fn clint_set_future(cycles: u64) {
    unsafe {
        let mtimecmp = get_clint_addr().add(0x4000 + 8 * crate::cpu::hart_id()) as *mut u64;
        let mtime = get_clint_addr().add(0xbff8) as *const u64;
        mtimecmp.write_volatile(mtime.read_volatile() + cycles);
    }
}
