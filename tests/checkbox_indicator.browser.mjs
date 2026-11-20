import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8122'});
  await waitFor('!!document.getElementById("ci-replacement")');
  const actual = await evaluate(`['unchecked','checked','mixed','disabled','replacement'].map(id => {
    const root = document.getElementById('ci-'+id), css = getComputedStyle(root);
    const marks = [...root.children].filter(mark => getComputedStyle(mark).display !== 'none');
    return [css.width, css.height, css.opacity, root.getAttribute('aria-hidden'), marks.map(mark => {
      const box = mark.getBoundingClientRect(); return [box.width,box.height];
    })];
  })`);
  assert.deepEqual(actual, [
    ['24px','24px','1','true',[]], ['24px','24px','1','true',[[14,14]]],
    ['20px','20px','1','true',[[10,2]]], ['24px','24px','0.5','true',[]], ['24px','24px','1','true',[]],
  ]);
  assert.equal(await evaluate('document.getElementById("ci-replacement").textContent.trim()'), '…');
  await send('Emulation.setEmulatedMedia', {features: [{name:'prefers-reduced-motion', value:'reduce'}]});
  const paint = id => evaluate(`getComputedStyle(document.getElementById('${id}')).backgroundColor`);
  assert.equal(await evaluate(`getComputedStyle(document.getElementById('ci-checked')).transitionDuration`), '0s');
  const baseline = await paint('ci-checked');
  assert.equal(await evaluate('matchMedia("(hover:hover)").matches'), true);
  const point = await evaluate(`(() => {const item=document.getElementById('ci-checked');item.scrollIntoView({block:'center'});const r=item.getBoundingClientRect();return {x:r.x+10,y:r.y+10};})()`);
  await send('Input.dispatchMouseEvent', {type:'mouseMoved', ...point});
  await waitFor(`getComputedStyle(document.getElementById('ci-checked')).backgroundColor !== ${JSON.stringify(baseline)}`);
  await evaluate(`document.getElementById('checkbox-indicator-scope').removeAttribute('data-astryx-indicator-scope')`);
  assert.equal(await paint('ci-checked'), baseline);
  await send('Emulation.setEmulatedMedia', {features: [{name:'forced-colors', value:'active'}]});
  for (const [id, selector, property] of [['ci-checked','svg','color'],['ci-mixed','.astryx-checkbox-indicator-dash','backgroundColor']]) {
    assert.equal(await evaluate(`(() => {const box=document.getElementById('${id}');return getComputedStyle(box).backgroundColor !== getComputedStyle(box.querySelector('${selector}')).${property};})()`), true, id);
  }
  console.log('CheckboxIndicator: state geometry, replacement, scoped hover, reduced motion and forced colors passed');
});
