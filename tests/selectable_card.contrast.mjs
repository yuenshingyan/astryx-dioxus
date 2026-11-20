// Conditional essential-selection audit. Source colors are not redesigned.
import {readFile} from 'node:fs/promises';
import {createColorResolver,over,contrast} from './contrast/colors.mjs';
const fixture=JSON.parse(await readFile(new URL('./fixtures/card-neutral-reference.json',import.meta.url),'utf8'));
const variants=['default','transparent','muted','blue','cyan','gray','green','orange','pink','purple','red','teal','yellow'];
const findings=[];let count=0,minimum=Infinity;
for(const theme of ['core','neutral'])for(const [mode,name] of ['light','dark'].entries()){
 const tokens=Object.fromEntries(Object.entries(fixture.pinnedTokens[theme]).map(([k,v])=>[k,Array.isArray(v)?v[mode]:v]));const color=createColorResolver(tokens,mode);
 for(const surface of ['body','card','surface'])for(const variant of variants){
  const under=color('--color-background-'+surface),fill=variant==='transparent'?[0,0,0,0]:color('--color-background-'+(variant==='default'?'card':variant)),paint=over(fill,under),ring=color(['default','transparent','muted'].includes(variant)?'--color-accent':'--color-border-'+variant);
  for(const state of ['rest','hover','pressed']){
   const inside=state==='rest'?paint:over(color('--color-overlay-'+state),paint),visible=state==='rest'?over(ring,paint):over(color('--color-overlay-'+state),over(ring,paint));
   const ratio=contrast(visible,inside);count++;minimum=Math.min(minimum,ratio);if(ratio<3)findings.push({theme,mode:name,surface,variant,state,ratio});
  }
 }
}
console.log(`SelectableCard persistent ring: ${findings.length}/${count} conditional essential-selection findings below 3:1; minimum ${minimum.toFixed(3)}. Custom theme contrast remains caller-owned; Card text has a separate audit. Forced colors removes the box-shadow indicator on borderless variants; no treatment or approval is invented.`);
if(findings.length){console.log(JSON.stringify(findings,null,2));process.exitCode=1;}
