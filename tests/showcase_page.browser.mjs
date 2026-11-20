import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor}) => {
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8080'});
  await waitFor('!!document.getElementById("stack-examples")');
  assert.equal(await evaluate('document.querySelector("main").firstElementChild?.tagName'), 'H1', 'the page heading precedes all examples');
  assert.equal(await evaluate('document.title'), await evaluate('document.querySelector("main > h1").textContent'));

  const menu = 'main > nav[aria-label="Showcase sections"]';
  const ids = await evaluate(`[...document.querySelectorAll('${menu} a')].map(a => a.hash.slice(1))`);
  for (const id of ['aspect-ratio-examples', 'stack-examples']) {
    assert.ok(ids.includes(id), `${id} is accessible from the section navigation`);
  }
  const invalid = await evaluate(`(() => {
    const nav = document.querySelector('${menu}');
    const targets = [...document.querySelectorAll('main [id]')];
    return [...nav.querySelectorAll('a')].map(a => a.hash.slice(1)).filter(id => {
      const matches = targets.filter(target => target.id === id);
      return matches.length !== 1 || !(nav.compareDocumentPosition(matches[0]) & Node.DOCUMENT_POSITION_FOLLOWING);
    });
  })()`);
  assert.deepEqual(invalid, [], 'section links point to unique targets after the navigation');

  await evaluate(`document.querySelector('${menu} a[href="#stack-examples"]').click()`);
  await waitFor('location.hash === "#stack-examples" && document.querySelector(":target")?.id === "stack-examples"');
  console.log('Showcase title order and section navigation: PASS');
});
