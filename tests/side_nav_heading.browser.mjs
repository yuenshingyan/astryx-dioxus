import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8148'});
  await waitFor(`!!document.getElementById('side-nav-heading-basic')`);
  assert.deepEqual(await evaluate(`(() => {
    const whole = document.getElementById('side-nav-heading-basic');
    const trailing = document.getElementById('side-nav-heading-trailing');
    return [whole.tagName, whole.getAttribute('href'), whole.offsetHeight >= 32,
      !!whole.closest('nav > .astryx-side-nav__top'),
      !!whole.querySelector('h1, h2, h3, h4, h5, h6, [role="heading"]'),
      trailing.tagName, !!trailing.querySelector(':scope > a + span > button'),
      !!trailing.querySelector('a button')];
  })()`), ['A', '/', true, true, false, 'DIV', true, false]);

  const {root} = await send('DOM.getDocument');
  const {nodes} = await send('Accessibility.getFullAXTree');
  for (const [selector, role, name] of [
    ['#side-nav-heading-basic', 'link', 'Analytics'],
    ['#side-nav-heading-trailing > a', 'link', 'Analytics'],
    ['#side-nav-heading-trailing button', 'button', 'Settings'],
  ]) {
    const {nodeId} = await send('DOM.querySelector', {nodeId: root.nodeId, selector});
    const {node: {backendNodeId}} = await send('DOM.describeNode', {nodeId});
    const ax = nodes.find(node => node.backendDOMNodeId === backendNodeId);
    assert.deepEqual([ax?.ignored, ax?.role?.value, ax?.name?.value], [false, role, name], selector);
  }

  const press = async (key, windowsVirtualKeyCode, text) => {
    await send('Input.dispatchKeyEvent', {type: 'keyDown', key, code: key, windowsVirtualKeyCode, text});
    await send('Input.dispatchKeyEvent', {type: 'keyUp', key, code: key, windowsVirtualKeyCode});
  };
  await evaluate(`(() => {
    history.replaceState(history.state, '', location.pathname + location.search);
    document.querySelector('#side-nav-heading-trailing > a').focus();
  })()`);
  await press('Tab', 9);
  assert.equal(await evaluate(`document.activeElement.matches('#side-nav-heading-trailing button')`), true);
  await press('Enter', 13, '\r');
  await waitFor(`document.getElementById('side-nav-heading-settings-count').textContent === '1'`);
  assert.equal(await evaluate('location.hash'), '');

  await evaluate(`document.querySelector('#side-nav-heading-trailing > a').focus()`);
  await press('Enter', 13, '\r');
  await waitFor(`location.hash === '#side-nav-heading-examples'`);
  assert.equal(await evaluate(`document.getElementById('side-nav-heading-settings-count').textContent`), '1');
  assert.deepEqual(exceptions, []);
  console.log('SideNavHeading: Chrome DOM, AX names/roles, minimum geometry, and separate keyboard activation passed');
});
