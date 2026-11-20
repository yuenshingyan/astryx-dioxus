// Raster-only control: identical actual nodes/rectangles, layout vs fractional transform.
// No source paint, font, size or child declaration is replaced in either arm.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFile, mkdir, writeFile} from 'node:fs/promises';
import {withTarget} from './card_metrics.mjs';
const published=JSON.parse(await readFile(new URL('./fixtures/stack-published.json',import.meta.url),'utf8'));
const out=new URL('./fixtures/stack-raster-controls/',import.meta.url);
await mkdir(out,{recursive:true});
await withTarget(async({send,evaluate,waitFor,browser,exceptions})=>{
 await send('Network.enable');await send('Network.setBlockedURLs',{urls:['https://*']});
 await send('Page.navigate',{url:process.env.STACK_URL??'http://127.0.0.1:8097'});
 await waitFor(`document.querySelectorAll('#st-fill .astryx-avatar').length===3`);
 await evaluate(`document.fonts.load('500 12px Figtree').then(()=>document.fonts.load('700 14px Figtree')).then(()=>document.fonts.ready).then(()=>true)`);
 await send('Network.emulateNetworkConditions',{offline:true,latency:0,downloadThroughput:0,uploadThroughput:0});
 await evaluate(`(()=>{
  document.documentElement.style.cssText='font-size:16px;color-scheme:dark';
  const main=document.querySelector('main');main.dataset.astryxTheme='neutral';
  const stage=document.createElement('div');stage.id='raster-stage';stage.style.cssText='position:absolute;z-index:1;left:40px;top:40px;display:flex;flex-direction:column;align-items:center;background:var(--color-background-card);color:var(--color-text-primary);font-family:var(--font-family-body);font-size:14px;line-height:1.4286;-webkit-font-smoothing:antialiased';main.prepend(stage);
  for(const id of ['st-directions','st-alignment','st-fill']){const host=document.createElement('div');host.style.cssText='display:inline-flex;margin-bottom:32px';stage.append(host);host.append(document.getElementById(id))}
 })()`);
 const report={kind:'Same-node raster control, no component paint changes',browser:browser.Browser,viewport:[1200,1600,1],mode:'Neutral dark, LTR, root16, antialiased Figtree',sourceCapture:published.capturedAt,cases:{}};
 for(const [name,id]of [['directions','st-directions'],['alignment','st-alignment'],['fill','st-fill']]){
  const origin=published.cases[name].clip;
  const delta=await evaluate(`(()=>{const r=document.getElementById('${id}').getBoundingClientRect();return [${origin.x%1}-r.x%1,${origin.y%1}-r.y%1]})()`);
  const entry=report.cases[name]={delta};
  for(const mode of ['transform','layout','repeat']){
   await evaluate(`(()=>{const h=document.getElementById('${id}').parentElement;h.style.transform='none';h.style.left='0';h.style.top='0';h.style.position='relative';${mode==='transform'?`h.style.transform='translate(${delta[0]}px,${delta[1]}px)'`:`h.style.left='${delta[0]}px';h.style.top='${delta[1]}px'`}})()`);
   const metrics=await evaluate(`(()=>{const root=document.getElementById('${id}'),r=root.getBoundingClientRect();return {clip:{x:r.x,y:r.y,width:r.width,height:r.height,scale:1},boxes:[root,...root.querySelectorAll('*')].filter(e=>!e.matches('[role=tooltip]')).map(e=>{const b=e.getBoundingClientRect();return [b.x,b.y,b.width,b.height]})}})()`);
   if(mode!=='transform')assert.deepEqual(metrics,entry.transform.metrics,`${name}: identical geometry in both raster paths`);
   const shot=await send('Page.captureScreenshot',{format:'png',captureBeyondViewport:true,clip:metrics.clip});
   const bytes=Buffer.from(shot.data,'base64'),path=`${name}-${mode}.png`;
   entry[mode]={metrics,screenshot:{path,sha256:createHash('sha256').update(bytes).digest('hex')}};
   await writeFile(new URL(path,out),bytes);
  }
  assert.equal(entry.layout.screenshot.sha256,entry.repeat.screenshot.sha256,`${name}: unchanged repeat capture`);
  console.log(name,'same-node geometry exact; layout repeat byte-identical; transform control retained');
 }
 assert.equal(exceptions.length,0,JSON.stringify(exceptions));
 await writeFile(new URL('measurements.json',out),JSON.stringify(report,null,2)+'\n');
});
