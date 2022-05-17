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

pub mod common;
pub mod drivers;
pub mod spinlock;

extern "C" {
    static kernelvec: usize;
}

#[no_mangle]
pub unsafe extern "C" fn kinit() {
    mstatus::set_mpp(mstatus::MPP::Supervisor);
    mepc::write(kmain as usize);
    satp::write(0);

    asm!("csrw mie, zero");

    asm!("li t0, 0xffff");
    asm!("csrw medeleg, t0");
    asm!("li t0, 0xffff");
    asm!("csrw mideleg, t0");

    sie::set_sext();
    sie::set_ssoft();
    sie::set_stimer();

    pmpaddr0::write(0x3fffffffffffff);
    pmpcfg0::write(0xf);

    drivers::clint::timer_init();

    let id = mhartid::read();
    asm!("mv tp, {0}", in(reg) id);

    stvec::write(kernelvec, stvec::TrapMode::Direct);

    asm!("mret");
}

#[no_mangle]
extern "C" fn kmain() -> ! {
    loop {
        let uart = drivers::ns16550a::UART_DRIVER_HANDLE.lock();
        if let Some(byte) = uart.uart_get_byte() {
            uart.uart_put_byte(byte);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
