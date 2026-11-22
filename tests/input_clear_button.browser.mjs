import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8216'});
  const field = id => `document.getElementById('${id}')`;
  const clear = id => `${field(id)}?.parentElement.querySelector('button')`;
  const filter = field('project-filter');
  const filterClear = clear('project-filter');
  await waitFor(`${filterClear}?.getAttribute('aria-label') === 'Clear Search field with value'`);
  for (const id of ['project-search', 'disabled-text-input']) {
    assert.ok(await evaluate(`${clear(id)} === null`));
  }
  assert.equal(await evaluate(`document.querySelector('#text-input-form button')`), null);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'button' && node.name?.value === 'Clear Search field with value'));
  await evaluate(`${filterClear}.focus()`);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13, text: '\r'});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`${filter}.value === '' && !${filterClear} && document.activeElement === ${filter}`);
  await send('Input.insertText', {text: 'Roadmap'});
  await waitFor(`${filter}.value === 'Roadmap' && !!${filterClear}`);
  const point = await evaluate(`(() => { ${filterClear}.scrollIntoView({block:'center'}); const r = ${filterClear}.getBoundingClientRect(); return {x:r.x+r.width/2,y:r.y+r.height/2}; })()`);
  await send('Input.dispatchMouseEvent', {type: 'mousePressed', ...point, button: 'left', clickCount: 1});
  assert.ok(await evaluate(`document.activeElement === ${filter}`));
  await send('Input.dispatchMouseEvent', {type: 'mouseReleased', ...point, button: 'left', clickCount: 1});
  await waitFor(`${filter}.value === '' && !${filterClear} && document.activeElement === ${filter}`);
  assert.deepEqual(exceptions, []);
  console.log('InputClearButton: contextual name, keyboard and pointer activation, and input focus passed');
});
