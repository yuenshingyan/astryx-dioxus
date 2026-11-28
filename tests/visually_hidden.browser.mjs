import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8119'});
  await waitFor('!!document.getElementById("vh-actions")');
  assert.deepEqual(await evaluate(`['vh-heading','vh-live','vh-actions'].map(id => {
    const el = document.getElementById(id), css = getComputedStyle(el), rect = el.getBoundingClientRect();
    return [el.tagName, css.position, css.clip, rect.width, rect.height, css.pointerEvents, css.userSelect, el.getAttribute('aria-hidden')];
  })`), ['H2','DIV','DIV'].map(tag => [tag,'absolute','rect(0px, 0px, 0px, 0px)',1,1,'none','none',null]));
  await send('Accessibility.enable');
  const {nodes} = await send('Accessibility.getFullAXTree');
  const has = (role, name) => nodes.some(node => !node.ignored && node.role?.value === role && node.name?.value === name);
  assert.ok(has('heading', 'Build status'));
  for (const name of ['Download','Share','Delete']) assert.ok(has('button', name), name);
  for (const name of ['increase from last month','decrease from last month']) {
    assert.ok(nodes.some(node => !node.ignored && node.name?.value?.includes(name)), name);
  }
  await evaluate(`window.originalLive = document.getElementById('vh-live'); document.getElementById('vh-move').click()`);
  await waitFor('originalLive.textContent === "Task moved to In progress"');
  assert.equal(await evaluate('originalLive === document.getElementById("vh-live") && originalLive.getAttribute("aria-live") === "polite"'), true);
  assert.ok(has('button', 'Show passing builds only'));
  assert.deepEqual(await evaluate(`(() => {
    const control = document.getElementById('vh-filter');
    const region = window.originalResults = document.getElementById('vh-results');
    return [control.hasAttribute('aria-label'), control.hasAttribute('aria-labelledby'),
      region.tagName, region.getAttribute('aria-live'), region.textContent];
  })()`), [false, false, 'DIV', 'polite', '2 build results']);
  await evaluate(`document.getElementById('vh-filter').click()`);
  await waitFor('originalResults.textContent === "1 build result"');
  assert.equal(await evaluate(`originalResults === document.getElementById('vh-results')
    && document.getElementById('vh-filter').getAttribute('aria-pressed') === 'true'`), true);
  assert.equal(exceptions.length, 0);
  console.log('VisuallyHidden: clipping, AX content and names, and persistent task/result regions passed');
});
