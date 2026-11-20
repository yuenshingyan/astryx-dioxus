"""Record, not waive, RGB raster differences. No pixel-parity assertion.
Run after stack_toast.browser.mjs; optional argument is its evidence directory.
The stdlib RGB decoder follows the existing stack_patterns.py helper without
executing that script or changing any static-pattern fixture.
"""
import hashlib
import json
from pathlib import Path
import struct
import sys
import zlib


def png(path):
    data = path.read_bytes()
    assert data[:8] == b'\x89PNG\r\n\x1a\n'
    offset, compressed = 8, b''
    while offset < len(data):
        size = struct.unpack('>I', data[offset:offset + 4])[0]
        kind, body = data[offset + 4:offset + 8], data[offset + 8:offset + 8 + size]
        offset += size + 12
        if kind == b'IHDR':
            width, height, depth, color, compression, filtering, interlace = struct.unpack('>IIBBBBB', body)
            assert (depth, color, compression, filtering, interlace) == (8, 2, 0, 0, 0)
        if kind == b'IDAT':
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


actual_dir = Path(sys.argv[1] if len(sys.argv) > 1 else '/tmp/astryx-parity/toast-actual')
original_dir = Path(__file__).parent / 'fixtures/stack-toast'
original = json.loads((original_dir / 'published.json').read_text())
report = {}
for action in original['actions']:
    name = action['label'].lower()
    paths = [original_dir / action['screenshot']['path'], actual_dir / (name + '.png')]
    hashes = [hashlib.sha256(path.read_bytes()).hexdigest() for path in paths]
    assert hashes[0] == action['screenshot']['sha256']
    a, b = map(png, paths)
    assert a[:2] == b[:2]
    errors = [abs(x-y) for x, y in zip(a[2], b[2])]
    report[name] = {
        'size': a[:2], 'sha256': hashes,
        'meanAbsoluteChannelError': sum(errors)/len(errors),
        'maxChannelError': max(errors),
        'differentPixelFraction': sum(a[2][i:i+3] != b[2][i:i+3] for i in range(0, len(errors), 3))/(a[0]*a[1]),
    }
    print(name, json.dumps(report[name]))
(actual_dir / 'pixel-comparison.json').write_text(json.dumps(report, indent=2) + '\n')
