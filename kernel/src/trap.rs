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

#[no_mangle]
pub extern "C" fn m_trap(
    epc: usize,
    _tval: usize,
    cause: usize,
    _hart: usize,
    _status: usize,
    _frame: &mut crate::cpu::TrapFrame,
) -> usize {
    let is_async = cause >> 63 & 1 == 1;
    let cause_num = cause & 0xfff;
    let return_pc = epc;

    if is_async {
        match cause_num {
            7 => {
                crate::drivers::clint::clint_set_future(10_000_000);
                unsafe {
                    let hart = crate::drivers::ns16550a::UART_DRIVER_HANDLE.peek_hart();
                    if hart == crate::cpu::hart_id() {
                        crate::drivers::ns16550a::UART_DRIVER_HANDLE
                            .force()
                            .uart_put_byte(b'0' + crate::cpu::hart_id() as u8);
                    } else {
                        crate::drivers::ns16550a::UART_DRIVER_HANDLE
                            .lock()
                            .uart_put_byte(b'0' + crate::cpu::hart_id() as u8);
                    }
                };
            }
            _ => {}
        }
    } else {
    }

    return_pc
}
