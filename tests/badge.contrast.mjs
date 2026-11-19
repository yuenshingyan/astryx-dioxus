// Standalone Badge surfaces, not the separate Button-slot composite audit.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {contrast,createColorResolver,over} from './contrast/colors.mjs';
const core=JSON.parse(await readFile(new URL('./fixtures/button-core-palette.json',import.meta.url),'utf8'));
const neutral=JSON.parse(await readFile(new URL('./fixtures/badge-neutral-reference.json',import.meta.url),'utf8'));
const palettes={core:core.tokens,neutral:{...core.tokens,...neutral.tokens,'--color-background-body':'light-dark(#fff,#1b1b1b)','--color-background-surface':'light-dark(#fff,#262626)'}};
const variants=['neutral','info','success','warning','error','blue','cyan','green','orange','pink','purple','red','teal','yellow'];
const report={revision:core.revision,scope:'All 14 standalone built-in tones on core and showcase Neutral page/surface in light/dark. No hover/pressed state is invented for static Badge. Arbitrary parent/control backgrounds and rich/custom content are excluded; previous Button-slot findings remain separate.',textCases:0,textFailures:[],inheritedIconCases:0,inheritedIconFailures:[]};
assert.equal(contrast([0,0,0,1],[1,1,1,1]),21);
for(const [theme,tokens] of Object.entries(palettes))for(const [modeIndex,mode] of ['light','dark'].entries()){
 const color=createColorResolver(tokens,modeIndex);
 for(const canvas of ['--color-background-body','--color-background-surface'])for(const variant of variants){
  let foreground,fill;
  if(variant==='neutral'){
   foreground=color(theme==='neutral'?'--color-text-gray':'--color-text-primary');fill=color(theme==='neutral'?'--color-background-gray':'--color-neutral');
  }else if(['info','success','warning','error'].includes(variant)){
   foreground=color('--color-on-'+variant);fill=color('--color-'+variant);
  }else{foreground=color('--color-text-'+variant);fill=color('--color-background-'+variant)}
  const backdrop=over(fill,color(canvas)),ratio=contrast(over(foreground,backdrop),backdrop),info={theme,mode,canvas,variant,ratio:+ratio.toFixed(4)};
  report.textCases++;report.inheritedIconCases++;
  if(ratio<4.5)report.textFailures.push(info);
  if(ratio<3)report.inheritedIconFailures.push(info);
 }
}
assert.equal(report.textCases,112);assert.equal(report.inheritedIconCases,112);
if(process.argv.includes('--json'))console.log(JSON.stringify(report,null,2));
else console.log(`Standalone Badge: ${report.textFailures.length}/112 text failures below 4.5:1; ${report.inheritedIconFailures.length}/112 inherited glyph failures below 3:1. This does not replace the existing Badge-inside-Button composite audit.`);
if(report.textFailures.length||report.inheritedIconFailures.length)process.exitCode=1;
