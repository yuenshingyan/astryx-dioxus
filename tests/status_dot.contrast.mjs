// Conditional essential-mark/glyph audit, not a color-only-status certificate.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {contrast,createColorResolver,over} from './contrast/colors.mjs';
const source=JSON.parse(await readFile(new URL('./fixtures/status-dot-neutral-reference.json',import.meta.url),'utf8'));
const report={revision:source.pinnedCommit,scope:'Core/Neutral plate vs source body/card/surface and optional currentColor glyph vs plate, opacity 1 and original pulse minimum .5. Group opacity composites BOTH painted pixels directly over the parent background. Conditional 3:1 essential-mark/glyph applicability; adjacent visible labels can make a dot decorative. No antialiasing, color-distinguishability, arbitrary custom theme or forced-system-palette certificate.',cases:0,resting:{plateFailures:[],glyphFailures:[]},pulseMinimum:{plateFailures:[],glyphFailures:[]}};
assert.equal(contrast([0,0,0,1],[1,1,1,1]),21);
for(const [theme,palette] of Object.entries(source.pinnedContract.palettes))for(const [mode,tokens] of Object.entries(palette.resolved)){
 const resolve=createColorResolver(tokens,mode==='dark'?1:0);
 for(const variant of Object.keys(source.pinnedContract.plateTokens))for(const backgroundName of ['body','card','surface'])for(const opacity of [1,.5]){
  const background=resolve(`--color-background-${backgroundName}`);
  const plate=resolve(source.pinnedContract.plateTokens[variant]),ink=resolve(source.pinnedContract.inkTokens[variant]);
  const compositePlate=over([...plate.slice(0,3),opacity],background),compositeInk=over([...ink.slice(0,3),opacity],background);
  const plateRatio=contrast(compositePlate,background),glyphRatio=contrast(compositeInk,compositePlate);
  const bucket=opacity===1?report.resting:report.pulseMinimum,identity={theme,mode,variant,background:backgroundName};
  report.cases++;
  if(plateRatio<3)bucket.plateFailures.push({...identity,ratio:+plateRatio.toFixed(4)});
  if(glyphRatio<3)bucket.glyphFailures.push({...identity,ratio:+glyphRatio.toFixed(4)});
 }
}
assert.equal(report.cases,120);
if(process.argv.includes('--json'))console.log(JSON.stringify(report,null,2));else{
 console.log(`StatusDot resting: ${report.resting.plateFailures.length}/60 conditional plate and ${report.resting.glyphFailures.length}/60 optional-glyph cases below 3:1.`);
 console.log(`Original pulse minimum (.5): ${report.pulseMinimum.plateFailures.length}/60 conditional plate and ${report.pulseMinimum.glyphFailures.length}/60 optional-glyph cases below 3:1. Pulse/colors preserved; approved custom fallback and forced-color outlines are separate checks.`);
}
if(Object.values(report.resting).some(v=>v.length)||Object.values(report.pulseMinimum).some(v=>v.length))process.exitCode=1;
