import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8121'});
  await waitFor('!!document.getElementById("radio-replacement")');
  const values = await evaluate(`['unchecked','checked','disabled','replacement'].map(id => {
    const root = document.getElementById('radio-'+id), dot = root.querySelector('.astryx-radio-indicator-dot');
    const css = getComputedStyle(root);
    return [css.width, css.height, css.borderRadius, css.opacity, dot ? getComputedStyle(dot).width : null, root.getAttribute('aria-hidden')];
  })`);
  assert.deepEqual(values, [
    ['24px','24px','50%','1',null,'true'], ['20px','20px','50%','1','8px','true'],
    ['24px','24px','50%','0.5',null,'true'], ['24px','24px','50%','1',null,'true'],
  ]);
  assert.equal(await evaluate('document.getElementById("radio-replacement").textContent.trim()'), '…');
  await send('Emulation.setEmulatedMedia', {features: [{name:'prefers-reduced-motion', value:'reduce'}]});
  assert.equal(await evaluate(`getComputedStyle(document.getElementById('radio-checked')).transitionDuration`), '0s');
  assert.equal(await evaluate('matchMedia("(hover:hover)").matches'), true, 'desktop preview supports hover');
  {
    const baseline = await evaluate(`getComputedStyle(document.getElementById('radio-checked')).backgroundColor`);
    const point = await evaluate(`(() => { const el = document.getElementById('radio-checked'); el.scrollIntoView({block:'center'}); const r = el.getBoundingClientRect(); return {x:r.x+10,y:r.y+10}; })()`);
    await send('Input.dispatchMouseEvent', {type:'mouseMoved', ...point});
    await waitFor(`getComputedStyle(document.getElementById('radio-checked')).backgroundColor !== ${JSON.stringify(baseline)}`);
    await evaluate(`document.getElementById('radio-indicator-scope').removeAttribute('data-astryx-indicator-scope')`);
    assert.equal(await evaluate(`getComputedStyle(document.getElementById('radio-checked')).backgroundColor`), baseline, 'an unmarked host does not tint the circle');
  }
  await send('Emulation.setEmulatedMedia', {features: [{name:'forced-colors', value:'active'}]});
  assert.equal(await evaluate(`(() => { const circle = document.getElementById('radio-checked'); return getComputedStyle(circle).backgroundColor !== getComputedStyle(circle.firstElementChild).backgroundColor; })()`), true, 'selected dot is distinct from the circle fill in forced colors');
  console.log('RadioIndicator: state geometry, scoped hover, reduced motion and forced-colors selection passed');
});
