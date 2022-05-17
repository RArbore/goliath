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

    .section .text.init
    .global _start
    .global __kernel_stack_start
    .global kinit
_start:
    la a0, __bss_start
    la a1, __bss_end
    bgeu a0, a1, 2f
1:
    sd zero, 0(a0)
    addi a0, a0, 8
    bltu a0, a1, 1b
2:
    la sp, __kernel_stack_start
    li a0, 0x10000
    csrr a1, mhartid
    addi a1, a1, 1
    mul a0, a0, a1
    add sp, sp, a0
    call kinit

