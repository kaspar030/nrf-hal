use std::env;
use std::path::PathBuf;

use ld_memory::{Memory, MemorySection};

fn main() {
    // generate linker script
    let memory = Memory::new()
        .add_section(MemorySection::new("RAM", 0x20000000, 256 * 1024))
        .add_section(
            MemorySection::new("FLASH", 0x0, 1024 * 1024)
                .pagesize(4096)
                .from_env_with_prefix("NRF52840_FLASH"),
        );

    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    memory
        .to_file(out.join("memory.x"))
        .expect("writing memory.x");

    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=build.rs");
}
