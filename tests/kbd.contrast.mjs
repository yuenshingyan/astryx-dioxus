// Uniform keycap paint; platform glyphs/shortcut strings do not change the pair.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {contrast,createColorResolver,over} from './contrast/colors.mjs';
const core=JSON.parse(await readFile(new URL('./fixtures/button-core-palette.json',import.meta.url),'utf8'));
const source=JSON.parse(await readFile(new URL('./fixtures/kbd-neutral-reference.json',import.meta.url),'utf8'));
const neutral={...core.tokens,...source.profiles.tokens[0],'--color-background-body':'light-dark(#fff,#1b1b1b)','--color-background-surface':'light-dark(#fff,#262626)','--color-background-muted':'light-dark(#f1f1f1,#1b1b1b)'};
const report={revision:core.revision,scope:'Core and showcase Neutral keycap text on composited page/surface/muted fills, light/dark. No interactive focus/boundary requirement is invented for display-only key notation. Platform glyphs share paint. Custom themes, antialiasing and arbitrary parent/control backgrounds are excluded.',textCases:0,textFailures:[]};
assert.equal(contrast([0,0,0,1],[1,1,1,1]),21);
for(const [theme,tokens] of [['core',core.tokens],['neutral',neutral]])for(const [index,mode] of ['light','dark'].entries()){
 const resolve=createColorResolver(tokens,index);
 for(const canvas of ['--color-background-body','--color-background-surface','--color-background-muted']){
  const page=resolve('--color-background-body'),background=over(resolve(canvas),page),keycap=over(resolve('--color-neutral'),background),ratio=contrast(over(resolve('--color-text-primary'),keycap),keycap);
  report.textCases++;if(ratio<4.5)report.textFailures.push({theme,mode,canvas,ratio:+ratio.toFixed(4)});
 }
}
assert.equal(report.textCases,12);
if(process.argv.includes('--json'))console.log(JSON.stringify(report,null,2));else console.log(`Kbd core/Neutral keycap text: ${report.textFailures.length}/12 modeled cases below 4.5:1; no shortcut activation or focus target is implied.`);
if(report.textFailures.length)process.exitCode=1;
