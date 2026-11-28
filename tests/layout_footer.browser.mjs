import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8125'});
  await waitFor('!!document.getElementById("layout-footer-actions")');
  assert.deepEqual(await evaluate(`['showcase','actions'].map(id => {
    const footer = document.getElementById('layout-footer-' + id);
    return [footer.dataset.divider, footer.getAttribute('role'),
      [...footer.querySelectorAll('button')].map(button => button.textContent.trim())];
  })`), [['true',null,['Cancel','Save']], ['true',null,['Cancel','Save']]]);
  const slots = await evaluate(`['showcase','actions'].map(id => {
    const footer = document.getElementById('layout-footer-' + id);
    const middle = footer.previousElementSibling;
    const content = middle.querySelector('.astryx-layout-content');
    const before = footer.getBoundingClientRect().top;
    content.scrollTop = 32;
    return [footer.parentElement.matches('.astryx-layout__inner'),
      middle.matches('.astryx-layout__middle'),
      !!middle.querySelector('.astryx-layout-panel--start'),
      content.scrollTop > 0, footer.getBoundingClientRect().top === before];
  })`);
  assert.deepEqual(slots, [[true,true,true,true,true], [true,true,false,true,true]]);
  for (const [index, message] of ['Demo: your changes would be discarded here', 'Demo: your changes would be saved here'].entries()) {
    await evaluate(`document.querySelectorAll('#layout-footer-actions button')[${index}].click()`);
    await waitFor(`document.getElementById('layout-footer-actions').closest('.astryx-layout').parentElement.nextElementSibling.textContent === ${JSON.stringify(message)}`);
  }
  const geometry = await evaluate(`(() => {
    const footer = document.getElementById('layout-footer-showcase');
    const inner = footer.firstElementChild;
    const band = footer.getBoundingClientRect();
    const cap = Math.floor(band.width / 2);
    footer.style.setProperty('--layout-content-width', cap + 'px');
    const box = inner.getBoundingClientRect();
    const isCentered = Math.abs((box.left - band.left) - (band.right - box.right)) < 1;
    const isCapped = Math.abs(box.width - cap) < 1 && cap < band.width;
    footer.style.cssText = 'writing-mode:vertical-rl;direction:rtl;--layout-content-width:300px;--layout-padding-own-outer-x:20px;--layout-padding-own-outer-y:16px;--layout-padding-outer-x:8px;--layout-padding-outer-y:12px;--layout-padding-inner-y:4px';
    const probe = document.createElement('span');
    probe.setAttribute('data-container-corner', '');
    probe.style.color = 'var(--color-border)';
    inner.append(probe);
    const measure = () => {
      const css = getComputedStyle(inner);
      const edges = ['inline-start','inline-end','block-start','block-end'];
      return [edges.map(edge => css.getPropertyValue('padding-' + edge)),
        edges.map(edge => css.getPropertyValue('--container-padding-' + edge).trim()),
        css.getPropertyValue('--_container-bleed-block-end').trim()];
    };
    const inherited = measure();
    footer.style.removeProperty('--layout-padding-own-outer-x');
    footer.style.removeProperty('--layout-padding-own-outer-y');
    const fallback = measure();
    inner.style.setProperty('--_layout-footer-padding', '6px');
    const explicit = measure();
    inner.style.setProperty('--_layout-footer-padding', '0px');
    const outer = getComputedStyle(footer);
    return {isCentered, isCapped, inherited, fallback, explicit, zero: measure(), maxWidth: getComputedStyle(inner).maxWidth,
      divider: [outer.borderRightWidth, outer.borderRightStyle, outer.borderLeftWidth,
        outer.borderRightColor === getComputedStyle(probe).color], shrink: outer.flexShrink};
  })()`);
  const insets = (inline, start, end) => [[inline,inline,start,end], [inline,inline,start,end], end];
  assert.deepEqual(geometry, {
    isCentered: true, isCapped: true,
    inherited: insets('20px','4px','16px'), fallback: insets('8px','4px','12px'),
    explicit: insets('6px','6px','6px'), zero: insets('0px','0px','0px'),
    maxWidth: '300px', divider: ['1px','solid','0px',true], shrink: '0',
  });
  console.log('LayoutFooter: native slots, scroll pinning, action messages and computed logical insets and divider passed');
});
