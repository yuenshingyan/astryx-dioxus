import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8157'});
  await waitFor(`document.querySelectorAll('#citation-examples .astryx-citation').length === 13`);
  const props = await evaluate(`(() => {
    const examples = document.querySelector('#citation-examples');
    const linked = examples.querySelector('a.astryx-citation[data-variant="number"]');
    const inert = examples.querySelector('span.astryx-citation[data-variant="number"]');
    const icon = examples.querySelector('.astryx-citation__icon');
    linked.focus();
    return [linked.target, linked.rel, getComputedStyle(linked).verticalAlign,
      linked === document.activeElement, inert.tabIndex, inert.getAttribute('role'),
      icon.getAttribute('aria-hidden'), getComputedStyle(icon.parentElement).verticalAlign];
  })()`);
  assert.deepEqual(props, ['_blank', 'noopener noreferrer', 'super', true, -1, 'group', 'true', 'baseline']);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.name?.value === 'Citation 1: React Documentation' && ['link', 'doc-noteref'].includes(node.role?.value)));
  assert.ok(nodes.some(node => !node.ignored && node.name?.value === 'Citation 3: W3C Specification' && node.role?.value === 'group'));
  assert.deepEqual(exceptions, []);
  console.log('Citation: inline variants, safe new-tab attributes, keyboard focus, and accessible names passed');
});
