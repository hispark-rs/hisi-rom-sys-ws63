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

/// WS63 security-ROM entry points and PKE instruction-ROM facts.
///
/// These addresses are transcribed from the Apache-2.0
/// `security_unified/rom_drv_api/security_rom_table.h` and WS63 porting header
/// in the matching vendor SDK. They are kept here so consumers do not scatter
/// chip ROM addresses through protocol or driver code.
pub mod security {
    /// Initialize the security-ROM PKE HAL state.
    pub const HAL_PKE_INIT: usize = 0x0010_3424;
    /// Release the security-ROM PKE HAL state.
    pub const HAL_PKE_DEINIT: usize = 0x0010_3438;
    /// Acquire the hardware PKE owner lock.
    pub const HAL_PKE_LOCK: usize = 0x0010_3490;
    /// Release the hardware PKE owner lock.
    pub const HAL_PKE_UNLOCK: usize = 0x0010_3508;
    /// Enable PKE power-analysis noise generation.
    pub const HAL_PKE_ENABLE_NOISE: usize = 0x0010_351e;
    /// Disable PKE power-analysis noise generation.
    pub const HAL_PKE_DISABLE_NOISE: usize = 0x0010_3532;
    /// Load one masked operand into PKE data RAM.
    pub const HAL_PKE_SET_RAM: usize = 0x0010_359c;
    /// Read one masked operand from PKE data RAM.
    pub const HAL_PKE_GET_RAM: usize = 0x0010_364a;
    /// Clear PKE data RAM.
    pub const HAL_PKE_CLEAN_RAM: usize = 0x0010_3692;
    /// Configure one PKE single/batch instruction.
    pub const HAL_PKE_SET_MODE: usize = 0x0010_36a8;
    /// Start the configured PKE instruction.
    pub const HAL_PKE_START: usize = 0x0010_371a;
    /// Wait for completion and return the hardware status.
    pub const HAL_PKE_WAIT_DONE: usize = 0x0010_3738;
    /// Program the Montgomery reduction parameter.
    pub const HAL_PKE_SET_MONT_PARA: usize = 0x0010_37bc;
    /// Load one curve's Montgomery constants into PKE data RAM.
    pub const HAL_PKE_SET_ECC_PARAM: usize = 0x0010_37d0;

    /// Start of the WS63 PKE instruction ROM consumed by batch operations.
    pub const PKE_INSTRUCTION_ROM_START: usize = 0x00c0_0000;
}

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

    #[test]
    fn security_rom_entry_points_are_halfword_aligned() {
        for address in [
            security::HAL_PKE_INIT,
            security::HAL_PKE_DEINIT,
            security::HAL_PKE_LOCK,
            security::HAL_PKE_UNLOCK,
            security::HAL_PKE_ENABLE_NOISE,
            security::HAL_PKE_DISABLE_NOISE,
            security::HAL_PKE_SET_RAM,
            security::HAL_PKE_GET_RAM,
            security::HAL_PKE_CLEAN_RAM,
            security::HAL_PKE_SET_MODE,
            security::HAL_PKE_START,
            security::HAL_PKE_WAIT_DONE,
            security::HAL_PKE_SET_MONT_PARA,
            security::HAL_PKE_SET_ECC_PARAM,
        ] {
            assert_eq!(address & 1, 0);
        }
        assert_eq!(security::PKE_INSTRUCTION_ROM_START, 0x00c0_0000);
    }
}
