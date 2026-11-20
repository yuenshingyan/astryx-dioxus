import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8118'});
  await waitFor('!!document.querySelector("#section-wash-highlight .astryx-button")');
  assert.deepEqual(await evaluate(`['variants','with-dividers','wash-highlight'].map(id =>
    [...document.querySelectorAll('#section-'+id+' .astryx-section')].map(surface => {
      const css = getComputedStyle(surface);
      return [surface.dataset.variant, css.paddingInlineStart, css.borderBottomWidth];
    }))`), [
    [['section','20px','0px'],['muted','20px','0px'],['transparent','20px','0px']],
    [['section','20px','1px'],['section','20px','1px'],['section','20px','0px']],
    [['section','16px','0px'],['muted','24px','0px']],
  ]);
  const geometry = await evaluate(`(() => {
    const wrapper = document.querySelector('#section-with-dividers .astryx-section-wrapper');
    const surface = wrapper.firstElementChild;
    const host = document.createElement('div');
    host.style.cssText='--container-padding-inline-start:20px;--container-padding-inline-end:8px;--container-padding-block-start:16px;--container-padding-block-end:12px;direction:rtl';
    wrapper.before(host); host.append(wrapper);
    wrapper.style.height='125px';
    surface.classList.add('astryx-section--divider-start');
    const css = getComputedStyle(wrapper), inner = getComputedStyle(surface);
    return [css.marginRight, css.marginLeft, css.marginTop, css.marginBottom, inner.borderRightWidth, surface.getBoundingClientRect().height];
  })()`);
  assert.deepEqual(geometry, ['-20px','-8px','-16px','-12px','1px',125]);
  const nested = await evaluate(`(() => {
    const parent = document.querySelector('#section-wash-highlight .astryx-section');
    const child = document.querySelector('#section-with-dividers .astryx-section-wrapper').cloneNode(true);
    child.removeAttribute('style');
    child.firstElementChild.removeAttribute('style');
    parent.style.setProperty('--_section-padding-propagated','24px');
    parent.append(child);
    const inset = () => getComputedStyle(child.firstElementChild).paddingInlineStart;
    const inherited = inset();
    parent.style.setProperty('--astryx-section-padding-inline-start','6px');
    const themed = inset();
    parent.style.removeProperty('--astryx-section-padding-inline-start');
    const boundary = document.createElement('div');
    boundary.setAttribute('data-astryx-layer-root','');
    child.before(boundary); boundary.append(child);
    return [inherited,themed,inset()];
  })()`);
  assert.deepEqual(nested, ['24px','6px','16px']);
  assert.equal(exceptions.length,0);
  console.log('Section: three patterns, nested padding, overlay reset, explicit height and RTL divider/bleed passed');
});
