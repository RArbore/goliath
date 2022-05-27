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
use core::sync::atomic::{AtomicBool, Ordering};
use riscv::register::*;

pub mod common;
pub mod cpu;
pub mod drivers;
pub mod page;
pub mod spinlock;
pub mod trap;

static KINIT_FINISHED: AtomicBool = AtomicBool::new(false);

#[no_mangle]
pub unsafe extern "C" fn kinit() {
    pmpaddr0::write(0x3fffffffffffff);
    pmpcfg0::write(0xf);

    let satp = cpu::build_satp(cpu::SATPMode::Off, 0);
    cpu::mscratch_write(&mut cpu::KERNEL_TRAP_FRAME[0] as *mut cpu::TrapFrame as usize);
    cpu::sscratch_write(&mut cpu::KERNEL_TRAP_FRAME[0] as *mut cpu::TrapFrame as usize);
    cpu::KERNEL_TRAP_FRAME[0].satp = satp;
    cpu::KERNEL_TRAP_FRAME[0].hartid = 0;
    KINIT_FINISHED.store(true, Ordering::Release);
}

#[no_mangle]
pub unsafe extern "C" fn kinit_hart(hartid: usize) {
    pmpaddr0::write(0x3fffffffffffff);
    pmpcfg0::write(0xf);

    loop {
        if KINIT_FINISHED
            .compare_exchange(true, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            break;
        }
    }
    let satp = cpu::build_satp(cpu::SATPMode::Off, 0);
    cpu::mscratch_write(&mut cpu::KERNEL_TRAP_FRAME[hartid] as *mut cpu::TrapFrame as usize);
    cpu::sscratch_write(&mut cpu::KERNEL_TRAP_FRAME[hartid] as *mut cpu::TrapFrame as usize);
    cpu::KERNEL_TRAP_FRAME[0].satp = satp;
    cpu::KERNEL_TRAP_FRAME[hartid].hartid = hartid;
}

#[no_mangle]
extern "C" fn kmain() -> ! {
    drivers::clint::clint_set_future(10_000_000);
    loop {
        let uart = drivers::ns16550a::UART_DRIVER_HANDLE.lock();
        if let Some(_) = uart.uart_get_byte() {
            uart.uart_put_byte(b'0' + crate::cpu::hart_id() as u8);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
