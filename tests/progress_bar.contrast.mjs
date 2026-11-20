// Source-token model, not a screen-reader/device audit or a WCAG certificate.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {createColorResolver,over,contrast} from './contrast/colors.mjs';
const fixture=JSON.parse(await readFile(new URL('./fixtures/progress-bar-neutral-reference.json',import.meta.url),'utf8'));
const fills={accent:'--color-accent',success:'--color-success',warning:'--color-warning',error:'--color-error',neutral:'--color-text-disabled',disabled:'--color-text-disabled',custom:'--color-accent'};
const inks={accent:'--color-on-accent',success:'--color-on-success',warning:'--color-on-warning',error:'--color-on-error',neutral:'--color-text-primary',disabled:'--color-text-secondary',custom:'--color-on-accent'};
const rows=[],trackRows=[],findings=[];
for(const theme of ['core','neutral'])for(const [index,mode] of ['light','dark'].entries()){
 const palette=fixture.pinnedPalettes[theme],tokens={...palette.declarations};
 if(palette.progressBarTrackOverride)tokens['--color-background-muted']=palette.progressBarTrackOverride.replace(/^--color-background-muted:\s*/,'');
 const color=createColorResolver(tokens,index),boundary=color('--color-text-primary');
 for(const surface of ['body','card','surface']){
  const background=color(`--color-background-${surface}`),track=over(color('--color-background-muted'),background);
  trackRows.push({theme,mode,surface,boundaryVsBackdrop:contrast(boundary,background)});
  for(const variant of Object.keys(fills)){
   const fill=color(fills[variant]),ink=color(inks[variant]),floor=over([...boundary.slice(0,3),.84],track);
   const row={theme,mode,surface,variant,sourceFillVsTrack:contrast(fill,track),boundaryVsTrack:contrast(boundary,track),floorBoundaryVsTrack:contrast(floor,track),onFillMarkVsFill:contrast(ink,fill),onTrackMarkVsTrack:contrast(color(variant==='disabled'?'--color-text-secondary':'--color-text-primary'),track)};
   rows.push(row);assert.ok(row.boundaryVsTrack>=3);assert.ok(row.floorBoundaryVsTrack>=3);
   if(variant!=='custom'){
    const source=fixture.contrast[`${theme}/${mode}`].variants.find(v=>v.variant===variant).surfaces[surface];
    assert.ok(Math.abs(row.sourceFillVsTrack-source.fillVsTrack)<1e-9,'source palette and alpha composition reproduced');
   }
   for(const key of ['onFillMarkVsFill','onTrackMarkVsTrack'])if(row[key]<3)findings.push({...row,kind:key,ratio:row[key],applicability:'Unchanged source disabled-marker ink. Conditional essential-mark finding; inactive-component applicability needs review. Marks never occur in indeterminate pulse mode.'});
  }
 }
}
assert.ok(trackRows.every(row=>row.boundaryVsBackdrop>=3));
console.log(`ProgressBar approved boundaries: 0/${rows.length} resting, 0/${rows.length} conservative .84-floor and 0/${trackRows.length} whole-track/backdrop failures below 3:1.`);
console.log(`Unchanged source disabled-marker findings: ${findings.length}/${rows.length*2} below 3:1; no claim that the approved graphic boundary fixes marker ink.`);
console.log(JSON.stringify({minimumBoundary:Math.min(...rows.map(r=>r.boundaryVsTrack)),minimumFloorBoundary:Math.min(...rows.map(r=>r.floorBoundaryVsTrack)),minimumTrackBackdrop:Math.min(...trackRows.map(r=>r.boundaryVsBackdrop)),findings},null,2));
if(findings.length)process.exitCode=1;
