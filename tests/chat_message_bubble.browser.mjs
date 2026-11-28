import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8185'});
  await waitFor(`!!document.getElementById('chat-bubble-wide')`);
  const state = await evaluate(`(() => {
    const group = document.getElementById('chat-bubble-group');
    const middle = document.getElementById('chat-bubble-middle');
    const css = getComputedStyle(middle);
    const corners = () => [css.borderTopLeftRadius, css.borderBottomLeftRadius, css.borderTopRightRadius, css.borderBottomRightRadius];
    const ltr = corners();
    group.dir = 'rtl';
    const rtl = corners();
    const wide = document.getElementById('chat-bubble-wide');
    const wideCss = getComputedStyle(wide);
    const avatar = group.querySelector('.astryx-chat-message__avatar');
    const name = group.querySelector('[data-chat-name]');
    const names = [...document.querySelectorAll('#chat-message-bubble-examples .astryx-chat-message-bubble')];
    return {ltr, rtl, fits: wide.offsetWidth === wide.parentElement.clientWidth, cap: wideCss.maxWidth,
      ghost: [wideCss.paddingBlockStart, wideCss.paddingInlineStart, wideCss.backgroundColor],
      avatarOffset: avatar.getBoundingClientRect().top - group.getBoundingClientRect().top,
      nameHeight: name.getBoundingClientRect().height,
      densities: [...new Set(names.map(bubble => bubble.dataset.density))].sort()};
  })()`);
  assert.deepEqual(state, {
    ltr: ['4px', '4px', '28px', '28px'], rtl: ['28px', '28px', '4px', '4px'],
    fits: true, cap: 'none', ghost: ['0px', '16px', 'rgba(0, 0, 0, 0)'], avatarOffset: 20, nameHeight: 20,
    densities: ['balanced', 'compact', 'spacious'],
  });
  const reflow = await evaluate(`(() => {
    const fixture = document.getElementById('chat-bubble-narrow');
    const name = fixture.querySelector('[data-chat-name]');
    const metadata = fixture.querySelector('.astryx-chat-message-bubble__metadata');
    const lineCount = element => {
      const range = document.createRange();
      range.selectNodeContents(element);
      return range.getClientRects().length;
    };
    return {
      width: fixture.clientWidth,
      fits: [fixture, name, metadata].every(element => element.scrollWidth <= element.clientWidth),
      nameFitsVertically: name.scrollHeight <= name.clientHeight,
      nameGrows: name.clientHeight > ${state.nameHeight},
      nameWraps: lineCount(name) > 1,
      metadataWraps: lineCount(metadata) > 1,
    };
  })()`);
  assert.deepEqual(reflow, {
    width: 320, fits: true, nameFitsVertically: true, nameGrows: true, nameWraps: true, metadataWraps: true,
  });
  assert.deepEqual(exceptions, []);
  console.log('ChatMessageBubble: logical corners, name-row avatar offset, densities, full-width ghost and narrow slot reflow passed');
});
