import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Page.navigate', {url:process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8287'});
  const region = "document.getElementById('dropdown-menu-link-rows')";
  const trigger = `${region}?.querySelector('button')`;
  const menu = `${region}.querySelector('[role=menu]')`;
  const rows = `${menu}.querySelectorAll('[role=menuitem]')`;
  await waitFor(trigger);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  await evaluate(`${trigger}.focus(); ${trigger}.click()`);
  await waitFor(`${menu}.matches(':popover-open') && document.activeElement === ${rows}[0]`);
  assert.deepEqual(await evaluate(`Array.from(${rows}, node => [node.tagName, node.getAttribute('href'), node.getAttribute('aria-disabled')])`), [
    ['A', '#calendar-examples', null], ['A', null, 'true'], ['A', null, 'true'], ['A', '#carousel-examples', null],
  ]);
  assert.ok(await evaluate(`${rows}[3].target === '_blank' && ${rows}[3].relList.contains('noopener') && ${rows}[3].relList.contains('noreferrer')`));
  const key = async (key, code, text) => {
    await send('Input.dispatchKeyEvent',{type:'keyDown',key,code:key,windowsVirtualKeyCode:code,text});
    await send('Input.dispatchKeyEvent',{type:'keyUp',key,code:key,windowsVirtualKeyCode:code});
  };
  await key('ArrowDown',40);
  await waitFor(`document.activeElement === ${rows}[1]`);
  await key('Enter',13,'\r');
  assert.ok(await evaluate(`${menu}.matches(':popover-open') && !location.hash`));
  await key('Home',36);
  await waitFor(`document.activeElement === ${rows}[0]`);
  await key('Enter',13,'\r');
  await waitFor(`!${menu}.matches(':popover-open') && location.hash === '#calendar-examples' && document.querySelector('#dropdown-menu-examples output').textContent === 'Opened calendar'`);
  assert.deepEqual(exceptions, []);
  console.log('DropdownMenuItem links: native anchor, inert destinations, safe new-tab relationship, and keyboard navigation passed');
});
