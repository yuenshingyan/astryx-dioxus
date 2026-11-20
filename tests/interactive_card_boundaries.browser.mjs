import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

// Run against the interactive_card_boundaries example, not the main showcase.
const url=process.env.INTERACTIVE_CARD_BOUNDARIES_URL||'http://127.0.0.1:8083';
await withTarget(async({send,evaluate,pause,waitFor,exceptions})=>{
  await send('Page.navigate',{url});
  await waitFor(`document.querySelector('#inner-selectable > input') && document.querySelector('#inner-clickable > button')`);
  const text=()=>evaluate(`document.querySelector('#events').textContent`);
  const point=async selector=>evaluate(`(()=>{const e=document.querySelector(${JSON.stringify(selector)});e.scrollIntoView({block:'center'});const b=e.getBoundingClientRect();return{x:b.x+b.width/2,y:b.y+b.height/2}})()`);
  const mouseDown=async(selector,pointerType='mouse')=>{
    const p=await point(selector);
    await send('Input.dispatchMouseEvent',{type:'mouseMoved',...p,pointerType});
    await send('Input.dispatchMouseEvent',{type:'mousePressed',...p,button:'left',buttons:1,clickCount:1,pointerType});
    return p;
  };
  const mouseUp=async(p,pointerType='mouse')=>send('Input.dispatchMouseEvent',{type:'mouseReleased',...p,button:'left',buttons:0,clickCount:1,pointerType});
  const click=async selector=>{const p=await mouseDown(selector);await mouseUp(p);};
  const clear=async()=>{await click('#clear-events');await waitFor(`document.querySelector('#events').textContent === ''`);};

  await click('#inner-selectable-surface');
  await waitFor(`document.querySelector('#events').textContent === 'inner-change:true'`);
  assert.equal(await evaluate(`document.querySelector('#inner-selectable > input').checked`),true);
  assert.equal((await text()).includes('outer-action'),false);

  await clear();
  await click('#inner-clickable-surface');
  await waitFor(`document.querySelector('#events').textContent === 'inner-action|outer-click'`);
  assert.equal(await evaluate(`document.querySelector('#outer-selectable > input').checked`),false);

  // The innermost surface owns paint for each real browser pointer type.
  for(const pointerType of ['mouse','pen']){
    await clear();await pause(300);
    const hover=await point('#inner-selectable-surface');
    await send('Input.dispatchMouseEvent',{type:'mouseMoved',...hover,pointerType});
    await evaluate(`Promise.all(document.getElementById('outer-clickable').getAnimations({subtree:true}).map(animation=>animation.finished.catch(()=>{})))`);
    const outerPaint=()=>evaluate(`(()=>{const e=document.getElementById('outer-clickable'),p=getComputedStyle(e,'::after');return [p.backgroundColor,p.backgroundImage,p.opacity,getComputedStyle(e).borderColor]})()`);
    const before=await outerPaint();
    const p=await mouseDown('#inner-selectable-surface',pointerType);
    assert.deepEqual(await outerPaint(),before,`${pointerType}: nested press must not add ancestor pressed paint`);
    assert.deepEqual(await evaluate(`['outer-clickable','inner-selectable'].map(id=>document.getElementById(id).getAttribute('data-astryx-press'))`),[null,'on'],pointerType);
    await mouseUp(p,pointerType);
    await pause(300);
  }
  await send('Emulation.setTouchEmulationEnabled',{enabled:true,maxTouchPoints:1});
  await clear();await pause(300);
  const touch=await point('#inner-selectable-surface');
  await send('Input.dispatchTouchEvent',{type:'touchStart',touchPoints:[{...touch,id:1,radiusX:3,radiusY:3,force:1}]});
  assert.deepEqual(await evaluate(`['outer-clickable','inner-selectable'].map(id=>document.getElementById(id).getAttribute('data-astryx-press'))`),[null,'on']);
  await send('Input.dispatchTouchEvent',{type:'touchEnd',touchPoints:[]});
  await send('Emulation.setTouchEmulationEnabled',{enabled:false});
  await pause(300);

  await click('#disable-inner');
  await waitFor(`document.querySelector('#inner-selectable > input').getAttribute('aria-disabled') === 'true'`);
  await clear();
  await click('#inner-selectable-surface');
  await pause(100);
  assert.equal(await text(),'','disabled inner option must not fall through to outer action');
  await click('#inner-clickable-surface');
  await waitFor(`document.querySelector('#events').textContent === 'outer-click'`);
  assert.equal(await evaluate(`document.querySelector('#outer-selectable > input').checked`),false);
  await clear();await pause(300);
  const disabledPoint=await mouseDown('#inner-clickable-surface');
  assert.deepEqual(await evaluate(`['outer-selectable','inner-clickable'].map(id=>document.getElementById(id).getAttribute('data-astryx-press'))`),[null,null]);
  await mouseUp(disabledPoint);

  const openTip=async selector=>{
    const p=await point(selector);await send('Input.dispatchMouseEvent',{type:'mouseMoved',...p});
    const id=await evaluate(`document.querySelector(${JSON.stringify(selector)}).getAttribute('data-astryx-status-tooltip-id')+'-tooltip'`);
    await waitFor(`document.getElementById(${JSON.stringify(id)})?.matches(':popover-open')`);
    return '#'+id;
  };
  await clear();
  const tip=await openTip('#nested-help');
  assert.equal(await evaluate(`document.querySelector('#layer-outer').contains(document.querySelector(${JSON.stringify(tip)}))`),false,'real policy tooltip is portaled outside card DOM');
  await click(tip);
  await waitFor(`document.querySelector('#events').textContent === 'layer-outer-click|layer-inner-click'`);
  assert.deepEqual(await evaluate(`['layer-outer','layer-inner'].map(id=>document.querySelector('#'+id+' > input').checked)`),[false,false]);
  await clear();
  await click(await openTip('#outside-help'));
  await pause(100);
  assert.equal(await text(),'','unowned portal must not reach unrelated card callbacks');

  await clear();
  await evaluate(`window.detachedCard=document.querySelector('#inner-selectable')`);
  await click('#mount-cases');
  await waitFor(`!document.querySelector('#outer-clickable')`);
  await evaluate(`window.detachedCard.click();window.detachedCard=null`);
  await pause(100);
  assert.equal(await text(),'');
  await click('#mount-cases');
  await waitFor(`document.querySelector('#inner-selectable > input')`);
  await click('#disable-inner');
  await clear();
  await click('#inner-clickable-surface');
  await waitFor(`document.querySelector('#events').textContent === 'inner-action|outer-click'`);
  assert.deepEqual(exceptions,[]);
  console.log('Interactive card boundaries passed: cross-card ownership, disabled markers, mouse/pen/touch paint, ordered nested portaled capture, unrelated layers and removal/remount.');
});
