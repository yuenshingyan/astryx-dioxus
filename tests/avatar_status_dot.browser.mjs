import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8123'});
  await waitFor('!!document.getElementById("avatar-dot-busy")');
  for (const scheme of ['light','dark']) {
    const actual = await evaluate(`(() => {
      document.getElementById('avatar-status-dot-examples').style.colorScheme = '${scheme}';
      return ['online','away','busy'].map(id => {
        const root = document.getElementById('avatar-dot-'+id);
        const style = getComputedStyle(root);
        const glyph = root.querySelector('svg');
        return [root.offsetWidth,root.offsetHeight,style.borderTopWidth,style.backgroundColor,
          glyph?.dataset.shape ?? null,glyph?.getAttribute('aria-hidden') ?? null];
      });
    })()`);
    const paint = scheme === 'light' ? ['rgb(25, 129, 0)','rgb(255, 255, 255)','rgb(201, 48, 58)'] : ['rgb(100, 175, 76)','rgb(38, 38, 38)','rgb(255, 112, 93)'];
    assert.deepEqual(actual, [[10,10,'1px',paint[0],null,null],[10,10,'1px',paint[1],'ring','true'],[10,10,'1px',paint[2],'minus','true']]);
  }
  await send('Accessibility.enable');
  const {nodes} = await send('Accessibility.getFullAXTree');
  for (const name of ['Online','Away','Do not disturb']) {
    assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'image' && node.name?.value === name), name);
  }
  console.log('AvatarStatusDot: standalone geometry, shape, theme colors, and accessible names passed');
});
