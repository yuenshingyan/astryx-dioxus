import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8149'});
  await waitFor(`!!document.getElementById('top-nav-narrow-links')`);
  assert.deepEqual(await evaluate(`(() => {
    const section = document.getElementById('top-nav-heading-examples');
    const logo = document.getElementById('top-nav-logo-only');
    const row = document.getElementById('top-nav-narrow');
    const link = row.querySelector('a');
    const title = link.querySelector('.astryx-top-nav-heading__title');
    const trailing = row.querySelector('button');
    const linked = [...document.querySelectorAll('#top-nav-narrow-links .astryx-link')];
    trailing.focus();
    return [
      [...section.querySelectorAll('nav > *')].map(node => node.tagName),
      getComputedStyle(logo).gridTemplateColumns.trim().split(/\\s+/).length,
      logo.querySelector('svg').getBoundingClientRect().width,
      row.offsetHeight >= 32,
      title.scrollWidth > title.clientWidth && getComputedStyle(title).textOverflow === 'ellipsis',
      trailing.getBoundingClientRect().right <= link.getBoundingClientRect().left,
      trailing === document.activeElement && !link.contains(trailing),
      linked.length === 2 && linked.every(node => node.clientWidth > 0 && node.scrollWidth <= node.clientWidth),
    ];
  })()`), [['A', 'DIV', 'A'], 1, 20, true, true, true, true, true]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'link' && node.name?.value === 'Acme home'));
  assert.deepEqual(exceptions, []);
  console.log('TopNavHeading: logo-only naming and sizing, RTL truncation, wrapped links, and trailing focus passed');
});
