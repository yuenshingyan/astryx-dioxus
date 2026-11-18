// Source-token calculations, not a browser or assistive-technology certificate.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';

import {contrast,createColorResolver,over} from './contrast/colors.mjs';

const palette=JSON.parse(await readFile(new URL('./fixtures/button-core-palette.json',import.meta.url),'utf8'));
assert.equal(contrast([0,0,0,1],[1,1,1,1]),21);
const variants={
  primary:['--color-accent','--color-on-accent'],
  secondary:['--color-neutral','--color-text-primary'],
  ghost:['transparent','--color-text-primary'],
  destructive:['--color-error','--color-on-error'],
};
const report={
  revision:palette.revision,
  scope:'Core tokens, light/dark, page/surface backdrops. Essential glyph and spinner arc: rest/hover/pressed, 3:1. Focus: outside backdrop, 3:1. Resting surface boundary: conditional 3:1 only if needed to identify the control. Disabled exempt; arbitrary caller colors, custom themes, raster antialiasing, and images excluded.',
  essentialCases:0,essentialFailures:[],focusCases:0,focusFailures:[],boundaryCases:0,boundariesBelowConditionalTarget:[],
};
['light','dark'].forEach((mode,modeIndex)=>{
  const resolve=createColorResolver(palette.tokens,modeIndex);
  ['--color-background-body','--color-background-surface'].forEach(canvas=>{
    const backdrop=resolve(canvas);
    Object.entries(variants).forEach(([variant,[fill,foreground]])=>{
      const base=over(resolve(fill),backdrop);
      report.focusCases++;
      const focusRatio=contrast(resolve(variant==='destructive'?'--color-error':'--color-accent'),backdrop);
      if(focusRatio<3)report.focusFailures.push({mode,canvas,variant,ratio:+focusRatio.toFixed(4)});
      report.boundaryCases++;
      const boundaryRatio=contrast(base,backdrop);
      if(boundaryRatio<3)report.boundariesBelowConditionalTarget.push({mode,canvas,variant,ratio:+boundaryRatio.toFixed(4)});
      ['rest','hover','pressed'].forEach(state=>{
        const background=state==='rest'?base:over(resolve(`--color-overlay-${state}`),base);
        const ratio=contrast(resolve(foreground),background);
        report.essentialCases++;
        if(ratio<3)report.essentialFailures.push({mode,canvas,variant,state,ratio:+ratio.toFixed(4)});
      });
    });
  });
});
assert.equal(report.essentialCases,48);assert.equal(report.focusCases,16);assert.equal(report.boundaryCases,16);
if(process.argv.includes('--json'))console.log(JSON.stringify(report,null,2));
else {
  console.log(`IconButton core palette: ${report.essentialFailures.length}/${report.essentialCases} essential glyph/arc cases below 3:1; ${report.focusFailures.length}/${report.focusCases} focus/backdrop cases below 3:1.`);
  console.log(`${report.boundariesBelowConditionalTarget.length}/${report.boundaryCases} resting surfaces below conditional boundary 3:1. If identifying the action requires a visible boundary, supply a stronger theme. A recognizable glyph need not depend on a boundary; original paint is retained, not certified compliant.`);
}
if(report.essentialFailures.length||report.focusFailures.length||report.boundariesBelowConditionalTarget.length)process.exitCode=1;
