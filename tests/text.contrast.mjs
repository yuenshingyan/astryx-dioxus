// Source-palette audit: disabled is only a COLOR, not a disabled state.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {contrast, createColorResolver} from './contrast/colors.mjs';
const source = JSON.parse(await readFile(new URL('./fixtures/button-core-palette.json', import.meta.url), 'utf8'));
assert.equal(source.revision,'476a3153417dbf75c565f7df05b0784e94ddd59e');
const colors = {primary:'--color-text-primary',secondary:'--color-text-secondary',placeholder:'--color-text-secondary',disabled:'--color-text-disabled',accent:'--color-text-accent'};
const types = {body:4.5,large:4.5,label:4.5,supporting:4.5,code:4.5,'display-1':3,'display-2':3,'display-3':3};
const report = {revision:source.revision,scope:'Core light/dark page/surface; all concrete Text types/colors. Displays 29–42px normal use 3:1; others use 4.5:1. Inherit/custom themes, arbitrary size/weight overrides and caller backdrops are not modeled. Disabled color is exempt only for genuinely inactive UI; Text itself never disables anything.',cases:0,failures:[]};
for (const [index,mode] of ['light','dark'].entries()) {
  const resolve = createColorResolver(source.tokens,index);
  for (const canvas of ['--color-background-body','--color-background-surface']) for (const [type,threshold] of Object.entries(types)) for (const [color,token] of Object.entries(colors)) {
    report.cases++;
    const ratio = contrast(resolve(token),resolve(canvas));
    if (ratio < threshold) report.failures.push({mode,canvas,type,color,threshold,ratio:+ratio.toFixed(4)});
  }
}
assert.equal(report.cases,160);
if (process.argv.includes('--json')) console.log(JSON.stringify(report,null,2));
else console.log(`Text core source palette: ${report.failures.length}/${report.cases} conditional active-text failures; ${report.failures.filter(x=>x.color==='disabled').length} use disabled ink. Source paint retained; genuinely inactive text has its normal exemption.`);
if (report.failures.length) process.exitCode=1;
