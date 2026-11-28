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
  assert.deepEqual(await evaluate(`(() => {
    const circle = document.querySelector('#radio-source-example .astryx-radio-indicator');
    const box = getComputedStyle(circle), dot = getComputedStyle(circle.firstElementChild);
    return [box.getPropertyValue('--radius-full').trim(), box.width, box.height, box.borderRadius, dot.width, dot.height, dot.borderRadius];
  })()`), ['0px', '24px', '24px', '50%', '10px', '10px', '50%']);
  assert.deepEqual(await evaluate(`(() => {
    const roots = Array.from(document.querySelectorAll('#radio-restyle .astryx-radio-indicator'));
    const checked = getComputedStyle(roots[1]), disabled = getComputedStyle(roots[2]);
    const probe = document.createElement('span');
    probe.style.backgroundColor = 'var(--color-background-muted)';
    roots[2].append(probe);
    const muted = getComputedStyle(probe).backgroundColor;
    probe.remove();
    return [roots.map(root => getComputedStyle(root).borderTopWidth),
      getComputedStyle(roots[1].firstElementChild).borderRadius,
      disabled.backgroundColor === muted, disabled.backgroundColor !== checked.backgroundColor];
  })()`), [['2px', '2px', '2px'], '2px', true, true]);
  await send('Emulation.setEmulatedMedia', {features: [{name:'prefers-reduced-motion', value:'reduce'}]});
  assert.equal(await evaluate(`getComputedStyle(document.getElementById('radio-checked')).transitionDuration`), '0s');
  assert.equal(await evaluate('matchMedia("(hover:hover)").matches'), true, 'desktop preview supports hover');
  {
    const point = await evaluate(`(() => { const el = document.getElementById('radio-checked'); el.scrollIntoView({block:'center'}); const r = el.getBoundingClientRect(); return {x:r.x+10,y:r.y+10}; })()`);
    await send('Input.dispatchMouseEvent', {type: 'mouseMoved', x: 0, y: 0});
    const paint = `['unchecked', 'checked', 'disabled-checked'].map(id => {
      const css = getComputedStyle(document.getElementById('radio-' + id));
      return [css.backgroundColor, css.borderTopColor, css.opacity];
    })`;
    const baseline = await evaluate(paint);
    await send('Input.dispatchMouseEvent', {type:'mouseMoved', ...point});
    await waitFor(`JSON.stringify(${paint}) !== ${JSON.stringify(JSON.stringify(baseline))}`);
    const hovered = await evaluate(paint);
    assert.notEqual(hovered[0][0], baseline[0][0], 'the unchecked fill tints');
    assert.notEqual(hovered[0][1], baseline[0][1], 'the unchecked border tints');
    assert.notEqual(hovered[1][1], baseline[1][1], 'the enabled checked border tints');
    assert.notEqual(hovered[2][0], baseline[2][0], 'the checked fill tints even when disabled');
    assert.deepEqual(hovered[2], [hovered[1][0], baseline[2][1], '0.5'], 'the disabled checked border and opacity stay muted');
    await evaluate(`document.getElementById('radio-indicator-scope').removeAttribute('data-astryx-indicator-scope')`);
    assert.deepEqual(await evaluate(paint), baseline, 'an unmarked host does not tint the circle');
  }
  await send('Emulation.setEmulatedMedia', {features: [{name:'forced-colors', value:'active'}]});
  assert.equal(await evaluate(`(() => { const circle = document.getElementById('radio-checked'); return getComputedStyle(circle).backgroundColor !== getComputedStyle(circle.firstElementChild).backgroundColor; })()`), true, 'selected dot is distinct from the circle fill in forced colors');
  console.log('RadioIndicator: state geometry, scoped hover, reduced motion and forced-colors selection passed');
});
