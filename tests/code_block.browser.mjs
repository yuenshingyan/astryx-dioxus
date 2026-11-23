import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.addScriptToEvaluateOnNewDocument', {source: `
    Object.defineProperty(navigator, 'clipboard', {value: {async writeText(text) {
      if (window.rejectCodeCopy) throw new Error('Clipboard denied');
      window.copiedCode = text;
    }}});
  `});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8246'});
  const block = `document.querySelector('#code-block-examples')`;
  const pre = `${block}.querySelector('pre')`;
  const button = `${block}.querySelector('button')`;
  const status = `${block}.querySelector('[data-code-block-status][role=status]')`;
  await waitFor(`${block} && ${button}`);
  assert.equal(await evaluate(`${pre}.tabIndex`), 0);
  assert.equal(await evaluate(`${pre}.querySelector('code').textContent.split('\\n').length`), 13);
  assert.equal(await evaluate(`${pre}.querySelectorAll('[aria-hidden=true][data-number]').length`), 13);
  await evaluate(`${button}.click()`);
  await waitFor(`${status}.textContent === 'Copied code'`);
  assert.equal(await evaluate('window.copiedCode'), await evaluate(`${pre}.querySelector('code').textContent`));
  assert.ok((await evaluate('window.copiedCode')).includes('fetch(`/api/users/${id}`)'));
  await evaluate(`window.rejectCodeCopy = true; ${button}.click()`);
  await waitFor(`${status}.textContent === 'Could not copy code'`);
  assert.deepEqual(exceptions, []);
  console.log('Code Block: line gutter, exact-source copy and failure status passed');
});
