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

    .option norvc

    .align 4
    .section .text.init
    .global _start
    .global kinit
_start:
    .option push
    .option norelax
    la gp, __global_pointer
    .option pop

    csrw satp, zero

    la a0, __bss_start
    la a1, __bss_end
    bgeu a0, a1, bss_loop_end
bss_loop:
    sd zero, 0(a0)
    addi a0, a0, 8
    bltu a0, a1, bss_loop
bss_loop_end:

    la sp, __kernel_stack_end

    li t0, 0b11 << 11
    csrw mstatus, t0
    csrw mie, zero
    la t0, kinit
    csrw mepc, t0
    
    la ra, after_kinit
    mret

after_kinit:	
    li t0, (0b01 << 11) | (1 << 7) | (1 << 5)
    csrw mstatus, t0

    la t1, m_trap_vec
    csrw mtvec, t1

    la t2, kmain
    csrw mepc, t2

    li t3, 0x888
    csrw mie, t3

    csrr tp, mhartid

    mret

    .end
