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
  assert.deepEqual(await evaluate(`(() => {
    const button = ${filterClear};
    return [button.type, document.getElementById(button.getAttribute('aria-describedby'))?.textContent];
  })()`), ['button', 'Clear']);

  // Synthetic dispatch checks the DOM cancellation policy, not native touch activation.
  assert.deepEqual(await evaluate(`(() => {
    const button = ${filterClear};
    const options = {bubbles: true, cancelable: true};
    return [
      ...['mouse', 'pen', 'touch'].map(pointerType => button.dispatchEvent(new PointerEvent('pointerdown', {...options, pointerType}))),
      button.dispatchEvent(new MouseEvent('mousedown', options)),
    ];
  })()`), [false, false, true, false]);
  assert.equal(await evaluate(`${filter}.value`), 'Design systems');

  const key = async (key, code, windowsVirtualKeyCode, text) => {
    await send('Input.dispatchKeyEvent', {type: 'keyDown', key, code, windowsVirtualKeyCode, text});
    await send('Input.dispatchKeyEvent', {type: 'keyUp', key, code, windowsVirtualKeyCode});
  };
  const wasCleared = `${filter}.value === '' && !${filterClear} && document.activeElement === ${filter}`;
  await evaluate(`${filter}.focus()`);
  for (const activation of [['Enter', 'Enter', 13, '\r'], [' ', 'Space', 32, ' ']]) {
    await key('Tab', 'Tab', 9);
    assert.ok(await evaluate(`document.activeElement === ${filterClear}`));
    await key(...activation);
    await waitFor(wasCleared);
    await send('Input.insertText', {text: 'Roadmap'});
    await waitFor(`${filter}.value === 'Roadmap' && !!${filterClear}`);
  };
  const point = await evaluate(`(() => { ${filterClear}.scrollIntoView({block:'center'}); const r = ${filterClear}.getBoundingClientRect(); return {x:r.x+r.width/2,y:r.y+r.height/2}; })()`);
  await send('Input.dispatchMouseEvent', {type: 'mousePressed', ...point, button: 'left', clickCount: 1});
  assert.ok(await evaluate(`document.activeElement === ${filter}`));
  await send('Input.dispatchMouseEvent', {type: 'mouseReleased', ...point, button: 'left', clickCount: 1});
  await waitFor(wasCleared);
  assert.deepEqual(exceptions, []);
  console.log('InputClearButton: DOM/AX name, press cancellation policy, keyboard and mouse activation, and input focus passed');
});
