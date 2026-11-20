// Source-token text audit; decorative Card edges/shadows have no invented 3:1 rule.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {createColorResolver,over,contrast} from './contrast/colors.mjs';
const fixture=JSON.parse(await readFile(new URL('./fixtures/card-neutral-reference.json',import.meta.url),'utf8'));
const variants=['default','transparent','muted','blue','cyan','gray','green','orange','pink','purple','red','teal','yellow','custom'];
const corrected=new Set(['blue','cyan','green','orange','pink','purple','red','teal','yellow']);
const findings=[];let count=0,minimum=Infinity;
for(const theme of ['core','neutral'])for(const [mode,name] of ['light','dark'].entries()){
 const tokens=Object.fromEntries(Object.entries(fixture.pinnedTokens[theme]).map(([k,v])=>[k,Array.isArray(v)?v[mode]:v]));
 const color=createColorResolver(tokens,mode);
 for(const surface of ['body','card','surface'])for(const variant of variants){
  const under=color(`--color-background-${surface}`),fill=variant==='transparent'||variant==='custom'?[0,0,0,0]:color(`--color-background-${variant==='default'?'card':variant}`),paint=over(fill,under);
  for(const role of ['primary','secondary']){
   const ink=theme==='neutral'&&corrected.has(variant)&&role==='secondary'?color(mode===0?'#474747':'#b9b9b9'):color(`--color-text-${role}`);
   const ratio=contrast(ink,paint);count++;minimum=Math.min(minimum,ratio);if(ratio<4.5)findings.push({theme,mode:name,surface,variant,role,ratio});
  }
 }
}
console.log(`Card text: ${findings.length}/${count} modeled findings below 4.5:1; minimum ${minimum.toFixed(3)}. Source grouping surfaces and resting shadows are decorative, not standalone essential graphics.`);
if(findings.length){console.log(JSON.stringify(findings,null,2));process.exitCode=1;}else assert.ok(minimum>=4.5);
