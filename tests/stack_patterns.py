"""Offline RGB PNG comparison; no imaging dependency or live assets required."""
import hashlib
import json
from pathlib import Path
import struct
import zlib

ROOT = Path(__file__).parent / "fixtures"


def png(path):
    data = path.read_bytes()
    assert data[:8] == b"\x89PNG\r\n\x1a\n"
    offset, compressed = 8, b""
    while offset < len(data):
        size = struct.unpack(">I", data[offset:offset + 4])[0]
        kind, body = data[offset + 4:offset + 8], data[offset + 8:offset + 8 + size]
        offset += size + 12
        if kind == b"IHDR":
            width, height, depth, color, compression, filtering, interlace = struct.unpack(">IIBBBBB", body)
            assert (depth, color, compression, filtering, interlace) == (8, 2, 0, 0, 0)
        if kind == b"IDAT":
            compressed += body
    raw, result, previous = zlib.decompress(compressed), bytearray(), bytearray(width * 3)
    for y in range(height):
        start = y * (width * 3 + 1)
        mode, row = raw[start], bytearray(raw[start + 1:start + 1 + width * 3])
        for x in range(len(row)):
            left, up, corner = row[x - 3] if x >= 3 else 0, previous[x], previous[x - 3] if x >= 3 else 0
            if mode == 0:
                prediction = 0
            elif mode == 1:
                prediction = left
            elif mode == 2:
                prediction = up
            elif mode == 3:
                prediction = (left + up) // 2
            else:
                assert mode == 4
                p = left + up - corner
                choices = [(abs(p - left), left), (abs(p - up), up), (abs(p - corner), corner)]
                prediction = min(enumerate(choices), key=lambda c: (c[1][0], c[0]))[1][1]
            row[x] = (row[x] + prediction) % 256
        result.extend(row)
        previous = row
    return width, height, result


actual = json.loads((ROOT / "stack-actual/measurements.json").read_text())
original = json.loads((ROOT / "stack-published.json").read_text())
report = {}
for name in ("directions", "alignment", "fill"):
    paths = [ROOT / original["cases"][name]["screenshot"]["path"], ROOT / "stack-actual" / actual["cases"][name]["screenshot"]["path"]]
    for path, expected in zip(paths, [original["cases"][name]["screenshot"], actual["cases"][name]["screenshot"]]):
        assert hashlib.sha256(path.read_bytes()).hexdigest() == expected["sha256"]
    a, b = [png(path) for path in paths]
    assert a[:2] == b[:2], (name, a[:2], b[:2])
    errors = [abs(x - y) for x, y in zip(a[2], b[2])]
    report[name] = {"size": a[:2], "meanAbsoluteChannelError": sum(errors) / len(errors), "maxChannelError": max(errors), "differentPixelFraction": sum(a[2][i:i + 3] != b[2][i:i + 3] for i in range(0, len(errors), 3)) / (a[0] * a[1])}
    # This is a measured raster comparison, not a zero-difference certificate.
    print(name, json.dumps(report[name]))
output = ROOT / "stack-actual/pixel-comparison.json"
output.write_text(json.dumps(report, indent=2) + "\n")
