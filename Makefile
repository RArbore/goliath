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

build/kernel.bin: kernel/src/asm/boot.S target/riscv64gc-unknown-none-elf/release/libkernel.a
	riscv64-elf-gcc -march=rv64gc -static -fvisibility=hidden -nostartfiles -nostdlib -Tkernel/src/kernel.ld kernel/src/asm/boot.S -Ltarget/riscv64gc-unknown-none-elf/release/ -lkernel -o build/kernel.bin
target/riscv64gc-unknown-none-elf/release/libkernel.a:
	cargo build --release

run: build/kernel.bin
	qemu-system-riscv64 -machine virt -cpu rv64 -smp 4 -m 128M -bios none -nographic -kernel build/kernel.bin
clean:
	rm -rf build/*.bin
	cargo clean

.DEFAULT: run
.PHONY: run clean
