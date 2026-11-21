import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8190'});
  const selector = '#chat-tokenized-text-examples .astryx-chat-tokenized-text';
  await waitFor(`document.querySelectorAll('${selector}').length === 3`);
  const rows = await evaluate(`Array.from(document.querySelectorAll('${selector}'), root => ({
    text: root.textContent,
    display: getComputedStyle(root).display,
    tokens: Array.from(root.querySelectorAll('.astryx-chat-tokenized-text__token'), wrapper => [
      wrapper.textContent, wrapper.firstElementChild.dataset.variant, getComputedStyle(wrapper).display,
      Math.abs(wrapper.getBoundingClientRect().height - Number.parseFloat(getComputedStyle(root).lineHeight)) <= 1 / 64,
    ]),
  }))`);
  const mixed = {
    text: '@Cindy filed #bug and #feature for the sprint', display: 'inline',
    tokens: [['@Cindy', 'blue', 'inline-flex', true], ['#bug', 'red', 'inline-flex', true], ['#feature', 'green', 'inline-flex', true]],
  };
  assert.deepEqual(rows, [mixed, {
    text: 'Assign @Cindy and @Alex as reviewers.', display: 'inline',
    tokens: [['@Cindy', 'blue', 'inline-flex', true], ['@Alex', 'blue', 'inline-flex', true]],
  }, mixed]);
  assert.deepEqual(exceptions, []);
  console.log('ChatTokenizedText: all three arrangements, text order, badge paint variants and inline geometry passed');
});
