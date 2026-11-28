import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8240'});
  const shell = `document.querySelector('#app-shell-examples .astryx-app-shell')`;
  const skip = `${shell}.querySelector('.astryx-app-shell-skip')`;
  const main = `${shell}.querySelector('[role=main]')`;
  await waitFor(`${shell}?.querySelector('[role=main]')`);
  assert.equal(await evaluate(`${skip}.hash`), await evaluate(`'#' + ${main}.id`));
  await evaluate(`${skip}.focus()`);
  await send('Input.dispatchKeyEvent', {type: 'keyDown', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13, text: '\r'});
  await send('Input.dispatchKeyEvent', {type: 'keyUp', key: 'Enter', code: 'Enter', windowsVirtualKeyCode: 13});
  await waitFor(`document.activeElement === ${main}`);
  assert.ok(await evaluate(`${main}.scrollHeight > ${main}.clientHeight`));
  await evaluate(`${main}.scrollTop = 100`);
  assert.ok(await evaluate(`${main}.scrollTop > 0`));
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'main'));
  const insets = await evaluate(`(() => {
    const example = document.querySelector('#app-shell-inset-example');
    const shell = example.querySelector('.astryx-app-shell');
    const layout = getComputedStyle(shell.querySelector(':scope > .astryx-layout'));
    const nested = shell.querySelector('[role=main] .astryx-section');
    const control = example.querySelector('.astryx-section > .astryx-section-wrapper > .astryx-section');
    const padding = node => getComputedStyle(node).paddingInlineStart;
    const defaultInset = padding(nested);
    example.style.setProperty('--astryx-section-padding', '20px');
    return {
      layoutMargins: [layout.marginInlineStart, layout.marginInlineEnd, layout.marginBlockStart, layout.marginBlockEnd],
      defaultInset,
      themedInset: padding(nested),
      inheritedInset: padding(control),
    };
  })()`);
  assert.deepEqual(insets, {
    layoutMargins: ['0px', '0px', '0px', '0px'],
    defaultInset: '16px',
    themedInset: '20px',
    inheritedInset: '24px',
  });
  assert.deepEqual(exceptions, []);
  console.log('AppShell: skip focus, main landmark, bounded scrolling and nested Section insets passed');
});
