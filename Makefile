#   This file is part of vtrace. \
\
    vtrace is free software: you can redistribute it and/or modify \
    it under the terms of the GNU Lesser General Public License as published by \
    the Free Software Foundation, either version 3 of the License, or \
    any later version. \
\
    vtrace is distributed in the hope that it will be useful, \
    but WITHOUT ANY WARRANTY; without even the implied warranty of \
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the \
    GNU Lesser General Public License for more details. \
\
    You should have received a copy of the GNU Lesser General Public License \
    along with vtrace. If not, see <https://www.gnu.org/licenses/>.

run:
	cargo build --release
	qemu-system-riscv64 -machine virt -cpu rv64 -smp 4 -m 128M -bios none -nographic -kernel target/riscv64gc-unknown-none-elf/release/kernel
clean:
	rm -rf build/*.bin
	cargo clean

.DEFAULT: run
.PHONY: run clean
