import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8115'});
  await waitFor('!!document.getElementById("stack-item-scroll")');
  assert.deepEqual(await evaluate(`['showcase','fill'].map(id => [...document.getElementById('stack-item-'+id).children].map(item => getComputedStyle(item).flexGrow))`), [['0','1','0'],['0','1']]);
  assert.deepEqual(await evaluate(`['showcase','fill'].map(id => {
    const root = document.getElementById('stack-item-'+id);
    return [...root.children].reduce((width,item)=>width+item.getBoundingClientRect().width,0) + (root.children.length-1)*parseFloat(getComputedStyle(root).columnGap) === root.clientWidth;
  })`), [true,true]);
  await waitFor('document.getElementById("stack-item-scroll").getAttribute("role") === "group"');
  assert.equal(await evaluate('document.getElementById("stack-item-scroll").getAttribute("aria-label")'), 'Scrollable example');
  assert.equal(await evaluate(`(() => {const item=document.getElementById('stack-item-scroll'); item.scrollTop=30; return item.scrollTop;})()`), 30);
  await evaluate('document.getElementById("stack-item-toggle-overflow").click()');
  await waitFor('!document.getElementById("stack-item-scroll").hasAttribute("role") && !document.getElementById("stack-item-scroll").hasAttribute("aria-label")');
  await evaluate('document.getElementById("stack-item-toggle-overflow").click()');
  await waitFor('document.getElementById("stack-item-scroll").getAttribute("role") === "group"');
  assert.equal(exceptions.length,0);
  console.log('StackItem: both patterns, fill sizing, native scrolling, and dynamic overflow naming passed');
});
