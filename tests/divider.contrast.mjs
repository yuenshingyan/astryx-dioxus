// Conditional separation-line audit, not a claim that every decorative rule is essential.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {contrast,createColorResolver,over} from './contrast/colors.mjs';
const core=JSON.parse(await readFile(new URL('./fixtures/button-core-palette.json',import.meta.url),'utf8'));
const source=JSON.parse(await readFile(new URL('./fixtures/divider-neutral-reference.json',import.meta.url),'utf8'));
const neutral={...core.tokens,...source.tokens,'--color-background-body':'light-dark(#fff,#1b1b1b)','--color-background-surface':'light-dark(#fff,#262626)'};
const report={revision:core.revision,scope:'Core and showcase Neutral, light/dark, page/surface, subtle/strong. Rule threshold 3:1 applies when the separator is needed to understand grouping; labels use secondary at 4.5:1. Orientation/full bleed do not alter paint. Unmatched custom paint, arbitrary rich labels and custom themes are excluded.',lineCases:0,conditionalLineFindings:[],labelCases:0,labelFailures:[]};
assert.equal(contrast([0,0,0,1],[1,1,1,1]),21);
for(const [theme,tokens] of [['core',core.tokens],['neutral',neutral]])for(const [index,mode] of ['light','dark'].entries()){
 const resolve=createColorResolver(tokens,index);
 for(const canvas of ['--color-background-body','--color-background-surface'])for(const variant of ['subtle','strong']){
  const background=resolve(canvas),paint=over(resolve(variant==='subtle'?'--color-border':'--color-border-emphasized'),background),lineRatio=contrast(paint,background),labelRatio=contrast(resolve('--color-text-secondary'),background),info={theme,mode,canvas,variant};
  report.lineCases++;report.labelCases++;
  if(lineRatio<3)report.conditionalLineFindings.push({...info,ratio:+lineRatio.toFixed(4)});
  if(labelRatio<4.5)report.labelFailures.push({...info,ratio:+labelRatio.toFixed(4)});
 }
}
assert.equal(report.lineCases,16);assert.equal(report.labelCases,16);
if(process.argv.includes('--json'))console.log(JSON.stringify(report,null,2));
else console.log(`Divider source palette: ${report.conditionalLineFindings.length}/16 rules below conditional 3:1; ${report.labelFailures.length}/16 labels below 4.5:1. Preserve source paint; strengthen the theme if rules are essential to understand grouping.`);
if(report.conditionalLineFindings.length||report.labelFailures.length)process.exitCode=1;
