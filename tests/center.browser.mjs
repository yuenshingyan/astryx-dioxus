import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8114'});
  await waitFor('!!document.getElementById("center-showcase")');
  await evaluate('document.fonts.ready.then(() => true)');
  assert.deepEqual(await evaluate(`Array.from(document.querySelectorAll('#center-formatting button'), e => e.getAttribute('aria-label'))`), ['Bold','Italic','Underline','List','Link','Image']);
  const cases = await evaluate(`['center-formatting','center-empty-state','center-showcase'].map(id => {
    const actual = document.getElementById(id);
    const reference = actual.cloneNode(true);
    reference.removeAttribute('id');
    reference.removeAttribute('class');
    reference.style.cssText = id === 'center-formatting'
      ? 'display:flex;justify-content:center;width:100%'
      : 'display:flex;justify-content:center;align-items:center;height:' + (id === 'center-showcase' ? '240px;width:100%' : '200px');
    actual.after(reference);
    const measure = root => {
      const r = root.getBoundingClientRect(), child = root.firstElementChild.getBoundingClientRect();
      const s = getComputedStyle(root);
      return [r.width,r.height,child.x-r.x,child.y-r.y,child.width,child.height,s.display,s.alignItems,s.justifyContent];
    };
    const result = [measure(actual),measure(reference)];
    reference.remove();
    return result;
  })`);
  for (const [actual, reference] of cases) assert.deepEqual(actual, reference);
  const modes = await evaluate(`(() => {
    const root = document.getElementById('center-formatting');
    const result = [];
    for (const dir of ['ltr','rtl']) for (const mode of ['horizontal-tb','vertical-rl']) {
      root.dir = dir; root.style.writingMode = mode;
      const s = getComputedStyle(root);
      result.push([s.justifyContent,s.alignItems]);
    }
    return result;
  })()`);
  assert.deepEqual(modes, Array.from({length:4}, () => ['center','normal']));
  const states = await evaluate(`(() => {
    const vertical = document.getElementById('center-vertical');
    const result = [];
    for (const mode of ['horizontal-tb', 'vertical-rl']) for (const dir of ['ltr', 'rtl']) {
      vertical.style.writingMode = mode;
      vertical.dir = dir;
      const s = getComputedStyle(vertical);
      result.push([s.justifyContent, s.alignItems, s.paddingTop, s.paddingRight, s.paddingBottom, s.paddingLeft]);
    }
    const inline = getComputedStyle(document.getElementById('center-inline'));
    const inherit = document.getElementById('center-inherited-size');
    const dimensions = node => {
      const s = getComputedStyle(node);
      return [s.width, s.maxWidth, s.minHeight];
    };
    return { result, inline: [inline.display, inline.justifyContent, inline.alignItems],
      inherit: dimensions(inherit), parent: dimensions(inherit.parentElement),
      rem: parseFloat(getComputedStyle(document.documentElement).fontSize) };
  })()`);
  const padding = (...factors) => ['normal', 'center', ...factors.map(factor => `${factor * states.rem}px`)];
  assert.deepEqual(states.result, [
    padding(.25, .5, .75, 0), padding(.25, 0, .75, .5),
    padding(0, .25, .5, .75), padding(.5, .25, 0, .75),
  ]);
  assert.deepEqual(states.inline, ['inline-flex', 'center', 'center']);
  assert.deepEqual(states.inherit, states.parent);
  console.log('Center: three patterns match source-derived flex geometry; axis, padding, inline display and inherited dimensions passed');
});
