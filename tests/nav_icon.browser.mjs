import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8150'});
  await waitFor(`document.querySelectorAll('#nav-icon-examples .astryx-nav-icon').length === 5`);
  for (const value of ['light', 'dark']) {
    await send('Emulation.setEmulatedMedia', {features: [{name: 'prefers-color-scheme', value}]});
    const results = await evaluate(`(() => {
      const section = document.getElementById('nav-icon-examples');
      const probe = document.createElement('span');
      probe.style.cssText = 'background-color:var(--color-accent);color:var(--color-on-accent)';
      section.append(probe);
      const expected = getComputedStyle(probe);
      const marks = [...section.querySelectorAll('.astryx-nav-icon')];
      const samples = marks.map(mark => {
        const style = getComputedStyle(mark);
        const box = mark.getBoundingClientRect();
        const glyph = mark.querySelector('svg');
        const rect = glyph.getBoundingClientRect();
        return [box.width, box.height, style.borderRadius, style.flexShrink,
          style.backgroundColor === expected.backgroundColor, getComputedStyle(glyph).stroke === expected.color,
          Math.abs(rect.left + rect.width / 2 - box.left - box.width / 2) < 0.02,
          Math.abs(rect.top + rect.height / 2 - box.top - box.height / 2) < 0.02,
          mark.tabIndex, mark.getAttribute('role'), mark.getAttribute('aria-hidden')];
      });
      probe.remove();
      return [samples, marks[1].getBoundingClientRect().left - marks[0].getBoundingClientRect().right];
    })()`);
    assert.deepEqual(results, [Array.from({length: 5}, () => [32, 32, '50%', '0', true, true, true, true, -1, null, null]), 16]);
  }
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.name?.value === 'Acme Platform' && node.role?.value === 'link'));
  assert.deepEqual(await evaluate(`[...document.querySelectorAll('#top-nav-heading-examples nav > *')].map(node => [!!node.querySelector('.astryx-nav-icon'), getComputedStyle(node).paddingInlineStart])`), [[true, '0px'], [true, '0px'], [false, '8px']]);
  assert.deepEqual(exceptions, []);
  console.log('NavIcon: five centered marks, theme token paint, inert roots, and TopNavHeading composition passed');
});
