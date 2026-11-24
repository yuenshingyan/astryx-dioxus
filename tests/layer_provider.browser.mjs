import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8279'});
  const root = "document.getElementById('layer-provider-examples')";
  const toggle = `${root}?.querySelector('button')`;
  const panel = name => `${root}.querySelector('[role=dialog][aria-label="${name}"]')`;
  const edge = (name, key) => `${panel(name)}.style.getPropertyValue('--_astryx-layer-inset-${key}')`;
  await waitFor(toggle);
  assert.equal(await evaluate(edge('Inherited inset', 'block-end')), '56px');
  assert.equal(await evaluate(edge('Local inset', 'inline-start')), '24px');
  assert.equal(await evaluate(edge('Local inset', 'block-end')), '');
  assert.equal(await evaluate(`${panel('No provider')}.hasAttribute('data-layer-inset')`), false);
  const trigger = `document.querySelector('[aria-controls="' + ${panel('Inherited inset')}.id + '"]')`;
  await evaluate(`${trigger}.scrollIntoView({block:'center'}); ${trigger}.click()`);
  await waitFor(`${panel('Inherited inset')}.matches(':popover-open')`);
  assert.ok(await evaluate(`${panel('Inherited inset')}.getBoundingClientRect().width <= innerWidth - 96 + 1`));
  await evaluate(`${toggle}.click()`);
  await waitFor(`!${panel('Inherited inset')}.hasAttribute('data-layer-inset')`);
  assert.equal(await evaluate(edge('Local inset', 'inline-start')), '24px');
  assert.deepEqual(exceptions, []);
  console.log('LayerProvider: inherited and replaced edges, sibling isolation, reactive clearing, and viewport width budget passed');
});
