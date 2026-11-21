import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8159'});
  await waitFor(`!!document.getElementById('field-email')`);
  assert.deepEqual(await evaluate(`(() => {
    const input = document.getElementById('field-email');
    const aligned = document.getElementById('field-contact').closest('.astryx-field');
    const desc = document.getElementById('field-contact-desc');
    const detached = document.getElementById('field-handle-status');
    return [input.labels[0].textContent.startsWith('Email'), input.closest('.astryx-field').offsetWidth,
      getComputedStyle(aligned).display, desc.parentElement.contains(document.getElementById('field-contact')),
      detached.dataset.variant];
  })()`), [true, 320, 'contents', true, 'detached']);
  await evaluate(`document.getElementById('field-email-desc').click()`);
  assert.equal(await evaluate(`document.activeElement.id`), 'field-email');
  await send('Input.insertText', {text: 'invalid'});
  await waitFor(`document.getElementById('field-email-status')?.textContent.includes('Enter a valid email address.')`);
  assert.equal(await evaluate(`getComputedStyle(document.getElementById('field-email-status')).zIndex`), '-1');
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'textbox' && node.name?.value.startsWith('Email') && node.description?.value.includes('Enter a valid email address.')));
  await send('Input.insertText', {text: '@example.com'});
  await waitFor(`!document.getElementById('field-email-status')`);
  assert.equal(await evaluate(`document.activeElement.id`), 'field-email');
  const help = `document.querySelector('label[for="field-password"] + button')`;
  await waitFor(`!!document[Symbol.for('astryx.dioxus.button.runtime')]?.records.get(${help})`);
  assert.equal(await evaluate(`${help}.getAttribute('aria-label')`), 'About password requirements');
  assert.equal(await evaluate(`document.getElementById('field-password').getAttribute('aria-describedby')`), 'field-password-desc');
  await evaluate(`${help}.click()`);
  await waitFor(`${help}.getAttribute('aria-expanded') === 'true'`);
  assert.equal(await evaluate(`document.activeElement.id`), 'field-email');
  assert.deepEqual(exceptions, []);
  console.log('Field: label and description, attached and detached status, horizontal layout, and retained focus passed');
});
