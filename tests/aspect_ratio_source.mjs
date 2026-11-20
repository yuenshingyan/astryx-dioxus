// Offline pinned-source verification and declaration lowering, NOT captured release output.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
const base = new URL('./fixtures/aspect_ratio_source/', import.meta.url);
const manifest = JSON.parse(await readFile(new URL('manifest.json', base), 'utf8'));
assert.equal(manifest.pin, '476a3153417dbf75c565f7df05b0784e94ddd59e');
for (const [path, hash] of Object.entries(manifest.files)) {
  const bytes = await readFile(new URL(path, base));
  assert.equal(createHash('sha1').update(`blob ${bytes.length}\0`).update(bytes).digest('hex'), hash, path);
}
const source = await readFile(new URL('packages/core/src/AspectRatio/AspectRatio.tsx', base), 'utf8');
const kebab = name => name.replace(/[A-Z]/g, c => `-${c.toLowerCase()}`);
function declarations(name) {
  const block = source.split(`  ${name}: {`)[1]?.split('\n  },')[0];
  assert.ok(block, `source style ${name}`);
  return [...block.matchAll(/^\s+(\w+): ('[^']*'|\d+),/gm)].map(([,key,value]) => `${kebab(key)}:${value.replace(/^'|'$/g,'')}`).join(';');
}
export const referenceCss = `@layer astryx-base {
.ref-ratio { ${declarations('container')}; aspect-ratio:var(--astryx-aspect-ratio); }
.ref-ratio[data-shape="ellipse"] { ${declarations('ellipse')} }
.ref-content { ${declarations('child')} }
.ref-content[data-astryx-aspect-ratio-override="center"] { ${declarations('childCenter')} }
.ref-content:is([data-astryx-aspect-ratio-override="cover"],[data-astryx-aspect-ratio-override="contain"]) > * { ${declarations('childFill')} }
.ref-content[data-astryx-aspect-ratio-override="cover"] > :is(img,video) { ${declarations('childCover')} }
.ref-content[data-astryx-aspect-ratio-override="contain"] > :is(img,video) { ${declarations('childContain')} }
}`;
assert.ok(referenceCss.includes('width:100%;height:100%'));
for (const [name, snippets] of Object.entries({
  Showcase:['height: 120', "width: 'auto'", 'ratio: 4 / 3', 'ratio: 16 / 9'],
  Widescreen:['width={600}','ratio={16 / 9}'],
  SquareImage:['width={300}','ratio={1}'],
  CircleImage:['width={300}','shape="ellipse"','ratio={1}'],
  ImageGallery:['width={600}','columns={3} gap={4}','ratio={4 / 3}'],
  WithSkeleton:['width={600}','ratio={16 / 9}','<Skeleton width="100%" height="100%"'],
})) {
  const template=await readFile(new URL(`packages/cli/assets/templates/blocks/components/AspectRatio/AspectRatio${name}.tsx`,base),'utf8');
  for(const snippet of snippets) assert.ok(template.includes(snippet), `${name}: ${snippet}`);
}
console.log(`AspectRatio: ${Object.keys(manifest.files).length} Git blobs verified; six pinned patterns verified (source-derived, not release measurements).`);
