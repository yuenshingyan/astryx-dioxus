import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8134'});
  await waitFor(`!!document.querySelector('#metadata-project > dl > dd a')`);
  assert.deepEqual(await evaluate(`['metadata-deployment','metadata-project'].map(id => {
    const list = document.querySelector('#' + id + ' > dl');
    return [...list.children].map(node => node.tagName);
  })`), [Array.from({length: 8}, (_, i) => i % 2 ? 'DD' : 'DT'), Array.from({length: 12}, (_, i) => i % 2 ? 'DD' : 'DT')]);
  assert.deepEqual(await evaluate(`(() => {
    const term = document.querySelector('[data-testid="metadata-environment-label"]');
    const value = document.querySelector('[data-testid="metadata-environment-value"]');
    const textTop = node => {
      const range = document.createRange();
      range.selectNodeContents(node);
      return range.getBoundingClientRect().top;
    };
    return [term.textContent, value.textContent, term.nextElementSibling === value,
      Math.abs(textTop(term) - textTop(value)) <= 1 / 64,
      getComputedStyle(term).minHeight, getComputedStyle(value).marginInlineStart,
      getComputedStyle(term).wordBreak, getComputedStyle(value).wordBreak];
  })()`), ['Environment', 'Production', true, true, '24px', '0px', 'break-word', 'break-word']);
  assert.deepEqual(await evaluate(`(() => {
    const example = document.querySelector('#metadata-list-multi');
    const item = example.querySelector('dl > div').cloneNode(true);
    const fixture = document.createElement('dl');
    fixture.style.cssText = 'position:absolute;width:80px;word-break:normal;overflow-wrap:normal';
    fixture.append(item);
    const [term, value] = item.children;
    term.textContent = value.textContent = 'MetadataIdentifier'.repeat(4);
    example.append(fixture);
    try {
      return [getComputedStyle(term).wordBreak, getComputedStyle(value).wordBreak,
        term.scrollWidth > term.clientWidth, value.scrollWidth === value.clientWidth,
        value.clientHeight > term.clientHeight];
    } finally {
      fixture.remove();
    }
  })()`), ['normal', 'break-word', true, true, true]);
  await evaluate(`window.metadataOwner = document.querySelector('#metadata-project > dl > dd a'); metadataOwner.focus()`);
  assert.equal(await evaluate(`document.activeElement === metadataOwner && metadataOwner.textContent === 'Alice Johnson'`), true);
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && ['term','DescriptionListTerm'].includes(node.role?.value) && node.name?.value === 'Environment'));
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'link' && node.name?.value === 'Alice Johnson'));
  assert.deepEqual(exceptions, []);
  console.log('MetadataListItem: both examples, term/value alignment and wrapping, native AX term and focusable value link passed');
});
