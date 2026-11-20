import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8129'});
  const channel = priority => `document.querySelector('[data-astryx-live-region="${priority}"]')`;
  const current = `document.getElementById('field-status-current')`;
  const click = () => evaluate(`document.getElementById('field-status-change').click()`);
  await waitFor(`${channel('assertive')}?.textContent.includes('This field is required')`);
  await waitFor(`${channel('polite')}?.textContent.includes('Your changes have been saved')`);
  assert.deepEqual(await evaluate(`(() => {
    const nodes = [...document.querySelectorAll('#field-status-examples .astryx-field-status[data-variant="detached"]')];
    return nodes.map(node => [node.dataset.type, node.querySelector('.astryx-field-status-icon').getAttribute('aria-hidden'),
      [...node.querySelectorAll('svg')].filter(svg => getComputedStyle(svg).display !== 'none').length,
      node.hasAttribute('aria-live')]);
  })()`), ['error','success','error','warning','success'].map(type => [type, 'true', 1, false]));
  await click();
  await waitFor(`${channel('polite')}.textContent.includes('Review your postal code')`);
  assert.deepEqual(await evaluate(`(() => {
    const css = getComputedStyle(${current});
    return [css.pointerEvents, parseFloat(css.marginTop) < 0, !!${current}.querySelector('svg'),
        document.getElementById('field-status-input').getAttribute('aria-describedby')];
  })()`), ['none', true, false, 'field-status-current']);
  await click();
  await waitFor(`${channel('assertive')}.textContent === 'Review your postal code'`);
  assert.equal(await evaluate(`${channel('assertive')}.getAttribute('role')`), 'alert');
  await click();
  await waitFor(`${channel('assertive')}.textContent === 'Enter a valid postal code'`);
  await click();
  await waitFor(`${current}.textContent.trim() === ''`);
  await click();
  await waitFor(`${current} === null`);
  await send('Emulation.setEmulatedMedia', {features: [{name: 'forced-colors', value: 'active'}]});
  assert.equal(await evaluate(`getComputedStyle(document.querySelector('.astryx-field-status')).outlineStyle`), 'solid');
  assert.deepEqual(exceptions, []);
  console.log('FieldStatus: both arrangements, decorative glyph, overlap, priority, mount and message changes, empty status, and forced colors passed');
});
