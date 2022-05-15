/* 
 *  This file is part of goliath.
 *  goliath is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  any later version.
 *  goliath is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *  You should have received a copy of the GNU General Public License
 *  along with goliath. If not, see <https://www.gnu.org/licenses/>.
 */

OUTPUT_ARCH("riscv")
OUTPUT_FORMAT("elf64-littleriscv")

ENTRY( _start )
SECTIONS
{
	. = 0x20400000;
	.text : { *(.text) }
	.gnu_build_id : { *(.note.gnu.build-id) }
	.rodata : { * (.rodata) }

	. = 0x80000000;
	.data : { *(.data) }
	.sdata : { *(.sdata) }
	.debug : { *(.debug) }
	. += 0x1000;
	stack_top = .;

	_end = .;
}
