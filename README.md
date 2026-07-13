# hisi-rom-sys-ws63

Generated, `no_std` WS63 application-core mask-ROM symbol, callback ABI, and
Wi-Fi patch metadata. This is the chip-coupled backend for the chip-neutral
[`hisi-rom-sys`](https://crates.io/crates/hisi-rom-sys) facade.

The canonical language-neutral input is `ws63-RF/rom`; `tools/sync-ws63-rom.py`
normalizes it and records provenance hashes. This crate does not own HAL
drivers, image formats, runtime patch controllers, or high-level radio APIs.
