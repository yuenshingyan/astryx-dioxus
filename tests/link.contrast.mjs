// Source-token audit. Active color="disabled" is not a disabled-control exemption.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {contrast,createColorResolver,over} from './contrast/colors.mjs';
const palette=JSON.parse(await readFile(new URL('./fixtures/button-core-palette.json',import.meta.url),'utf8'));
assert.equal(contrast([0,0,0,1],[1,1,1,1]),21);
const colors={primary:'--color-text-primary',secondary:'--color-text-secondary',disabled:'--color-text-disabled',placeholder:'--color-text-secondary',accent:'--color-text-accent',inherit:'--color-text-primary'};
const report={revision:palette.revision,scope:'Active built-in colors on core page/surface, light/dark. Inherit assumes the default primary parent. Hover-capable and coarse pressed states are separate. Disabled controls, custom colors/themes, and arbitrary surrounding text/backgrounds are excluded; no 1.4.1 link-distinguishability certificate is implied.',textCases:0,textFailures:[],focusCases:0,focusFailures:[]};
['light','dark'].forEach((mode,modeIndex)=>{
  const resolve=createColorResolver(palette.tokens,modeIndex);
  ['--color-background-body','--color-background-surface'].forEach(canvas=>{
    const background=resolve(canvas),tint=resolve('--color-tint-hover');
    Object.entries(colors).forEach(([color,token])=>{
      const foreground=resolve(token);
      report.focusCases++;
      const focusRatio=contrast(resolve('--color-accent'),background);
      if(focusRatio<3)report.focusFailures.push({mode,canvas,color,ratio:+focusRatio.toFixed(4)});
      ['rest','hover','pressed-fine','pressed-coarse'].forEach(state=>{
        const is_tinted=['primary','secondary','accent'].includes(color)&&['hover','pressed-fine'].includes(state);
        const paint=is_tinted?[...foreground.slice(0,3).map((channel,index)=>channel*0.85+tint[index]*0.15),1]:foreground;
        const backdrop=state.startsWith('pressed')?over(resolve('--color-overlay-pressed'),background):background;
        const ratio=contrast(paint,backdrop);
        report.textCases++;
        if(ratio<4.5)report.textFailures.push({mode,canvas,color,state,ratio:+ratio.toFixed(4)});
      });
    });
  });
});
assert.equal(report.textCases,96);assert.equal(report.focusCases,24);
if(process.argv.includes('--json'))console.log(JSON.stringify(report,null,2));
else{
  console.log(`Link core palette: ${report.textFailures.length}/${report.textCases} active text cases below 4.5:1; ${report.focusFailures.length}/${report.focusCases} focus/backdrop cases below 3:1.`);
  console.log('color="disabled" alone does not disable a Link. Original colors are retained; use is_disabled for unavailable controls or a stronger theme for active muted links.');
}
if(report.textFailures.length||report.focusFailures.length)process.exitCode=1;
