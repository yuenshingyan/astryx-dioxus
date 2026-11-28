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
      row.querySelectorAll('a').length === 1 && link.getAttribute('href') === '#top-nav-heading-examples',
      linked.length === 2 && linked.every(node => node.clientWidth > 0 && node.scrollWidth <= node.clientWidth),
    ];
  })()`), [['A', 'DIV', 'A'], 1, 20, true, true, true, true, true, true]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  const hasAXNode = (role, name) => nodes.some(node => !node.ignored && node.role?.value === role && node.name?.value === name);
  assert.ok(hasAXNode('link', 'Acme home'));
  assert.ok(hasAXNode('link', 'Acme Platform product and account'));
  assert.ok(!hasAXNode('heading', 'Acme Platform'));

  async function press(key, windowsVirtualKeyCode) {
    const text = key === 'Enter' ? '\r' : undefined;
    await send('Input.dispatchKeyEvent', {type: 'keyDown', key, code: key, windowsVirtualKeyCode, text});
    await send('Input.dispatchKeyEvent', {type: 'keyUp', key, code: key, windowsVirtualKeyCode});
  }
  await evaluate(`document.querySelector('#top-nav-narrow a').focus()`);
  await press('Tab', 9);
  assert.ok(await evaluate(`document.activeElement === document.querySelector('#top-nav-narrow button')`));
  await press('Enter', 13);
  await waitFor(`document.getElementById('top-nav-settings-activations').textContent.trim() === '1'`);
  assert.equal(await evaluate('location.hash'), '');
  await evaluate(`document.querySelector('#top-nav-narrow a').focus()`);
  await press('Enter', 13);
  await waitFor(`location.hash === '#top-nav-heading-examples'`);
  assert.equal(await evaluate(`document.getElementById('top-nav-settings-activations').textContent.trim()`), '1');
  assert.deepEqual(exceptions, []);
  console.log('TopNavHeading: DOM/AX, RTL geometry, and independent native keyboard activation passed');
});
