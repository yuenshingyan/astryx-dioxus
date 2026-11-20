// Opt-in network evidence; never needed by offline tests. Run under CDP lock.
import assert from 'node:assert/strict';
import {writeFile} from 'node:fs/promises';
import {withTarget} from './card_metrics.mjs';
await withTarget(async({send,evaluate,waitFor,pause,browser})=>{
 const url='https://astryx.atmeta.com/components/Stack';
 await send('Page.navigate',{url});
 await waitFor(`document.querySelectorAll('#example-StackFillItem .astryx-avatar').length===3`);
 await evaluate('document.fonts.ready.then(()=>true)');
 const evidence={url,capturedAt:new Date().toISOString(),browser:browser.Browser,deploymentCommit:'unknown',sourcePin:'476a3153417dbf75c565f7df05b0784e94ddd59e',viewport:[1200,1600,1],observations:await evaluate(`(()=>{
  const root=document.querySelector('#example-StackFillItem'),a=root.querySelector('.astryx-avatar'),s=getComputedStyle(a);
  return{avatarAttributes:Object.fromEntries([...a.attributes].filter(a=>a.name!=='class'&&a.name!=='style').map(a=>[a.name,a.value])),fontSmoothing:s.webkitFontSmoothing,bodyFontSmoothing:getComputedStyle(document.body).webkitFontSmoothing,images:[...root.querySelectorAll('img')].map(e=>({src:e.src,alt:e.alt})),tokenActions:document.querySelectorAll('#example-StackDirections .astryx-token button,#example-StackDirections .astryx-token a').length}
 })()`)};
 await evaluate(`window.avatar=document.querySelector('#example-StackFillItem .astryx-avatar');avatar.scrollIntoView({block:'center'});avatar.focus()`);
 for(const modifiers of [8,0])for(const type of ['keyDown','keyUp'])await send('Input.dispatchKeyEvent',{type,key:'Tab',code:'Tab',windowsVirtualKeyCode:9,modifiers});
 await waitFor(`[...document.querySelectorAll('[role=tooltip]')].some(e=>e.textContent==='Olivia Chen'&&e.getBoundingClientRect().width>0)`);
 await pause(400);
 evidence.keyboard=await evaluate(`({isAvatar:document.activeElement===avatar,focusVisible:avatar.matches(':focus-visible'),outline:getComputedStyle(avatar).outline,describedBy:avatar.getAttribute('aria-describedby'),tips:[...document.querySelectorAll('[role=tooltip]')].filter(e=>e.getBoundingClientRect().width>0).map(e=>({text:e.textContent,rect:e.getBoundingClientRect().toJSON()}))})`);
 for(const type of ['keyDown','keyUp'])await send('Input.dispatchKeyEvent',{type,key:'Escape',code:'Escape',windowsVirtualKeyCode:27});
 await pause(400);
 evidence.escape=await evaluate(`({active:document.activeElement===avatar,visibleTips:[...document.querySelectorAll('[role=tooltip]')].filter(e=>{const b=e.getBoundingClientRect();return b.width&&b.height&&getComputedStyle(e).visibility!=='hidden'}).length})`);
 await evaluate(`document.querySelector('#example-StackFillItem .astryx-button').click()`);
 await waitFor(`document.body.innerText.includes('Demo: this profile would open here')`);
 evidence.action=await evaluate(`({text:[...document.querySelectorAll('[role=status],[role=alert]')].map(e=>e.textContent).filter(Boolean),messagePresent:document.body.innerText.includes('Demo: this profile would open here')})`);
 assert.equal(evidence.keyboard.isAvatar,true);assert.equal(evidence.keyboard.describedBy,null);assert.equal(evidence.keyboard.tips[0].text,'Olivia Chen');assert.equal(evidence.escape.visibleTips,0);assert.equal(evidence.observations.images.length,0);
 await writeFile(new URL('./fixtures/stack-published-interactions.json',import.meta.url),JSON.stringify(evidence,null,2)+'\n');
 console.log('Captured original Avatar focus/tooltip/Escape and published View message; no external profile images.');
});
