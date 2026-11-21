import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8166'});
  const option = value => `document.querySelector('#command-palette-item-examples [data-value="${value}"]')`;
  const picked = `document.getElementById('command-palette-item-picked')`;
  await waitFor(`!!${option('save')}`);
  assert.deepEqual(await evaluate(`[${option('open-recent')}.dataset.highlighted, ${option('publish')}.tabIndex, ${option('save')}.querySelectorAll('kbd').length]`), ['highlighted', -1, 2]);
  await evaluate(`${option('save')}.click()`);
  await waitFor(`${picked}.textContent === 'save' && ${option('save')}.getAttribute('aria-selected') === 'true'`);
  await evaluate(`${option('publish')}.click()`);
  assert.equal(await evaluate(`${picked}.textContent`), 'save');
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'option' && node.name?.value === 'Publish (disabled)' && node.properties?.some(property => property.name === 'disabled' && property.value?.value === true)));
  assert.deepEqual(exceptions, []);
  console.log('CommandPaletteItem: pointer selection, disabled option, and state reflection passed');
});
