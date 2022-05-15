#   This file is part of goliath. \
\
    goliath is free software: you can redistribute it and/or modify \
    it under the terms of the GNU Lesser General Public License as published by \
    the Free Software Foundation, either version 3 of the License, or \
    any later version. \
\
    goliath is distributed in the hope that it will be useful, \
    but WITHOUT ANY WARRANTY; without even the implied warranty of \
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the \
    GNU Lesser General Public License for more details. \
\
    You should have received a copy of the GNU Lesser General Public License \
    along with goliath. If not, see <https://www.gnu.org/licenses/>.

run: boot.bin
	qemu-system-riscv64 -machine sifive_e -nographic -kernel boot.bin
boot.bin: boot.S qemu.ld
	riscv64-elf-gcc -march=rv64gc -static -fvisibility=hidden -nostdlib -nostartfiles -Tqemu.ld boot.S -o boot.bin
