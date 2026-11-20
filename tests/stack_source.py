#!/usr/bin/env python3
"""Verify pinned Git blobs and extract an OFFLINE SOURCE-DERIVED fixture, not published measurements.
Run: python3 tests/stack_source.py [--check] [cache root]
Only stdlib; no catalog, npm, React, StyleX, or live network required.
"""
import hashlib
import json
from pathlib import Path
import re
import sys

PIN = '476a3153417dbf75c565f7df05b0784e94ddd59e'
ROOT = Path(next((arg for arg in sys.argv[1:] if arg != '--check'), '/tmp/astryx-layout-primitives-reference'))
tree = json.loads((ROOT / 'tree.json').read_text())
assert tree['sha'] == PIN and not tree.get('truncated', False)
entries = {entry['path']: entry['sha'] for entry in tree['tree'] if entry['type'] == 'blob'}
paths = [path for path in entries if path.startswith('packages/core/src/Stack/') and '/__' not in path]
paths += [path for path in entries if path.startswith('packages/cli/assets/templates/blocks/components/Stack/')]
paths += ['docs/specs/components/Stack/Stack.spec.md']
paths += ['packages/core/src/Token/Token.tsx', 'packages/core/src/Avatar/Avatar.tsx']
paths += ['packages/core/src/' + path for path in ['Layout/padding.stylex.ts', 'utils/types.ts', 'theme/tokens.stylex.ts', 'utils/themeProps.ts', 'utils/mergeProps.ts', 'BaseProps.ts', 'HStack/HStack.tsx', 'VStack/VStack.tsx']]
sources = {}
for path in sorted(paths):
    file = ROOT / path
    if not file.exists():
        file = ROOT / 'stack/dependencies' / path
    data = file.read_bytes()
    blob = hashlib.sha1(b'blob ' + str(len(data)).encode() + b'\0' + data).hexdigest()
    assert blob == entries[path], f'Git blob mismatch: {path}'
    sources[path] = {'gitBlob': blob, 'text': data.decode()}


def text(suffix):
    return next(value['text'] for path, value in sources.items() if path.endswith(suffix))


def style_map(name, source):
    body = re.search(r'const ' + name + r' = stylex.create\(\{(.*?)\}\);', source, re.S)[1]
    result = {}
    for key, declarations in re.findall(r"([\w.]+|'[^']+'):\s*\{([^{}]*)\}", body):
        css = {}
        for prop, literal, token in re.findall(r"(\w+):\s*(?:'([^']*)'|spacingVars\['([^']*)'\]),?", declarations):
            css[re.sub(r'[A-Z]', lambda match: '-' + match[0].lower(), prop)] = literal or 'var(' + token + ')'
        assert css, (key, declarations)
        result[key.strip("'")] = css
    assert result, name
    return result

style = text('/stack.stylex.ts')
styles = {name: style_map(name, style) for name in ['alignItemsStyles', 'justifyContentStyles', 'directionStyles', 'wrapStyles', 'baseStyles', 'gapStyles']}
padding = text('/padding.stylex.ts')
for name in ['paddingInlineStartStyles', 'paddingInlineEndStyles', 'paddingBlockStartStyles', 'paddingBlockEndStyles']:
    # Exported const uses the same grammar as local const.
    styles[name] = style_map(name, padding)
spacing = dict(re.findall(r"'(--spacing-[\d-]+)': '([^']+)'", text('/tokens.stylex.ts')))
source = text('/Stack.tsx')
for invariant in ["hAlign ?? (direction === 'horizontal' ? justify : align)", "vAlign ?? (direction === 'horizontal' ? align : justify)", 'paddingInlineStart ?? paddingInline ?? padding', 'paddingInlineEnd ?? paddingInline ?? padding', 'paddingBlockStart ?? paddingBlock ?? padding', 'paddingBlockEnd ?? paddingBlock ?? padding', '{...style, ...sizingStyle}', "themeProps('stack', {direction, gap, wrap})"]:
    assert invariant in source, invariant
assert 'containerPadding' not in source
def static_rule(suffix, key):
    """Lower only flat static rules; fail rather than ignore unknown expressions."""
    body = re.search(r'\b' + key + r': \{(.*?)\n  \}', text(suffix), re.S)[1]
    body = re.sub(r'//[^\n]*', '', body)
    declarations = {}
    for prop, value in re.findall(r'([\w-]+):\s*([^\n]+)', body):
        value = value.rstrip().removesuffix(',').removesuffix(' as const')
        value = re.sub(r"\w+Vars\['([^']+)'\]", r'var(\1)', value)
        value = re.sub(r'\$\{(var\([^)]+\))\}', r'\1', value)
        if value[0] in "'\"`":
            value = value[1:-1]
        else:
            assert re.fullmatch(r'\d+(?:\.\d+)?|var\([^)]+\)', value), (prop, value)
        prop = re.sub(r'[A-Z]', lambda match: '-' + match[0].lower(), prop)
        declarations[prop] = value
    assert declarations, (suffix, key)
    return declarations


demo_styles = {
    'tokenBase': static_rule('/Token.tsx', 'base'),
    'tokenLabel': static_rule('/Token.tsx', 'label'),
    'tokenMd': static_rule('/Token.tsx', 'md'),
    'tokenDefault': static_rule('/Token.tsx', 'default'),
    'avatarWrapper': static_rule('/Avatar.tsx', 'wrapper'),
    'avatarContent': static_rule('/Avatar.tsx', 'content'),
    'avatarFallback': static_rule('/Avatar.tsx', 'fallback'),
    'itemReset': static_rule('/stackItem.stylex.ts', 'reset'),
    'itemStatic': static_rule('/stackItem.stylex.ts', 'static'),
    'itemFill': static_rule('/stackItem.stylex.ts', 'fill'),
}
assert "case 'md':\n      return 36;" in text('/Avatar.tsx')
assert 'const INITIALS_FONT_SIZE_RATIO = 0.4;' in text('/Avatar.tsx')
fixture = {
    'kind': 'source-derived declarations and verified originals; NOT published or React/StyleX browser measurements',
    'pin': PIN,
    'release': 'No release tag/version equivalence asserted',
    'sources': sources,
    'styles': styles,
    'spacing': spacing,
    'demoStyles': demo_styles,
}
output = Path('tests/fixtures/stack-source-reference.json')
serialized = json.dumps(fixture, indent=2, ensure_ascii=False) + '\n'
if '--check' in sys.argv:
    assert output.read_text() == serialized, 'Fixture changed; regenerate explicitly'
else:
    output.write_text(serialized)
print(f'Verified {len(sources)} pinned Git blobs; source-derived fixture ' + ('matches' if '--check' in sys.argv else 'written'))
