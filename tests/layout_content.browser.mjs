import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8126'});
  await waitFor('document.getElementById("layout-content-scroll")?.getAttribute("role") === "group"');
  assert.deepEqual(await evaluate(`['basic','showcase'].map(id=>{
    const root=document.getElementById('layout-content-'+id);
    return [root.getAttribute('role'),root.getAttribute('aria-label'),getComputedStyle(root).overflow];
  })`), [['region','Main Content','auto'],['region','Main Content Area','auto']]);
  assert.equal(await evaluate(`(()=>{const root=document.getElementById('layout-content-scroll');root.scrollTop=30;return root.scrollTop===30&&root.getAttribute('aria-label')==='Scrollable example'})()`), true);
  await evaluate('document.getElementById("layout-content-toggle").click()');
  await waitFor(`!document.getElementById('layout-content-scroll').hasAttribute('role') && !document.getElementById('layout-content-scroll').hasAttribute('aria-label')`);
  const edges = await evaluate(`(() => {
    const root = document.getElementById('layout-content-basic');
    const host = document.createElement('div');
    host.style.cssText='width:400px;container-type:inline-size;--layout-alignment-width:300px;--layout-padding-own-outer-x:20px;--layout-padding-outer-y:12px;--layout-padding-inner-x:8px;--layout-padding-inner-y:4px;--_layout-flush-block-start:0px';
    root.before(host); host.append(root);
    const measure = () => {
      const css = getComputedStyle(root);
      return [css.paddingInlineStart, css.paddingInlineEnd, css.paddingBlockStart, css.paddingBlockEnd,
        css.getPropertyValue('--container-padding-inline-end').trim()];
    };
    const open = measure();
    root.classList.remove('astryx-layout-content--open-start', 'astryx-layout-content--open-header');
    const onePanel = measure();
    root.classList.add('astryx-layout-content--full-bleed');
    root.style.setProperty('--_layout-content-padding','0px');
    return [open, onePanel, measure()];
  })()`);
  assert.deepEqual(edges, [['70px','70px','12px','12px','20px'],['8px','120px','0px','12px','20px'],['0px','0px','0px','0px','0px']]);
  console.log('LayoutContent: arrangements, scroll naming, slot insets, alignment gutters, and full bleed passed');
});
