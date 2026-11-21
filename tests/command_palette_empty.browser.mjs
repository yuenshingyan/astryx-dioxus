import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Emulation.setDeviceMetricsOverride', {width: 320, height: 800, deviceScaleFactor: 1, mobile: false});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8162'});
  await waitFor(`!!document.getElementById('command-palette-empty-wrap')`);
  assert.deepEqual(await evaluate(`(() => {
    const node = document.getElementById('command-palette-empty-wrap');
    const style = getComputedStyle(node);
    const range = document.createRange();
    range.selectNodeContents(node.firstElementChild);
    const bounds = node.getBoundingClientRect();
    const lines = [...range.getClientRects()];
    return [style.justifyContent, style.alignItems, style.textAlign,
      getComputedStyle(node.firstElementChild).overflowWrap,
      lines.length > 1 && lines.every(line => line.left >= bounds.left && line.right <= bounds.right),
      node.scrollWidth === node.clientWidth, node.getAttribute('role'), node.getAttribute('aria-live'), node.tabIndex];
  })()`), ['center', 'center', 'center', 'anywhere', true, true, null, null, -1]);
  const {nodes} = await send('Accessibility.getFullAXTree');
  for (const name of ['No commands available', 'No commands available yet']) {
    assert.ok(nodes.some(node => !node.ignored && node.name?.value === name && node.role?.value === 'StaticText'));
  }
  assert.deepEqual(exceptions, []);
  console.log('CommandPaletteEmpty: centered content, narrow token wrapping, and accessible static text passed');
});
