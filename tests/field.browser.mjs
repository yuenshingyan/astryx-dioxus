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
      desc.dataset.type, getComputedStyle(desc).display, detached.dataset.variant];
  })()`), [true, 320, 'contents', true, 'supporting', 'block', 'detached']);
  await evaluate(`document.getElementById('field-email-desc').click()`);
  assert.equal(await evaluate(`document.activeElement.id`), 'field-email');
  await send('Input.insertText', {text: 'invalid'});
  await waitFor(`document.getElementById('field-email-status')?.textContent.includes('Enter a valid email address.')`);
  assert.equal(await evaluate(`getComputedStyle(document.getElementById('field-email-status')).zIndex`), '-1');
  const axNodes = async () => (await send('Accessibility.getFullAXTree')).nodes.filter(node => !node.ignored);
  const email = (await axNodes()).find(node => node.role?.value === 'textbox' &&
    node.description?.value === 'We will never share your email. Enter a valid email address.');
  assert.equal(email?.name?.value, 'Email Required');
  assert.equal(email?.properties.find(({name}) => name === 'required')?.value.value, true);
  assert.equal(email?.properties.find(({name}) => name === 'invalid')?.value.value, 'true');
  await waitFor(`document.querySelector('[data-astryx-live-region="assertive"]')?.textContent.includes('Enter a valid email address.')`);
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
  const volume = `document.getElementById('field-volume')`;
  const volumeFeedback = `document.getElementById('field-volume-feedback')`;
  const slider = async () => (await axNodes()).find(node => node.role?.value === 'slider' &&
    node.description?.value === 'Adjust the playback volume. Volume can be adjusted.');
  assert.deepEqual(await evaluate(`[${volume}.labels[0].textContent, ${volume}.getAttribute('aria-describedby'),
    ${volumeFeedback}.dataset.variant, ${volumeFeedback}.hasAttribute('aria-live'),
    ${volume}.hasAttribute('aria-invalid')]`),
  ['Volume', 'field-volume-help field-volume-feedback', 'detached', false, false]);
  assert.equal((await slider())?.name?.value, 'Volume');
  // Emulate caller overrides on the native child; Field must not fold help or
  // status into its name. CDP reports the computed tree, not spoken output.
  await evaluate(`(() => {
    ${volume}.setAttribute('aria-label', 'Caller name');
    ${volume}.setAttribute('aria-description', 'Caller fallback');
  })()`);
  assert.equal((await slider())?.name?.value, 'Caller name');
  await evaluate(`${volume}.setAttribute('aria-labelledby', 'field-volume-help')`);
  assert.equal((await slider())?.name?.value, 'Adjust the playback volume.');
  assert.deepEqual(exceptions, []);
  console.log('Field: DOM/AX naming and description precedence, required and validation state, horizontal layout and retained focus passed');
});
