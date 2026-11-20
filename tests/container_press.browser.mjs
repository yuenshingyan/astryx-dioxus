// Shared row/card press profile through the real component/controller, offline.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {withTarget} from './card_metrics.mjs';
const timingSource=await readFile(new URL('../src/motion/theme_timing.js',import.meta.url),'utf8');
await withTarget(async({send,evaluate,pause,waitFor,exceptions})=>{
 await send('Page.navigate',{url:process.env.SHOWCASE_URL??'http://127.0.0.1:8081'});
 await waitFor('!!document.getElementById("cc-dynamic")');await pause(200);
 await evaluate('document.getElementById("cc-dynamic").scrollIntoView({block:"center"})');await pause(100);
 await evaluate(`window.pressTiming=(element)=>{${timingSource};return readThemeTimingTokens(element,'--duration-exit','--ease-exit',{duration:'175ms',milliseconds:175,easing:'linear'})};window.pressRoot=document.getElementById('cc-dynamic');true`);
 const event=async(type,pointerType='mouse',dx=0)=>evaluate(`(()=>{const r=pressRoot.getBoundingClientRect();pressRoot.dispatchEvent(new PointerEvent('${type}',{bubbles:true,cancelable:true,pointerType:'${pointerType}',pointerId:93,isPrimary:true,button:0,clientX:r.x+r.width/2+${dx},clientY:r.y+r.height/2}));return pressRoot.dataset.astryxPress??null})()`);
 const state=()=>evaluate('pressRoot.dataset.astryxPress??null');
 const setTime=async value=>evaluate(`pressRoot.style.setProperty('--duration-exit',${JSON.stringify(value)});true`);
 for(const [value,expected]of [['650ms',650],['.3s',300],['calc(300ms + 200ms)',500],['-1ms',175],['not-a-time',175],['Infinityms',175],['1s, 2s',175],['0ms',0]]){
  await setTime(value);assert.equal(await evaluate('pressTiming(pressRoot).duration'),expected,value);
 }
 await evaluate('pressRoot.style.setProperty("--test-exit","450ms")');await setTime('var(--test-exit)');assert.equal(await evaluate('pressTiming(pressRoot).duration'),450);
 await evaluate('pressRoot.style.setProperty("--ease-exit","steps(4, end)")');assert.equal(await evaluate('pressTiming(pressRoot).easing'),'steps(4)');
 await setTime('650ms');
 for(const pointer of ['mouse','pen','touch']){
  assert.equal(await event('pointerdown',pointer),'on',pointer);
  assert.equal(await evaluate('getComputedStyle(pressRoot).transform'),'none');
  assert.equal(await event('pointerup',pointer),'fading');await pause(250);assert.equal(await state(),'fading');
  assert.equal(await event('pointerdown',pointer),'on','repeated press cancels old fade');
  await pause(480);assert.equal(await state(),'on','old release must not erase a new press');
  assert.equal(await evaluate('getComputedStyle(pressRoot).getPropertyValue("--_astryx-press-strength").trim()'),'1');
  await event('pointercancel',pointer);assert.equal(await state(),null);
  // Internal 11px travel must cancel even without a scrollable ancestor.
  await event('pointerdown',pointer);assert.equal(await event('pointermove',pointer,11),null,pointer+' travel');await event('pointercancel',pointer);
 }
 // Source endAfterRelease waits a finite CSS animation after the initial clock.
 await setTime('40ms');await event('pointerdown');
 await evaluate('window.pressStyle=document.createElement("style");pressStyle.textContent="@keyframes press-proof {from{opacity:1}to{opacity:.99}} #cc-dynamic[data-astryx-press=fading]{animation:press-proof 400ms linear both!important}";document.head.append(pressStyle)');
 await event('pointerup');await pause(120);assert.equal(await state(),'fading');await waitFor('!pressRoot.hasAttribute("data-astryx-press")');await evaluate('pressStyle.remove()');
 // Invalid timing cannot throw/strand fading; source clock falls back to175ms.
 await setTime('not-a-time');await event('pointerdown');await event('pointerup');await pause(60);assert.equal(await state(),'fading');await waitFor('!pressRoot.hasAttribute("data-astryx-press")');
 // Fine-primary TOUCH still receives min(exit,200ms) with reduced motion.
 await send('Emulation.setTouchEmulationEnabled',{enabled:false});
 await send('Emulation.setEmulatedMedia',{features:[{name:'prefers-reduced-motion',value:'reduce'}]});
 assert.equal(await evaluate('matchMedia("(pointer:fine)").matches'),true);
 await setTime('650ms');await event('pointerdown','touch');await event('pointerup','touch');await pause(60);assert.equal(await state(),'fading');assert.equal(await evaluate('getComputedStyle(pressRoot).animationDuration'),'0.2s');await pause(200);assert.equal(await state(),null);
 await send('Emulation.setEmulatedMedia',{features:[{name:'prefers-reduced-motion',value:'no-preference'}]});
 assert.equal(exceptions.length,0,JSON.stringify(exceptions));
 console.log('Container press passed: native token grammar/fallback/calc/var/easing, all-pointer clocks/repeat/no scale/10px pan, real animation completion and reduced fine-touch bound.');
});
