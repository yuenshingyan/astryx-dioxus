"""Offline causal raster controls; retains both historical capture directories."""
import hashlib
import json

from stack_patterns import ROOT, compare, difference

controls_dir = ROOT / "stack-raster-controls"
controls = json.loads((controls_dir / "measurements.json").read_text())
report = {
    "localFontSha256": hashlib.sha256((ROOT.parents[1] / "assets/fonts/Figtree-latin.woff2").read_bytes()).hexdigest(),
    "limits": "No original font binary or deployed commit was captured. Remaining <=1 channel-level differences are unclassified quantization-scale residuals, not proven pixel identity or a source paint defect.",
    "cases": {},
}
for name, entry in controls["cases"].items():
    paths = {}
    for mode in ("transform", "layout", "repeat"):
        shot = entry[mode]["screenshot"]
        paths[mode] = controls_dir / shot["path"]
        assert hashlib.sha256(paths[mode].read_bytes()).hexdigest() == shot["sha256"]
        assert entry[mode]["metrics"] == entry["transform"]["metrics"]
    comparisons = {
        "historicalVsTransform": difference(ROOT / f"stack-actual/{name}.png", paths["transform"]),
        "canonicalVsLayout": difference(ROOT / f"stack-raster/{name}.png", paths["layout"]),
        "layoutVsRepeat": difference(paths["layout"], paths["repeat"]),
        "transformVsLayout": difference(paths["transform"], paths["layout"]),
        "publishedVsLayout": difference(ROOT / f"stack-published/{name}.png", paths["layout"]),
    }
    for control in ("historicalVsTransform", "canonicalVsLayout", "layoutVsRepeat"):
        assert comparisons[control]["maxChannelError"] == 0, (name, control)
    assert comparisons["transformVsLayout"]["maxChannelError"] > 1
    # A new strict regression bound, not a widened historical comparator tolerance.
    assert comparisons["publishedVsLayout"]["maxChannelError"] <= 1
    report["cases"][name] = comparisons
    print(name, "historical reproduction, unchanged geometry, canonical equality and repeat controls passed")
compare("stack-actual")
compare("stack-raster")
(controls_dir / "comparison.json").write_text(json.dumps(report, indent=2) + "\n")
