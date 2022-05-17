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

extern "C" {
    static __bss_start: usize;
    static __bss_end: usize;
}

#[no_mangle]
pub unsafe extern "C" fn _start() {
    let mut bss_ptr = &__bss_start as *const usize as *mut usize;
    let end_bss_ptr = &__bss_end as *const usize as *mut usize;

    while bss_ptr < end_bss_ptr {
        *bss_ptr = 0;
        bss_ptr = bss_ptr.add(1);
    }

    asm!(
        "la sp, __kernel_stack_start",
        "li a0, 0x10000",
        "csrr a1, mhartid",
        "addi a1, a1, 1",
        "mul a0, a0, a1",
        "add sp, sp, a0",
    );

    crate::kinit();
}
