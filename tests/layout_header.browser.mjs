import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8124'});
  await waitFor('!!document.getElementById("layout-header-showcase")');
  assert.deepEqual(await evaluate(`['showcase', 'actions'].map(id => {
    const header = document.getElementById('layout-header-' + id);
    return [header.getAttribute('data-divider'), header.getAttribute('role'),
      [...header.querySelectorAll('button')].map(button => button.textContent.trim())];
  })`), [['true', null, ['Export', 'New Item']], ['true', null, ['New Item']]]);
  await evaluate(`document.querySelector('#layout-header-showcase button').click()`);
  await waitFor(`document.querySelector('#layout-header-showcase').parentElement.querySelector('output').textContent === 'Demo: an export would start here'`);
  const geometry = await evaluate(`(() => {
    const header = document.getElementById('layout-header-showcase');
    const inner = header.firstElementChild;
    header.style.cssText = 'writing-mode:vertical-rl;--layout-content-width:300px;--layout-padding-own-outer-x:20px;--layout-padding-outer-x:8px;--layout-padding-outer-y:12px;--layout-padding-inner-y:4px';
    const probe = document.createElement('span');
    probe.setAttribute('data-container-corner', '');
    inner.append(probe);
    const measure = () => {
      const style = getComputedStyle(inner);
      return [style.maxWidth, style.paddingInlineStart, style.paddingBlockStart, style.paddingBlockEnd,
        style.getPropertyValue('--_container-bleed-block-end').trim()];
    };
    const inherited = measure();
    inner.style.setProperty('--_layout-header-padding-end', '0px');
    const override = measure();
    return [inherited, override, getComputedStyle(header).borderLeftWidth, getComputedStyle(header).flexShrink];
  })()`);
  assert.deepEqual(geometry, [['300px','20px','12px','4px','4px'], ['300px','20px','12px','0px','0px'], '1px', '0']);
  console.log('LayoutHeader: both patterns, action message, edge insets, width cap, bleed and logical divider passed');
});
