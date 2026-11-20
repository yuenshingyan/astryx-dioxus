import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8116'});
  await waitFor('!!document.getElementById("h-stack-between")');
  for (const dir of ['ltr','rtl']) {
    const rows = await evaluate(`['basic','start','center','between'].map(id => {
      const root = document.getElementById('h-stack-'+id);
      root.dir = '${dir}';
      const s = getComputedStyle(root);
      const children = [...root.children];
      const positions = children.map(el => el.getBoundingClientRect().left);
      return [s.display,s.flexDirection,s.justifyContent,s.alignItems,s.columnGap,
        children.map(el => el.textContent.trim()),positions.every((x,i) => !i || (${dir === 'rtl' ? 'x < positions[i-1]' : 'x > positions[i-1]'}))];
    })`);
    assert.deepEqual(rows, [
      ['flex','row','normal','center','8px',['React','TypeScript','Node.js'],true],
      ['flex','row','normal','normal','8px',['React','TypeScript','Node.js'],true],
      ['flex','row','center','normal','16px',['Design','Engineering','Product'],true],
      ['flex','row','space-between','normal','8px',['Start','Middle','End'],true],
    ]);
  }
  console.log('HStack: both original patterns, alignment, gaps and RTL DOM order passed');
});
