// Intended Spinner surfaces; source paint is retained, not a compliance claim.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {contrast,createColorResolver,over} from './contrast/colors.mjs';
const palette=JSON.parse(await readFile(new URL('./fixtures/button-core-palette.json',import.meta.url),'utf8'));
const neutral={...palette.tokens,'--color-accent':'light-dark(#1b1b1b,#f1f1f1)','--color-text-primary':'light-dark(#000,#fff)','--color-text-secondary':'light-dark(#474747,#9e9e9e)','--color-background-body':'light-dark(#fff,#1b1b1b)','--color-background-surface':'light-dark(#fff,#262626)'};
const report={revision:palette.revision,scope:'Core and showcase Neutral: default/subtle/inherit on page and surface, onMedia on its original #1a1a2e panel, light/dark. Inherit assumes primary parent. Arc/backdrop and arc/track are essential 3:1 checks; optional string labels use primary at 4.5:1. Track alone is not essential. All four sizes share paint; custom themes, arbitrary media and rich labels remain caller-owned.',arcCases:0,arcBackdropFailures:[],arcTrackFailures:[],labelCases:0,labelFailures:[]};
assert.equal(contrast([0,0,0,1],[1,1,1,1]),21);
for(const [theme,tokens] of [['core',palette.tokens],['neutral',neutral]])for(const [index,mode] of ['light','dark'].entries()){
 const resolve=createColorResolver(tokens,index);
 for(const shade of ['default','subtle','inherit','onMedia'])for(const canvas of shade==='onMedia'?['#1a1a2e']:['--color-background-body','--color-background-surface']){
  const background=resolve(canvas),arc=resolve(shade==='default'?'--color-accent':shade==='subtle'?'--color-text-secondary':shade==='inherit'?'--color-text-primary':'--color-on-dark');
  const track=shade==='inherit'?over([...arc.slice(0,3),0.3],background):shade==='onMedia'?over([...arc.slice(0,3),77/255],background):resolve('--color-track');
  const info={theme,mode,shade,canvas};report.arcCases++;
  const backdropRatio=contrast(arc,background),trackRatio=contrast(arc,track),labelRatio=contrast(resolve('--color-text-primary'),background);
  if(backdropRatio<3)report.arcBackdropFailures.push({...info,ratio:+backdropRatio.toFixed(4)});
  if(trackRatio<3)report.arcTrackFailures.push({...info,ratio:+trackRatio.toFixed(4)});
  report.labelCases++;
  if(labelRatio<4.5)report.labelFailures.push({...info,ratio:+labelRatio.toFixed(4)});
 }
}
assert.equal(report.arcCases,28);assert.equal(report.labelCases,28);
if(process.argv.includes('--json'))console.log(JSON.stringify(report,null,2));
else console.log(`Spinner source paint: ${report.arcBackdropFailures.length}/28 essential arc/backdrop, ${report.arcTrackFailures.length}/28 arc/track below 3:1; ${report.labelFailures.length}/28 optional string labels below 4.5:1. On-media labels need caller-owned light rich text or a stronger theme on dark panels in light mode.`);
if(report.arcBackdropFailures.length||report.arcTrackFailures.length||report.labelFailures.length)process.exitCode=1;
