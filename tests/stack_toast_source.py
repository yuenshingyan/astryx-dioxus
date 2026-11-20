#!/usr/bin/env python3
"""Verify the private Toast slice's sources against the existing full pinned tree.
No network. Regenerate: python3 tests/stack_toast_source.py
Check: python3 tests/stack_toast_source.py --check
"""
import hashlib
import json
from pathlib import Path
import sys

PIN = '476a3153417dbf75c565f7df05b0784e94ddd59e'
TREE = Path('/tmp/astryx-layout-primitives-reference/tree.json')
ROOT = Path('/tmp/astryx-parity/toast-reference')
FIXTURE = Path('tests/fixtures/stack-toast/source.json')
PATHS = [
    'docs/specs/components/Toast/Toast.spec.md',
    'packages/core/src/Toast/Toast.tsx',
    'packages/core/src/Toast/ToastViewport.tsx',
    'packages/core/src/Toast/useToast.tsx',
    'packages/core/src/Toast/useToastGesture.ts',
    'packages/core/src/theme/MediaTheme.tsx',
    'packages/core/src/theme/onMediaTokens.ts',
    'packages/core/src/theme/tokens.stylex.ts',
    'packages/themes/neutral/src/neutralTheme.ts',
    'packages/themes/neutral/src/neutralPalettes.generated.ts',
    'packages/cli/assets/templates/blocks/components/Stack/StackAlignment.tsx',
    'packages/cli/assets/templates/blocks/components/Stack/StackFillItem.tsx',
]

def blob(data):
    return hashlib.sha1(b'blob ' + str(len(data)).encode() + b'\0' + data).hexdigest()

if '--check' not in sys.argv:
    tree = json.loads(TREE.read_text())
    assert tree['sha'] == PIN and not tree.get('truncated')
    hashes = {e['path']: e['sha'] for e in tree['tree'] if e['type'] == 'blob'}
    sources = {}
    for path in PATHS:
        data = (ROOT / path).read_bytes()
        assert blob(data) == hashes[path], path
        sources[path] = {'gitBlob': hashes[path], 'text': data.decode()}
    FIXTURE.write_text(json.dumps({'pin': PIN, 'sources': sources}, indent=2) + '\n')

fixture = json.loads(FIXTURE.read_text())
assert fixture['pin'] == PIN and sorted(fixture['sources']) == sorted(PATHS)
for path, source in fixture['sources'].items():
    assert blob(source['text'].encode()) == source['gitBlob'], path
if TREE.exists():
    tree = json.loads(TREE.read_text())
    assert tree['sha'] == PIN and not tree.get('truncated')
    hashes = {e['path']: e['sha'] for e in tree['tree'] if e['type'] == 'blob'}
    for path, source in fixture['sources'].items():
        assert source['gitBlob'] == hashes[path], path
    print('Verified 12 Git blobs against existing full pinned tree')
else:
    print('Verified 12 fixture blob digests; full reference tree unavailable (not reverified)')
source = fixture['sources']
assert 'onClick' not in source[PATHS[-1]]['text']
assert 'onClick' not in source[PATHS[-2]]['text']
published = json.loads(Path('tests/fixtures/stack-toast/published.json').read_text())
assert published['sourcePin'] == PIN and published['deploymentCommit'] == 'unknown'
assert published['title'] == 'Stack · Astryx' and len(published['actions']) == 3
for action in published['actions']:
    screenshot = action['screenshot']
    data = (FIXTURE.parent / screenshot['path']).read_bytes()
    assert hashlib.sha256(data).hexdigest() == screenshot['sha256']
for action, message in zip(published['actions'], [
    'Demo: your edits would be discarded here',
    'Demo: your changes would be saved here',
    'Demo: this profile would open here',
]):
    assert action['body'] == message
    assert action['clip']['width'] == 400 and action['clip']['height'] == 52
assert 'useToast' in published['StackAlignment'] and 'useToast' in published['StackFillItem']
print('Published actions differ from pinned inert Stack templates; deployment commit unknown')
