#!/usr/bin/env python3
"""Offline integrity/identity assertions for captured original Stack evidence."""
import hashlib
import json
from pathlib import Path

root = Path(__file__).parent / 'fixtures'
fixture = json.loads((root / 'stack-published.json').read_text())
assert fixture['url'] == 'https://astryx.atmeta.com/components/Stack'
assert fixture['title'] == 'Stack · Astryx'
assert fixture['version'] == '@astryxdesign/core v0.7.16'
assert fixture['viewport'] == [1200, 1600, 1]
assert fixture['rootFont'] == '16px'
assert 'deployment commit unknown' in fixture['provenance']['limits'].lower()
assert set(fixture['cases']) == {'alignment', 'directions', 'fill'}
for name, case in fixture['cases'].items():
    image = root / case['screenshot']['path']
    assert hashlib.sha256(image.read_bytes()).hexdigest() == case['screenshot']['sha256']
    assert '<script' not in case['html'].lower()
    assert case['nodes'][0]['box'] == [0, 0, *case['rootSize']]
    assert case['nodes'][0]['targets'] == ['astryx-stack']
    assert case['nodes'][0]['style']['backgroundColor'] == 'rgba(0, 0, 0, 0)'
assert fixture['cases']['directions']['nodes'][0]['data']['wrap'] == 'wrap'
tokens = [n for n in fixture['cases']['directions']['nodes'] if 'astryx-token' in n['targets']]
assert len(tokens) == 6
assert all(n['box'][3] == 24 and n['style']['borderRadius'] == '6px' for n in tokens)
assert 'useToast' in fixture['cases']['fill']['publishedCode']
assert 'StackAlignment' in fixture['cases']['alignment']['publishedCode']
for resource in fixture['resources']:
    assert resource['status'] == 200 and resource['bytes'] > 0
    assert len(resource['sha256']) == 64
print('Published Stack identity, three screenshot digests, observed geometry/palette metadata verified offline')
