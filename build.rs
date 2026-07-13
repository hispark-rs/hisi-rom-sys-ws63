use std::path::{Path, PathBuf};

fn metadata(name: &str, path: &Path) {
    println!("cargo::metadata={name}={}", path.display());
    println!("cargo::rerun-if-changed={}", path.display());
}

fn main() {
    let root = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory"));
    let ws63 = root.join("assets/ws63");
    metadata("rom_symbols", &ws63.join("ws63_acore_rom.lds"));
    metadata("rom_callbacks", &ws63.join("ws63_acore_rom_callbacks.txt"));
    metadata("wifi_patches", &ws63.join("ws63_acore_wifi_patches.txt"));
    metadata("manifest", &ws63.join("manifest.txt"));
    println!("cargo::rerun-if-changed=build.rs");
}
