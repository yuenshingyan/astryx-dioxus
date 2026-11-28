import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  const url = new URL(process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8218');
  const root = `document.getElementById('payment-banner')`;
  const dismiss = `${root}.querySelector('button[aria-label="Dismiss Payment failed."]')`;
  const origin = `document.getElementById('banner-test-origin')`;
  const open = async scenario => {
    url.searchParams.set('banner-test', scenario);
    await send('Page.navigate', {url: url.href});
    await waitFor(`location.href === ${JSON.stringify(url.href)} && ${root}?.querySelector('[role=alert]')`);
    await evaluate(`(() => {
      const button = document.createElement('button');
      button.id = 'banner-test-origin';
      button.textContent = 'Previous control';
      ${root}.before(button);
      button.addEventListener('focus', () => {
        button.dataset.bannerStillMounted = String(!!${root});
      });
      button.focus();
      delete button.dataset.bannerStillMounted;
    })()`);
  };

  await open('keyboard');
  assert.deepEqual(await evaluate(`Array.from(document.querySelectorAll('#banner-examples [data-status]'), node => node.dataset.status)`), ['info', 'success', 'warning', 'error']);
  assert.equal(await evaluate(`${root}.querySelector('[role=alert] button')`), null);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'alert'));
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'button' && node.name?.value === 'Dismiss Payment failed.'));
  await evaluate(`${root}.querySelector('button').click()`);
  await waitFor(`document.getElementById('banner-demo-result').textContent === 'Retry requested'`);
  await evaluate(`${root}.querySelector('button').focus(); ${dismiss}.focus()`);
  // Moving within the Banner must not replace the external origin.
  await send('Input.dispatchKeyEvent', {type:'keyDown', key:'Enter', code:'Enter', windowsVirtualKeyCode:13, text:'\r'});
  await send('Input.dispatchKeyEvent', {type:'keyUp', key:'Enter', code:'Enter', windowsVirtualKeyCode:13});
  await waitFor(`${root} === null`);
  assert.equal(await evaluate(`document.activeElement === ${origin}`), true);
  assert.equal(await evaluate(`${origin}.dataset.bannerStillMounted`), 'true');

  await open('pointer');
  // Model a pointer press that clears focus before the dismiss button receives it.
  // The later focusin has no valid relatedTarget; pointerdown must supply the origin.
  await evaluate(`
    ${dismiss}.dispatchEvent(new PointerEvent('pointerdown', {bubbles: true, pointerType: 'mouse'}));
    ${origin}.blur();
    ${dismiss}.focus();
    for (const relatedTarget of [document.body, document.createElementNS('http://www.w3.org/2000/svg', 'svg')]) {
      ${dismiss}.dispatchEvent(new FocusEvent('focusin', {bubbles: true, relatedTarget}));
    }
    ${dismiss}.click();
  `);
  await waitFor(`${root} === null`);
  assert.equal(await evaluate(`document.activeElement === ${origin}`), true);

  await open('detached');
  await evaluate(`(() => {
    const previous = ${origin};
    ${dismiss}.focus();
    previous.remove();
    window.detachedOriginWasFocused = false;
    previous.focus = () => { window.detachedOriginWasFocused = true; };
    ${dismiss}.click();
  })()`);
  await waitFor(`${root} === null`);
  assert.equal(await evaluate('window.detachedOriginWasFocused'), false);
  assert.deepEqual(exceptions, []);
  console.log('Banner: status variants, message-only alert, action, and dismissal focus handoff passed');
});
