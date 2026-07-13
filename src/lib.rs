//! Generated WS63 application-core mask-ROM facts.
//!
//! Chip-neutral consumers should normally depend on `hisi-rom-sys` with the
//! `chip-ws63` feature. This backend owns WS63 fixed addresses and generated
//! artifacts so the facade does not pretend those facts are portable.

#![no_std]

/// Inclusive start of the known WS63 application-core ROM symbol window.
pub const ROM_START: usize = 0x0010_9000;
/// Exclusive end of the known WS63 application-core ROM symbol window.
pub const ROM_END: usize = 0x0014_C000;
/// Number of symbols in [`ROM_SYMBOLS_LD`].
pub const SYMBOL_COUNT: usize = 3_752;
/// Number of ordered callback veneer targets.
pub const CALLBACK_COUNT: usize = 227;
/// Number of mandatory Wi-Fi instruction patches.
pub const WIFI_PATCH_COUNT: usize = 37;

/// SHA-256 of the canonical language-neutral linker symbol source.
pub const SYMBOLS_SOURCE_SHA256: &str =
    "fa8f0071c07374d443db10dd2d5569a48e1e07486538b2eae701252e2188edbc";
/// SHA-256 of the canonical ordered callback source.
pub const CALLBACKS_SOURCE_SHA256: &str =
    "66934b3acfac013106f19ae40753eeb561cfbe21bc5e347b50c3b52e409ea92e";
/// SHA-256 of the canonical mandatory Wi-Fi patch source.
pub const WIFI_PATCHES_SOURCE_SHA256: &str =
    "993e3a3007c0a660eae7f006172cfdee32ce5c1c5fedafdc780c1005487f8d22";

/// Language-neutral linker assignments for every known ROM symbol.
pub const ROM_SYMBOLS_LD: &str = include_str!("../assets/ws63/ws63_acore_rom.lds");
/// Ordered callback veneer ABI consumed by the ROM callback archive.
pub const ROM_CALLBACKS: &str = include_str!("../assets/ws63/ws63_acore_rom_callbacks.txt");
/// Required original symbol names for the WS63 Wi-Fi patch payload.
pub const WIFI_PATCHES: &str = include_str!("../assets/ws63/ws63_acore_wifi_patches.txt");
/// Provenance and digest manifest generated with the bundled artifacts.
pub const MANIFEST: &str = include_str!("../assets/ws63/manifest.txt");

#[cfg(test)]
mod tests {
    use super::*;

    fn data_lines(input: &str) -> impl Iterator<Item = &str> {
        input
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#') && !line.starts_with('*'))
    }

    #[test]
    fn generated_counts_match_contract() {
        assert_eq!(
            data_lines(ROM_SYMBOLS_LD)
                .filter(|line| line.contains(" = 0x"))
                .count(),
            SYMBOL_COUNT
        );
        assert_eq!(data_lines(ROM_CALLBACKS).count(), CALLBACK_COUNT);
        assert_eq!(data_lines(WIFI_PATCHES).count(), WIFI_PATCH_COUNT);
    }

    #[test]
    fn mandatory_patch_symbols_exist_in_rom_table() {
        for patch in data_lines(WIFI_PATCHES) {
            assert!(
                ROM_SYMBOLS_LD.lines().any(|line| {
                    line.trim_start().starts_with(patch)
                        && line[patch.len()..].trim_start().starts_with('=')
                }),
                "missing ROM symbol {patch}"
            );
        }
    }
}
