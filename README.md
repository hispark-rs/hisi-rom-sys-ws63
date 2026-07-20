# hisi-rom-sys-ws63

Generated, `no_std` WS63 application-core mask-ROM symbol, callback ABI, and
Wi-Fi patch metadata. This is the chip-coupled backend for the chip-neutral
[`hisi-rom-sys`](https://crates.io/crates/hisi-rom-sys) facade.

The published metadata also includes the public security-ROM entry points and
PKE instruction-ROM facts consumed by the fallible WS63 crypto backend. ROM
entry points that require private ROM RAM state remain intentionally hidden.

The canonical language-neutral input is `ws63-RF/rom`; `tools/sync-ws63-rom.py`
normalizes it and records provenance hashes. This crate does not own HAL
drivers, image formats, runtime patch controllers, or high-level radio APIs.
