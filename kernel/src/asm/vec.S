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
    .global timervec
timervec:   	
    csrrw a0, mscratch, a0
    sd a1, 0(a0)
    sd a2, 8(a0)
    sd a3, 16(a0)

    ld a1, 32(a0)
    ld a2, 40(a0)
    ld a3, 0(a1)
    add a3, a3, a2
    sd a3, 0(a1)

    li a1, 2
    csrw sip, a1

    sd a1, 0(a0)
    sd a2, 8(a0)
    sd a3, 16(a0)
    csrrw a0, mscratch, a0
    mret

    .end
