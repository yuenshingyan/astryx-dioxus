import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8183'});
  await waitFor(`document.querySelectorAll('#chat-message-metadata-examples .astryx-chat-message-metadata').length === 12`);
  const state = await evaluate(`(() => {
    const row = document.getElementById('chat-metadata-read');
    const x = el => el.getBoundingClientRect().left;
    const isReversed = x(row.firstElementChild) > x(row.lastElementChild);
    row.dir = 'rtl';
    const isMirrored = x(row.firstElementChild) < x(row.lastElementChild);
    const error = document.querySelector('#chat-metadata-error [data-status="error"]');
    const probe = document.createElement('span');
    probe.style.color = 'var(--color-text-error)';
    error.append(probe);
    const hasErrorColor = getComputedStyle(error).color === getComputedStyle(probe).color;
    probe.remove();
    const statuses = [...document.querySelectorAll('#chat-message-metadata-examples [data-status]')];
    return {isReversed, isMirrored, hasErrorColor, labels: [...new Set(statuses.map(s => s.textContent))].sort(), isPassive: !document.querySelector('#chat-message-metadata-examples :is(.astryx-chat-message-metadata, .astryx-chat-message-metadata__status):is([aria-live], [role], [tabindex])')};
  })()`);
  assert.deepEqual(state, {isReversed: true, isMirrored: true, hasErrorColor: true, labels: ['Delivered', 'Failed', 'Read', 'Sending', 'Sent'], isPassive: true});
  const animation = `getComputedStyle(document.querySelector('#chat-message-metadata-examples [data-status="sending"]')).animationName`;
  await send('Emulation.setEmulatedMedia', {features: [{name: 'prefers-reduced-motion', value: 'no-preference'}]});
  await waitFor(`${animation} === 'astryx-chat-message-sending'`);
  await send('Emulation.setEmulatedMedia', {features: [{name: 'prefers-reduced-motion', value: 'reduce'}]});
  await waitFor(`${animation} === 'none'`);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'StaticText' && node.name?.value === 'Failed'));
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'button' && node.name?.value === 'Good response'));
  assert.deepEqual(exceptions, []);
  console.log('ChatMessageMetadata: slot previews, status captions, sender order, RTL, and reduced motion passed');
});
