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

use core::panic::PanicInfo;
use riscv::register::*;

pub mod common;
pub mod cpu;
pub mod drivers;
pub mod spinlock;
pub mod trap;

#[no_mangle]
pub unsafe extern "C" fn kinit() {
    pmpaddr0::write(0x3fffffffffffff);
    pmpcfg0::write(0xf);

    let satp = cpu::build_satp(cpu::SATPMode::Off, 0);
    cpu::mscratch_write(&mut cpu::KERNEL_TRAP_FRAME[0] as *mut cpu::TrapFrame as usize);
    cpu::sscratch_write(&mut cpu::KERNEL_TRAP_FRAME[0] as *mut cpu::TrapFrame as usize);
    cpu::KERNEL_TRAP_FRAME[0].satp = satp;
    cpu::KERNEL_TRAP_FRAME[0].hartid = 0;
}

#[no_mangle]
pub unsafe extern "C" fn kinit_hart(hartid: usize) {
    let satp = cpu::build_satp(cpu::SATPMode::Off, 0);
    cpu::mscratch_write(&mut cpu::KERNEL_TRAP_FRAME[hartid] as *mut cpu::TrapFrame as usize);
    cpu::sscratch_write(&mut cpu::KERNEL_TRAP_FRAME[hartid] as *mut cpu::TrapFrame as usize);
    cpu::KERNEL_TRAP_FRAME[0].satp = satp;
    cpu::KERNEL_TRAP_FRAME[hartid].hartid = hartid;
}

#[no_mangle]
extern "C" fn kmain() -> ! {
    unsafe { (0x1000_0000 as *mut u8).write_volatile(b'C') };
    drivers::clint::clint_set_future(10_000_000);
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
