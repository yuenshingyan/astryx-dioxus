"""Assemble the public stylesheet without changing rule order or source bytes."""

import argparse
from pathlib import Path


ASSETS = Path(__file__).resolve().parents[1] / "assets"
SOURCE = ASSETS / "styles"
BUNDLE = ASSETS / "astryx.css"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="fail if the bundle is stale")
    args = parser.parse_args()

    entries = [line.strip() for line in (SOURCE / "order.txt").read_text(encoding="utf-8").splitlines()]
    entries = [line for line in entries if line and not line.startswith("#")]
    files = {path.relative_to(SOURCE).as_posix() for path in SOURCE.rglob("*.css")}
    if len(entries) != len(set(entries)) or set(entries) != files:
        parser.error("assets/styles/order.txt must list every CSS source exactly once")

    contents = b"".join((SOURCE / entry).read_bytes() for entry in entries)
    if args.check:
        if not BUNDLE.is_file() or BUNDLE.read_bytes() != contents:
            parser.exit(1, "assets/astryx.css is stale; run python3 scripts/bundle_styles.py\n")
    else:
        BUNDLE.write_bytes(contents)


if __name__ == "__main__":
    main()
