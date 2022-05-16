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

#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use riscv::register::*;

static UART_BASE_ADDR: usize = 0x1000_0000;
static HELLO_WORLD: &[u8] = b"Hello, World!";

#[no_mangle]
unsafe extern "C" fn kinit() {
    mstatus::set_mpp(mstatus::MPP::Supervisor);

    mepc::write(kmain as usize);

    asm!("csrw satp, zero");

    asm!("li t0, 0xffff");
    asm!("csrw medeleg, t0");
    asm!("li t0, 0xffff");
    asm!("csrw mideleg, t0");

    asm!("csrr a1, mhartid");
    asm!("mv tp, a1");

    asm!("mret");
}

#[no_mangle]
unsafe extern "C" fn kmain() -> ! {
    let ptr = UART_BASE_ADDR as *mut u8;
    for &c in HELLO_WORLD.iter() {
        loop {
            if ptr.add(5).read_volatile() & (1 << 5) != 0 {
                break;
            }
        }
        ptr.write_volatile(c);
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
