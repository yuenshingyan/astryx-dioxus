// Opt-in callback fixture; no event interception or animation-service workaround.
import assert from 'node:assert/strict';
import {openTextPage} from './internal/text_cdp.mjs';
const page = await openTextPage();
try {
  await page.send('Page.navigate', {url: `${process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8090'}/?typography-animation`});
  await page.waitFor('!!document.querySelector("#animation-heading")');
  // Inspect actual registration, without wrapping addEventListener. A listener
  // removed after startup would not restore compositor-only animation servicing.
  const {result} = await page.send('Runtime.evaluate', {expression: 'document.getElementById("main")'});
  const listeners = await page.send('DOMDebugger.getEventListeners', {objectId: result.objectId, depth: -1});
  assert.equal(listeners.listeners.some(listener => listener.type.startsWith('animation')), false);
  let expected = '';
  const dispatch = async (phase) => {
    for (const target of ['text', 'heading']) {
      for (const event of ['start', 'iteration', 'end']) {
        const name = `${target}-${phase}-${event}`;
        await page.evaluate(`document.querySelector('#animation-${target} span').dispatchEvent(new AnimationEvent('animation${event}', {bubbles:true, animationName:${JSON.stringify(name)}}))`);
        if ((phase === 1 && event === 'start') || phase === 2) expected += `${event}:${name};`;
        if (phase === 3 && event === 'iteration') expected += `raw:${name};`;
      }
    }
    await page.waitFor(`document.querySelector('#delivered').textContent === ${JSON.stringify(expected)}`);
    assert.equal(await page.evaluate('document.querySelector("#delivered").textContent'), expected);
  };
  await dispatch(0);
  for (let phase = 1; phase <= 4; phase++) {
    await page.evaluate('document.querySelector("#next").click()');
    await page.waitFor(`document.querySelector('#animation-text')?.dataset.phase === '${phase}' && document.querySelector('#animation-heading')?.dataset.phase === '${phase}'`);
    await dispatch(phase);
  }
  assert.deepEqual(page.errors, []);
  console.log('Typography animation: absent first registration, optional start/iteration/end, bubbled descendant delivery, raw callback forwarding and dynamic removal passed.');
} finally { await page.close(); }
