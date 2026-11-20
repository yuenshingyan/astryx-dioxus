// Source palette audit: overlay covers ink as well as the Card fill.
import {readFile} from 'node:fs/promises';
import {createColorResolver,over,contrast} from './contrast/colors.mjs';
const fixture=JSON.parse(await readFile(new URL('./fixtures/card-neutral-reference.json',import.meta.url),'utf8'));
const variants=['default','transparent','muted','blue','cyan','gray','green','orange','pink','purple','red','teal','yellow','custom'];
const corrected=new Set(['blue','cyan','green','orange','pink','purple','red','teal','yellow']);
const findings={activeText:[],disabledTextAdvisory:[],focus:[]};
const counts={activeText:0,disabledTextAdvisory:0,focus:0};
for(const theme of ['core','neutral'])for(const [index,mode]of ['light','dark'].entries()){
 const tokens=Object.fromEntries(Object.entries(fixture.pinnedTokens[theme]).map(([key,value])=>[key,Array.isArray(value)?value[index]:value]));const color=createColorResolver(tokens,index);
 for(const backdrop of ['body','card','surface'])for(const variant of variants){
  const under=color(`--color-background-${backdrop}`);const fill=variant==='transparent'||variant==='custom'?[0,0,0,0]:color(`--color-background-${variant==='default'?'card':variant}`);const base=over(fill,under);
  for(const state of ['rest','hover','pressed','disabled']){
   const overlay=state==='hover'?color('--color-overlay-hover'):state==='pressed'?color('--color-overlay-pressed'):[0,0,0,0];
   let paint=over(overlay,base);if(state==='disabled')paint=over([...paint.slice(0,3),.5],under);
   for(const role of ['primary','secondary']){
    const ink=theme==='neutral'&&corrected.has(variant)&&role==='secondary'?color(index===0?'#474747':'#b9b9b9'):color(`--color-text-${role}`);
    let text=over(overlay,over(ink,base));if(state==='disabled')text=over([...text.slice(0,3),.5],under);
    const ratio=contrast(text,paint),category=state==='disabled'?'disabledTextAdvisory':'activeText';counts[category]++;if(ratio<4.5)findings[category].push({theme,mode,backdrop,variant,state,role,ratio});
   }
  }
  for(const isDisabled of [false,true]){const ring=color('--focus-outline-color');const paint=over([...ring.slice(0,3),ring[3]*(isDisabled?.5:1)],under);const ratio=contrast(paint,under);counts.focus++;if(ratio<3)findings.focus.push({theme,mode,backdrop,variant,isDisabled,ratio});}
 }
}
for(const category of Object.keys(counts))console.log(`${category}: ${findings[category].length}/${counts[category]} conditional source-palette findings below ${category==='focus'?'3':'4.5'}:1`);
console.log('Disabled text is exempt/informational; focus remains essential while disabled. No threshold invented for decorative category fills, borders or shadows. Custom baseline is transparent, not a guarantee for arbitrary caller themes.');
console.log(JSON.stringify(findings,null,2));
if(findings.activeText.length||findings.focus.length)process.exitCode=1;
