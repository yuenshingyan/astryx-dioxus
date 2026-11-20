// Verified offline source extraction, not catalog/release measurements.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {runInNewContext} from 'node:vm';
const base=new URL('./fixtures/blockquote-upstream/',import.meta.url);
export const manifest=JSON.parse(await readFile(new URL('manifest.json',base),'utf8'));
const sources={};
for(const entry of manifest.files){
 const data=await readFile(new URL(entry.file,base));
 assert.equal(createHash('sha1').update(`blob ${data.length}\0`).update(data).digest('hex'),entry.blob,entry.path);
 sources[entry.path]=data.toString();
}
const get=p=>{assert.ok(sources[p],p);return sources[p]};
const component=get('packages/core/src/Blockquote/Blockquote.tsx');
const kebab=s=>s.replace(/[A-Z]/g,c=>'-'+c.toLowerCase());
function extractStyles(name){
 const block=component.match(new RegExp(`  ${name}: \\{([\\s\\S]*?)\\n  \\},`))?.[1];assert.ok(block,name);
 return [...block.matchAll(/(\w+): (?:\w+Vars\['([^']+)'\]|'([^']+)'|(0)),/g)].map(([,key,token,value,zero])=>`${kebab(key)}:${token?'var('+token+')':value??zero}`).join(';');
}
export const quoteCSS=extractStyles('root'),citeCSS=extractStyles('cite');
assert.equal(quoteCSS.split(';').length,10);assert.equal(citeCSS.split(';').length,5);
assert.match(component,/isRenderable\(cite\)/);assert.match(component,/<cite /);assert.doesNotMatch(component.replace(/\/\*[\s\S]*?\*\//g,''),/<footer[ >]/);
const normalize=s=>s.replace(/\s+/g,' ').trim();
export const patterns=['Showcase','WithCite','Testimonials'].map(name=>{
 const src=get(`packages/cli/assets/templates/blocks/components/Blockquote/Blockquote${name}.tsx`);
 const quotes=[...src.matchAll(/<Blockquote(?: cite="([^"]*)")?>([\s\S]*?)<\/Blockquote>/g)].map(([,cite,text])=>({text:normalize(text),cite:cite??null}));
 assert.equal(quotes.length,name==='Testimonials'?3:2);
 if(name==='Testimonials'){
  assert.match(src,/<Grid columns=\{2\} gap=\{3\} style=\{\{maxWidth: 600\}\}>/);
  assert.equal((src.match(/<Card padding=\{4\}>/g)??[]).length,3);
  assert.match(src,/<GridSpan columns=\{2\}>/);
 }else assert.match(src,/gap=\{4\}/);
 return {name,quotes};
});
const tokens=get('packages/core/src/theme/tokens.stylex.ts');
const tokenValue=name=>{const v=tokens.match(new RegExp(`'${name}': '([^']+)'`))?.[1];assert.ok(v,name);return v};
export const coreTokens=Object.fromEntries(['--spacing-0-5','--spacing-2','--spacing-3','--spacing-4','--color-text-secondary','--color-border-emphasized','--text-supporting-size','--text-supporting-leading','--font-size-sm','--color-background-body','--color-background-surface','--color-background-card','--color-border','--border-width','--radius-container'].map(k=>[k,tokenValue(k)]));
const neutral=get('packages/themes/neutral/src/neutralTheme.ts');
const paletteSource=get('packages/themes/neutral/src/neutralPaletteRefs.generated.ts');
const palette=runInNewContext('('+paletteSource.split('export const neutralPaletteRefs = ')[1].split(' as const;')[0]+')');
export const neutralTokens={...coreTokens};
for(const name of ['--color-text-secondary','--color-border-emphasized','--color-background-body','--color-background-surface','--color-background-card']){
 const match=neutral.match(new RegExp(`'${name}': \\[neutral.light\\[(\\d+)\\], neutral.dark\\[(\\d+)\\]\\]`));assert.ok(match,name);
 neutralTokens[name]=`light-dark(${palette.neutral.light[match[1]]}, ${palette.neutral.dark[match[2]]})`;
}
assert.match(neutral,/'--color-border': \[\s*withAlpha\(neutral.light\[0\], '14'\),\s*withAlpha\(neutral.dark\[100\], '1A'\)/);
neutralTokens['--color-border']='light-dark(#00000014, #ffffff1A)';
for(const name of ['--radius-container','--border-width']){
 const v=neutral.match(new RegExp(`'${name}': '([^']+)'`))?.[1];if(v)neutralTokens[name]=v;
}
// Testimonial framing is restricted to the source's default, unelevated Card.
// This is a source-derived native fixture, not execution of React/StyleX.
const card=get('packages/core/src/Card/Card.tsx');
for(const re of [/minWidth: 0/,/overflow: 'clip'/,/borderStyle: 'solid'/,/colorVars\['--color-background-card'\]/,/borderVars\['--border-width'\]/])assert.match(card,re);
export const cardCSS='min-width:0;overflow:clip;border-radius:var(--radius-container);border:var(--border-width) solid var(--color-border);background:var(--color-background-card);padding:calc(var(--spacing-4) - var(--border-width));box-shadow:0 0 transparent,0 0 transparent';
console.log(`Blockquote source check passed: ${manifest.files.length} Git blobs, pinned style/token extraction, seven quotes/three templates.`);
