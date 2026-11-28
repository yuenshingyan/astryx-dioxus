import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8227'});
  const overview = "document.getElementById('overview-tab')";
  const activity = "document.getElementById('activity-tab')";
  await waitFor(`${overview}?.tabIndex === 0`);
  await evaluate(`${overview}.focus()`);
  const key = async (name, code, text) => {
    await send('Input.dispatchKeyEvent', {type: 'keyDown', key: name, code: name, windowsVirtualKeyCode: code, text});
    await send('Input.dispatchKeyEvent', {type: 'keyUp', key: name, code: name, windowsVirtualKeyCode: code});
  };
  await key('ArrowLeft', 37);
  await waitFor(`document.activeElement === ${activity}`);
  assert.equal(await evaluate(`${overview}.getAttribute('aria-selected')`), 'true');
  assert.deepEqual(await evaluate(`[${overview}.tabIndex, ${activity}.tabIndex]`), [-1, 0]);
  await key('Enter', 13, '\r');
  await waitFor(`${activity}.getAttribute('aria-selected') === 'true' && !document.getElementById('activity-panel').hidden`);
  assert.ok(await evaluate(`document.getElementById('overview-panel').hidden`));
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'tablist' && node.name?.value === 'Project views'));
  const host = "document.getElementById('overflow-tabs')";
  const strip = `${host}.querySelector('nav')`;
  const first = `${strip}.querySelector('[data-tab-value=summary]')`;
  const last = `${strip}.querySelector('[data-tab-value=settings]')`;
  const hasFades = (left, right) => `(() => {
    const strip = ${strip};
    return strip?.hasAttribute('data-tab-overflow-left') === ${left} &&
      strip.hasAttribute('data-tab-overflow-right') === ${right};
  })()`;
  const mask = `getComputedStyle(${strip}).maskImage`;
  const expectFades = async (left, right) => {
    await waitFor(hasFades(left, right));
    assert.equal(await evaluate(`${mask} !== 'none'`), left || right);
  };
  await expectFades(false, true);
  await evaluate(`${strip}.scrollLeft = (${strip}.scrollWidth - ${strip}.clientWidth) / 2`);
  await expectFades(true, true);
  await evaluate(`${first}.focus()`);
  await key('End', 35);
  await waitFor(`document.activeElement === ${last}`);
  await expectFades(true, false);
  assert.equal(await evaluate(`${first}.getAttribute('aria-current')`), 'true');
  assert.equal(await evaluate(`${strip}.querySelectorAll('[tabindex="0"]').length`), 1);
  assert.ok(await evaluate(`(() => {
    const stop = ${last};
    const viewport = ${strip}.getBoundingClientRect();
    const style = getComputedStyle(stop);
    const ring = parseFloat(style.outlineOffset) + parseFloat(style.outlineWidth);
    return stop.matches(':focus-visible') && style.outlineStyle !== 'none' &&
      stop.getBoundingClientRect().right + ring <= viewport.right + 1;
  })()`));
  await key('Enter', 13, '\r');
  await waitFor(`${last}.getAttribute('aria-current') === 'true'`);
  await send('Emulation.setEmulatedMedia', {features: [{name: 'forced-colors', value: 'active'}]});
  await waitFor(`${mask} === 'none'`);
  assert.equal(await evaluate(`getComputedStyle(${last}.querySelector('.astryx-tab-indicator')).borderBlockEndStyle`), 'solid');
  assert.notEqual(await evaluate(`getComputedStyle(${last}).outlineStyle`), 'none');
  await send('Emulation.setEmulatedMedia', {features: []});
  await key('Home', 36);
  await waitFor(`document.activeElement === ${first}`);
  await expectFades(false, true);
  await evaluate(`${first}.blur(); ${first}.click()`);
  await waitFor(`${first}.getAttribute('aria-current') === 'true'`);
  await evaluate(`${last}.click()`);
  await expectFades(true, false);
  await evaluate(`${host}.style.inlineSize = 'max-content'`);
  await expectFades(false, false);
  await evaluate(`${host}.dir = 'rtl'; ${host}.style.inlineSize = '16rem'`);
  await expectFades(false, true);
  await evaluate(`${strip}.scrollLeft = 0`);
  await expectFades(true, false);
  assert.deepEqual(exceptions, []);
  console.log('TabList: roving focus, tab semantics, LTR/RTL fades, reveal, resize and forced colors passed');
});
