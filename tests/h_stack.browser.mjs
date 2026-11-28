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
  const frame = await evaluate(`(() => {
    const root = document.getElementById('h-stack-showcase');
    const layout = el => {
      const s = getComputedStyle(el);
      return [s.display, s.flexDirection, s.rowGap];
    };
    return {
      outer: [...layout(root), root.style.width, getComputedStyle(root).maxWidth],
      groups: [...root.children].map(group => [
        ...layout(group), group.children.length,
        group.firstElementChild.textContent.trim(), group.lastElementChild.id,
      ]),
    };
  })()`);
  assert.deepEqual(frame, {
    outer: ['flex', 'column', '24px', '100%', '400px'],
    groups: [
      ['flex', 'column', '8px', 2, 'HAlign: start', 'h-stack-start'],
      ['flex', 'column', '8px', 2, 'HAlign: center', 'h-stack-center'],
      ['flex', 'column', '8px', 2, 'HAlign: between', 'h-stack-between'],
    ],
  });

  await waitFor(`document.getElementById('h-stack-native-root')?.dataset.rootMounted === 'true'`);
  assert.deepEqual(await evaluate(`(() => {
    const root = document.getElementById('h-stack-native-root');
    return [root.tagName, root.parentElement.id, root.getAttribute('aria-label'),
      root.hasAttribute('role'), root.hasAttribute('tabindex'),
      [...root.children].map(child => [child.tagName, child.id])];
  })()`), ['NAV', 'h-stack-examples', 'Example pages', false, false, [
    ['A', 'h-stack-overview'], ['A', 'h-stack-patterns'],
  ]]);
  await evaluate(`document.getElementById('h-stack-overview').focus()`);
  for (const type of ['keyDown', 'keyUp']) {
    await send('Input.dispatchKeyEvent', {type, key: 'Tab', code: 'Tab', windowsVirtualKeyCode: 9});
  }
  assert.equal(await evaluate('document.activeElement.id'), 'h-stack-patterns');
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13, text: '\r'});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`location.hash === '#h-stack-showcase' && document.getElementById('h-stack-native-root').dataset.clicks === '1'`);
  console.log('HStack: catalog frames, RTL order, native root mounting and keyboard activation passed');
});
