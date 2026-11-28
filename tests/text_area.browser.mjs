import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8322'});
  const input = `document.getElementById('description-area')`;
  await waitFor(`${input}?.value === 'Initial description'`);
  assert.deepEqual(await evaluate(`[${input}.rows, getComputedStyle(${input}).resize, ${input}.labels[0].textContent.trim()]`), [3, 'vertical', 'Description']);
  assert.deepEqual(await evaluate(`[${input}.autocomplete, ${input}.spellcheck, document.getElementById('read-only-area').spellcheck]`), ['off', false, true]);
  await send('Page.bringToFront');
  await waitFor('document.hasFocus()');
  const lastEvent = "document.getElementById('text-area-last-event').textContent";
  await evaluate(`${input}.focus(); ${input}.setSelectionRange(${input}.value.length, ${input}.value.length)`);
  await waitFor(`${lastEvent} === 'Focused'`);
  await evaluate(`${input}.dispatchEvent(new ClipboardEvent('paste', {bubbles:true, cancelable:true}))`);
  await waitFor(`${lastEvent} === 'Paste requested'`);
  assert.equal(await evaluate(`${input}.value`), 'Initial description');
  const toggle = "document.getElementById('text-area-toggle-loading')";
  const spinner = `${input}.parentElement.querySelector('.astryx-text-area__loading .astryx-spinner')`;
  const padding = await evaluate(`getComputedStyle(${input}).paddingInlineEnd`);
  async function assertFlushResizeCorner() {
    const {edges, zIndex} = await evaluate(`(() => {
      const frame = ${input}.parentElement;
      const frameRect = frame.getBoundingClientRect();
      const controlRect = ${input}.getBoundingClientRect();
      const style = getComputedStyle(frame);
      return {
        edges: [
          controlRect.top - frameRect.top - parseFloat(style.borderTopWidth),
          frameRect.right - controlRect.right - parseFloat(style.borderRightWidth),
          frameRect.bottom - controlRect.bottom - parseFloat(style.borderBottomWidth),
          controlRect.left - frameRect.left - parseFloat(style.borderLeftWidth),
        ],
        zIndex: style.zIndex,
      };
    })()`);
    assert.ok(edges.every(delta => Math.abs(delta) < 0.5), `Textarea must fill the frame: ${edges}`);
    assert.equal(zIndex, '1');
  }
  await assertFlushResizeCorner();
  await evaluate(`${toggle}.click()`);
  await waitFor(`${input}.getAttribute('aria-busy') === 'true' && ${spinner}?.getAttribute('aria-hidden') === 'true'`);
  assert.ok(await evaluate(`parseFloat(getComputedStyle(${input}).paddingInlineEnd) > ${parseFloat(padding)}`));
  await assertFlushResizeCorner();
  await send('Input.insertText', {text: '\nSecond line'});
  await waitFor(`document.getElementById('description-preview').textContent.endsWith('Second line')`);
  assert.equal(await evaluate(`${input}.value`), 'Initial description\nSecond line');
  assert.ok(await evaluate(`document.activeElement === ${input}`));
  await evaluate(`${toggle}.click()`);
  await waitFor(`!${input}.hasAttribute('aria-busy') && !${spinner}`);
  assert.equal(await evaluate(`getComputedStyle(${input}).paddingInlineEnd`), padding);
  assert.deepEqual(await evaluate(`Array.from(new FormData(document.getElementById('text-area-form')))`), [['reference', 'Reference notes']]);
  await evaluate(`document.getElementById('disabled-area').focus()`);
  await waitFor(`${lastEvent} === 'Blurred'`);
  await send('Input.insertText', {text: 'blocked'});
  assert.deepEqual(await evaluate(`[document.activeElement.id, document.activeElement.value, document.activeElement.getAttribute('aria-disabled')]`), ['disabled-area', 'Unavailable', 'true']);
  const {nodes} = await send('Accessibility.getFullAXTree');
  const error = nodes.find(node => !node.ignored && node.role?.value === 'textbox' && node.name?.value.startsWith('Error message'));
  assert.match(error?.description?.value ?? '', /Explain the issue.*Enter a description/);
  assert.deepEqual(exceptions, []);
  console.log('TextArea: multiline editing, label/description, and read-only/disabled form semantics passed');
});
