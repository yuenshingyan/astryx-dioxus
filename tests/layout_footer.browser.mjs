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
  for (const [index, message] of ['Demo: your changes would be discarded here', 'Demo: your changes would be saved here'].entries()) {
    await evaluate(`document.querySelectorAll('#layout-footer-actions button')[${index}].click()`);
    await waitFor(`document.getElementById('layout-footer-actions').parentElement.querySelector('output').textContent === ${JSON.stringify(message)}`);
  }
  const geometry = await evaluate(`(() => {
    const footer = document.getElementById('layout-footer-showcase');
    const inner = footer.firstElementChild;
    footer.style.cssText = 'writing-mode:vertical-rl;--layout-content-width:300px;--layout-padding-own-outer-x:20px;--layout-padding-outer-x:8px;--layout-padding-outer-y:12px;--layout-padding-inner-y:4px';
    const probe = document.createElement('span');
    probe.setAttribute('data-container-corner', '');
    inner.append(probe);
    const measure = () => {
      const css = getComputedStyle(inner);
      return [css.maxWidth, css.paddingInlineStart, css.paddingBlockStart, css.paddingBlockEnd,
        css.getPropertyValue('--_container-bleed-block-end').trim()];
    };
    const inherited = measure();
    inner.style.setProperty('--_layout-footer-padding', '0px');
    return [inherited, measure(), getComputedStyle(footer).borderRightWidth, getComputedStyle(footer).flexShrink];
  })()`);
  assert.deepEqual(geometry, [['300px','20px','4px','12px','12px'], ['300px','0px','0px','0px','0px'], '1px', '0']);
  console.log('LayoutFooter: both arrangements, action messages, inherited insets, zero padding, bleed and logical divider passed');
});
