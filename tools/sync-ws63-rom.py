#!/usr/bin/env python3
"""Synchronize generated WS63 ROM artifacts from the language-neutral source."""

from __future__ import annotations

import argparse
import hashlib
import tempfile
from pathlib import Path


FILES = (
    "ws63_acore_rom.lds",
    "ws63_acore_rom_callbacks.txt",
    "ws63_acore_wifi_patches.txt",
)


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def data_lines(path: Path) -> list[str]:
    return [
        line.strip()
        for line in path.read_text().splitlines()
        if line.strip()
        and not line.lstrip().startswith(("#", "/*", "*"))
    ]


def render(source: Path, output: Path) -> None:
    output.mkdir(parents=True, exist_ok=True)
    symbols = [line for line in data_lines(source / FILES[0]) if " = 0x" in line]
    callbacks = data_lines(source / FILES[1])
    patches = data_lines(source / FILES[2])
    if len(symbols) != 3_752 or len(callbacks) != 227 or len(patches) != 37:
        raise ValueError("WS63 ROM source counts do not match the reviewed contract")

    generated = {
        FILES[0]: ["/* Generated ROM symbol facts; do not edit. */", *symbols],
        FILES[1]: ["# Generated callback ABI names; do not edit.", *callbacks],
        FILES[2]: ["# Generated Wi-Fi patch names; do not edit.", *patches],
    }
    for name, lines in generated.items():
        (output / name).write_text("\n".join(lines) + "\n")

    manifest = [
        "schema=1",
        "source=ws63-RF/rom",
        *(f"source_sha256.{name}={digest(source / name)}" for name in FILES),
        *(f"generated_sha256.{name}={digest(output / name)}" for name in FILES),
    ]
    (output / "manifest.txt").write_text("\n".join(manifest) + "\n")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    destination = Path(__file__).resolve().parents[1] / "assets/ws63"
    if args.check:
        with tempfile.TemporaryDirectory() as temporary:
            expected = Path(temporary)
            render(args.source, expected)
            for name in (*FILES, "manifest.txt"):
                if (expected / name).read_bytes() != (destination / name).read_bytes():
                    raise SystemExit(f"generated artifact drifted: {name}")
    else:
        render(args.source, destination)


if __name__ == "__main__":
    main()
