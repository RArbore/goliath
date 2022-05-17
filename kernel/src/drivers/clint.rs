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

use riscv::register::*;

extern "C" {
    static __clint_addr: usize;
    static timervec: usize;
}

unsafe fn get_clint_addr() -> *mut u8 {
    &__clint_addr as *const usize as _
}

static mut TIMER_SCRATCH: [[u64; 8]; crate::common::MAX_CPUS] = [[0; 8]; crate::common::MAX_CPUS];

pub unsafe fn timer_init() {
    let id = mhartid::read();

    let interval_cycles = 20000u64;
    let mtimecmp = get_clint_addr().add(0x4000 + 8 * id) as *mut u64;
    let mtime = get_clint_addr().add(0xBFF8) as *const u64;
    mtimecmp.write_volatile(mtime.read_volatile() + interval_cycles);

    let scratch = &mut TIMER_SCRATCH[id];
    scratch[3] = mtime as u64;
    scratch[4] = mtimecmp as u64;
    scratch[5] = interval_cycles;
    mscratch::write(scratch.as_mut_ptr() as usize);

    mtvec::write(timervec, mtvec::TrapMode::Direct);

    mstatus::set_mie();
}
