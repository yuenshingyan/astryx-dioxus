// Heading is never inactive merely because its ink is named disabled.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {contrast, createColorResolver} from './contrast/colors.mjs';
const source=JSON.parse(await readFile(new URL('./fixtures/button-core-palette.json',import.meta.url),'utf8'));
assert.equal(source.revision,'476a3153417dbf75c565f7df05b0784e94ddd59e');
const colors={primary:'--color-text-primary',secondary:'--color-text-secondary',placeholder:'--color-text-secondary',disabled:'--color-text-disabled',accent:'--color-text-accent'};
// At 16px root: h1=24px; h2=20px/600 is not WCAG large bold text.
const types={h1:3,h2:4.5,h3:4.5,h4:4.5,h5:4.5,h6:4.5,'display-1':3,'display-2':3,'display-3':3};
const report={revision:source.revision,scope:'Core defaults at 16px root; page/surface in light/dark. Inherit/custom paint, arbitrary overrides and image backdrops unmodeled. No inactive exemption for disabled ink alone.',cases:0,failures:[]};
for(const [index,mode] of ['light','dark'].entries()) {
 const resolve=createColorResolver(source.tokens,index);
 for(const canvas of ['--color-background-body','--color-background-surface']) for(const [type,threshold] of Object.entries(types)) for(const [color,token] of Object.entries(colors)) {
  report.cases++;
  const ratio=contrast(resolve(token),resolve(canvas));
  if(ratio<threshold)report.failures.push({mode,canvas,type,color,threshold,ratio:+ratio.toFixed(4)});
 }
}
assert.equal(report.cases,180);
if(process.argv.includes('--json'))console.log(JSON.stringify(report,null,2));
else console.log(`Heading core source palette: ${report.failures.length}/${report.cases} conditional active-text failures; ${report.failures.filter(x=>x.color==='disabled').length} use disabled ink. Source palette retained.`);
if(report.failures.length)process.exitCode=1;
