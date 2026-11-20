"""Localize Stack's archived one-level raster residuals without rewriting captures."""
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path

from stack_patterns import ROOT, png


def verified_png(directory, shot):
    path = directory / shot["path"]
    assert hashlib.sha256(path.read_bytes()).hexdigest() == shot["sha256"], path
    return png(path)


def contains(box, x, y):
    left, top, width, height = box
    return left <= x < left + width and top <= y < top + height


def neighborhood(image, x, y, width, height):
    return {
        image[3 * (row * width + column)]
        for row in range(max(0, y - 1), min(height, y + 2))
        for column in range(max(0, x - 1), min(width, x + 2))
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("capture", nargs="?", type=Path, default=ROOT / "stack-raster")
    args = parser.parse_args()
    published = json.loads((ROOT / "stack-published.json").read_text())
    actual = json.loads((args.capture / "measurements.json").read_text())
    assert actual["browser"] == published["browser"]
    assert actual["viewport"] == published["viewport"]
    assert actual["sourceCapture"] == published["capturedAt"]
    expected = {
        "directions": {},
        "alignment": {"astryx-button__label": 138, "astryx-button": 72},
        "fill": {"astryx-button__label": 138, "astryx-avatar-fallback": 156},
    }
    for name, counts in expected.items():
        reference, rendered = published["cases"][name], actual["cases"][name]
        assert rendered["clip"] == reference["clip"], name
        width, height, original = verified_png(ROOT, reference["screenshot"])
        actual_width, actual_height, image = verified_png(args.capture, rendered["screenshot"])
        assert (width, height) == (actual_width, actual_height), name
        regions = Counter()
        for index in range(0, len(original), 3):
            before, after = original[index:index + 3], image[index:index + 3]
            if before == after:
                continue
            y, x = divmod(index // 3, width)
            assert before == before[:1] * 3 and after == after[:1] * 3, (name, x, y)
            assert abs(before[0] - after[0]) == 1, (name, x, y)
            assert all(len(neighborhood(data, x, y, width, height)) > 1 for data in (original, image)), (name, x, y)
            owner = next((node["targets"][0] for node in reversed(rendered["nodes"])
                          if node["targets"] and contains(node["box"], x + 0.5, y + 0.5)), None)
            regions[owner] += 1
        assert regions == counts, (name, regions, counts)
        print(f"{name}: {sum(regions.values())} differing pixels; {dict(regions)}")


if __name__ == "__main__":
    main()
