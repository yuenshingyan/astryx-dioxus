import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8080'});
  await waitFor('document.querySelector("#grid-with-span")');
  assert.deepEqual(await evaluate(`['fixed','gallery','auto-fit','dashboard','with-span'].map(id => document.getElementById('grid-'+id).children.length)`), [12, 6, 6, 6, 9]);
  const tracks = id => evaluate(`getComputedStyle(document.getElementById('${id}')).gridTemplateColumns.split(' ').length`);
  assert.equal(await tracks('grid-fixed'), 3);
  assert.equal(await tracks('grid-gallery'), 2);
  assert.equal(await tracks('grid-auto-fit'), 2);
  await evaluate(`const range = document.getElementById('grid-panel-width'); range.value = 280; range.dispatchEvent(new Event('input',{bubbles:true}));`);
  await waitFor(`getComputedStyle(document.getElementById('grid-auto-fit')).gridTemplateColumns.split(' ').length === 1`);
  assert.deepEqual(await evaluate(`['grid-dashboard','grid-with-span'].map(id=>{const grid=document.getElementById(id);return grid.getBoundingClientRect().width===grid.lastElementChild.getBoundingClientRect().width})`), [true,true]);
  await evaluate(`document.getElementById('grid-fixed').dir='rtl'`);
  assert.equal(await evaluate(`(()=>{const grid=document.getElementById('grid-fixed');return grid.children[0].textContent==='Item 1' && grid.children[0].getBoundingClientRect().x>grid.children[1].getBoundingClientRect().x})()`), true);
  await evaluate(`document.head.insertAdjacentHTML('beforeend','<style>@media (max-width: 600px) { #grid-fixed {grid-template-columns:repeat(2,1fr)} }</style>')`);
  await send('Emulation.setDeviceMetricsOverride', {width: 600,height: 900,deviceScaleFactor: 1,mobile: false});
  assert.equal(await tracks('grid-fixed'), 2, 'unlayered responsive caller CSS overrides fixed props');
  console.log('Grid: five patterns, native track counts, panel resize, full spans, RTL DOM order and CSS override passed');
});
