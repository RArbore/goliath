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

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("riscv64-elf-gcc")
        .args(&[
            "-march=rv64gc",
            "-static",
            "-fvisibility=hidden",
            "-nostartfiles",
            "-nostdlib",
            "-c",
            "-Tld/qemu.ld",
            "src/asm/boot.S",
            "-o",
            format!("{}/boot.o", out_dir).as_str(),
        ])
        .status()
        .unwrap();
    Command::new("ar")
        .args(&["crs", "libboot.a", "boot.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=boot");
    println!("cargo:rerun-if-changed=src/asm/boot.S");
}
