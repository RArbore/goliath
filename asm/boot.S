#
# This file is part of goliath.
# goliath is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# any later version.
# goliath is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
# You should have received a copy of the GNU General Public License
# along with goliath. If not, see <https://www.gnu.org/licenses/>.
#

    .align 4
    .equ UART_REG_TXFIFO, 0
    .equ UART_BASE, 0x10013000

    .section .text
    .globl _start

_start:
    csrr t0, mhartid
    bnez t0, halt

    la a0, hello_msg

    li t0, UART_BASE

puts_loop:
    lbu t1, (a0)
    beqz t1, halt
puts_wait:
    lw t2, UART_REG_TXFIFO(t0)
    bltz t2, puts_wait
    sw t1, UART_REG_TXFIFO(t0)
    add a0, a0, 1
    j puts_loop

halt:
    j halt

    .section .rodata
hello_msg:
    .string "Hello, world!\n"
