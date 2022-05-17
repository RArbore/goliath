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

use crate::mutex::*;

extern "C" {
    static __uart_base_addr: usize;
}

static DEVICE_LOCK: Mutex<()> = Mutex::new(());

unsafe fn get_uart_base_addr() -> *mut u8 {
    &__uart_base_addr as *const usize as *mut u8
}

pub fn uart_get_byte() -> Option<u8> {
    DEVICE_LOCK.lock();
    unsafe {
        if get_uart_base_addr().add(5).read_volatile() & 1 != 0 {
            Some(get_uart_base_addr().read_volatile())
        } else {
            None
        }
    }
}

pub fn uart_put_byte(byte: u8) {
    DEVICE_LOCK.lock();
    loop {
        if unsafe { get_uart_base_addr().add(5).read_volatile() } & (1 << 5) != 0 {
            break;
        }
    }
    unsafe {
        get_uart_base_addr().write_volatile(byte);
    }
}
