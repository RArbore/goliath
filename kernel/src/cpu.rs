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

use core::arch::asm;

#[repr(usize)]
pub enum SATPMode {
    Off = 0,
    Sv39 = 8,
    Sv48 = 9,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TrapFrame {
    pub regs: [usize; 32],
    pub fregs: [usize; 32],
    pub satp: usize,
    pub trap_stack: *mut u8,
    pub hartid: usize,
}

pub static mut KERNEL_TRAP_FRAME: [TrapFrame; crate::common::MAX_CPUS] = [TrapFrame {
    regs: [0; 32],
    fregs: [0; 32],
    satp: 0,
    trap_stack: 0 as *mut u8,
    hartid: 0,
};
    crate::common::MAX_CPUS];

pub fn hart_id() -> usize {
    let mut hart_id: usize;
    unsafe {
        asm!("mv {}, tp", out(reg) hart_id);
    }
    hart_id
}

pub fn mscratch_write(val: usize) {
    unsafe {
        asm!("csrw mscratch, {}", in(reg) val);
    }
}

pub fn mscratch_read() -> usize {
    unsafe {
        let rval;
        asm!("csrr {}, mscratch", out(reg) rval);
        rval
    }
}

pub fn sscratch_write(val: usize) {
    unsafe {
        asm!("csrw sscratch, {}", in(reg) val);
    }
}

pub fn sscratch_read() -> usize {
    unsafe {
        let rval;
        asm!("csrr {}, sscratch", out(reg) rval);
        rval
    }
}

pub const fn build_satp(mode: SATPMode, addr: usize) -> usize {
    (mode as usize) << 60 | (addr >> 12) & 0xff_ffff_ffff
}
