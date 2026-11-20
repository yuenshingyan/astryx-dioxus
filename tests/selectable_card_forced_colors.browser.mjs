import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async({send,evaluate,waitFor,exceptions})=>{
  await send('Page.navigate',{url:process.env.INTERACTIVE_CARD_BOUNDARIES_URL||'http://127.0.0.1:8083'});
  await waitFor(`document.querySelector('#forced-default > input')`);
  const ids=['forced-default','forced-transparent','forced-custom','forced-disabled','forced-scroll'];
  const state=async id=>evaluate(`(()=>{const e=document.getElementById(${JSON.stringify(id)}),s=getComputedStyle(e),b=e.getBoundingClientRect();return{selected:e.dataset.selected,checked:e.querySelector('input').checked,box:[b.width,b.height],border:s.borderColor,width:s.borderWidth,shadow:s.boxShadow,outline:s.outlineWidth,offset:s.outlineOffset,background:s.backgroundColor}})()`);
  const settle=async id=>evaluate(`Promise.all(document.getElementById(${JSON.stringify(id)}).getAnimations({subtree:true}).map(a=>a.finished.catch(()=>{})))`);
  const toggle=async(id,selected)=>{await evaluate(`document.querySelector('#'+${JSON.stringify(id)}+' > input').click()`);await waitFor(`document.getElementById(${JSON.stringify(id)}).dataset.selected === '${selected}'`);await settle(id);};
  const normal={};
  for(const id of ids){await settle(id);normal[id]=await state(id);assert.equal(normal[id].selected,'true');}
  for(const id of ids.filter(id=>id!=='forced-disabled')){
    await toggle(id,false);assert.deepEqual((await state(id)).box,normal[id].box);
    await toggle(id,true);
  }
  await send('Emulation.setEmulatedMedia',{features:[{name:'forced-colors',value:'active'}]});
  const colors=await evaluate(`(()=>{const e=document.createElement('span');e.style.color='CanvasText';e.style.backgroundColor='Canvas';document.body.append(e);const s=getComputedStyle(e),c={ink:s.color,canvas:s.backgroundColor};e.remove();return c})()`);
  for(const id of ids){
    await settle(id);const selected=await state(id);
    assert.equal(selected.border,colors.ink,id);assert.equal(selected.width,'2px',id);assert.equal(selected.shadow,'none',id);
    if(id==='forced-disabled'){
      await evaluate(`document.querySelector('#forced-disabled > input').click()`);
      assert.equal((await state(id)).checked,true);continue;
    }
    await toggle(id,false);const unselected=await state(id);
    assert.equal(unselected.border,colors.canvas,id);assert.equal(unselected.width,'2px',id);
    assert.deepEqual(unselected.box,selected.box,`${id}: selection does not change forced-mode layout`);
    await toggle(id,true);
  }
  await send('Input.dispatchKeyEvent',{type:'keyDown',key:'Tab',code:'Tab',windowsVirtualKeyCode:9});
  await send('Input.dispatchKeyEvent',{type:'keyUp',key:'Tab',code:'Tab',windowsVirtualKeyCode:9});
  for(const id of ids){
    await evaluate(`document.querySelector('#'+${JSON.stringify(id)}+' > input').focus()`);
    assert.equal(await evaluate(`document.querySelector('#'+${JSON.stringify(id)}+' > input').matches(':focus-visible')`),true,id);
    const focused=await state(id);assert.equal(focused.border,colors.ink,id);assert.ok(parseFloat(focused.outline)>0,id);assert.ok(parseFloat(focused.offset)>=0,id);
  }
  const scrollBefore=await state('forced-scroll');
  assert.equal(await evaluate(`(()=>{const e=document.getElementById('forced-scroll');e.scrollTop=160;return e.scrollTop>0&&e.scrollHeight>e.clientHeight})()`),true);
  assert.deepEqual(await state('forced-scroll'),scrollBefore,'root border and focus cue remain stationary through content scrolling');
  await evaluate(`document.activeElement.blur()`);
  await send('Emulation.setEmulatedMedia',{features:[{name:'forced-colors',value:'none'}]});
  for(const id of ids){await settle(id);const after=await state(id);for(const property of ['box','border','width','shadow','background'])assert.deepEqual(after[property],normal[id][property],`${id}: normal ${property} unchanged`);}
  assert.deepEqual(exceptions,[]);
  console.log('SelectableCard forced colors passed: default/borderless/custom/disabled, independent focus, stable selection geometry, stationary scroll cue and unchanged normal palette.');
});
