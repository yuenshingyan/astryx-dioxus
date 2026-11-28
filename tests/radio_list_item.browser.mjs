import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8209'});
  const fixture = '#radio-list-item-interactions';
  const rich = '#radio-list-item-rich';
  const disabled = '#radio-list-item-disabled';
  const query = selector => `document.querySelector(${JSON.stringify(selector)})`;
  const counts = `${query('#radio-list-item-counts')}.textContent.trim()`;
  const count = state => waitFor(`${counts} === ${JSON.stringify(state)}`);
  const key = async (key, windowsVirtualKeyCode) => {
    for (const type of ['keyDown', 'keyUp']) await send('Input.dispatchKeyEvent', {type, key, windowsVirtualKeyCode});
  };
  const point = selector => evaluate(`(() => {
    const element = ${query(selector)};
    element.scrollIntoView({block: 'center'});
    const box = element.getBoundingClientRect();
    return {x: box.x + box.width / 2, y: box.y + box.height / 2};
  })()`);
  const click = async selector => {
    const xy = await point(selector);
    await send('Input.dispatchMouseEvent', {type: 'mouseMoved', ...xy});
    for (const type of ['mousePressed', 'mouseReleased']) await send('Input.dispatchMouseEvent', {type, ...xy, button: 'left', clickCount: 1});
  };
  const indicator = row => query(`${row} .astryx-radio-indicator`);
  const paint = row => `(() => { const css = getComputedStyle(${indicator(row)}); return [css.backgroundColor, css.borderTopColor]; })()`;

  await waitFor(`${query('#radio-list-item-counts')} !== null`);
  await count('standard:0:0:0');
  assert.deepEqual(await evaluate(`['basic','showcase'].map(pattern => {
    const root = document.getElementById('radio-list-item-' + pattern);
    return [...root.querySelectorAll('input')].map(input => [input.value, input.checked, input.disabled]);
  })`), [
    [['standard', true, false], ['express', false, false], ['overnight', false, false]],
    [['standard', true, false], ['express', false, false], ['overnight', false, false], ['sameday', false, true]],
  ]);

  await send('Emulation.setEmulatedMedia', {features: [{name: 'prefers-reduced-motion', value: 'reduce'}]});
  await send('Input.dispatchMouseEvent', {type: 'mouseMoved', x: 0, y: 0});
  const resting = await evaluate(paint(rich));
  await send('Input.dispatchMouseEvent', {type: 'mouseMoved', ...await point(`${rich} strong`)});
  await waitFor(`JSON.stringify(${paint(rich)}) !== ${JSON.stringify(JSON.stringify(resting))}`);
  const disabledRest = await evaluate(paint(disabled));
  await click(`${disabled} .astryx-item__label`);
  assert.deepEqual(await evaluate(paint(disabled)), disabledRest);
  assert.equal(await evaluate(`getComputedStyle(${query(`${disabled} .astryx-item__content`)}).opacity`), '0.5');
  await count('standard:0:0:0');

  await click(`${rich} a`);
  await count('standard:0:0:1');
  await click(`${rich} .astryx-item__description`);
  await count('express:1:1:1');
  assert.equal(await evaluate(`document.activeElement === ${query(`${rich} input`)}`), true);
  assert.equal(await evaluate(`${query(rich)}.dataset.selected`), 'selected');
  await click(`${rich} input`);
  await count('express:2:1:1');

  await evaluate(`${query('#radio-list-item-disable')}.focus()`);
  await key('Tab', 9);
  await waitFor(`document.activeElement === ${query(`${rich} input`)} && document.activeElement.matches(':focus-visible')`);
  assert.deepEqual(await evaluate(`[${query(rich)}, ${query(`${rich} .astryx-radio-list-item__control`)}, ${indicator(rich)}].map(el => getComputedStyle(el).outlineStyle)`), ['none', 'solid', 'none']);
  const {nodes} = await send('Accessibility.getFullAXTree');
  const express = nodes.find(node => !node.ignored && node.role?.value === 'radio' && node.name?.value === 'Express delivery');
  assert.equal(express?.description?.value, 'Two days See details');
  assert.equal(express?.properties?.find(p => p.name === 'checked')?.value?.value, 'true');
  const unavailable = nodes.find(node => !node.ignored && node.role?.value === 'radio' && node.name?.value === 'Unavailable');
  assert.equal(unavailable?.properties?.find(p => p.name === 'disabled')?.value?.value, true);

  await send('Emulation.setEmulatedMedia', {features: [{name: 'forced-colors', value: 'active'}]});
  assert.equal(await evaluate(`(() => {
    const circle = ${indicator(rich)}, ring = getComputedStyle(circle.parentElement);
    return ring.outlineStyle === 'solid' && parseFloat(ring.outlineWidth) > 0 &&
      getComputedStyle(circle).backgroundColor !== getComputedStyle(circle.firstElementChild).backgroundColor;
  })()`), true, 'computed forced-colors outline and dot');
  await key('Tab', 9);
  await waitFor(`document.activeElement === ${query(`${rich} a`)}`);
  assert.equal(await evaluate(`getComputedStyle(${query(`${rich} .astryx-radio-list-item__control`)}).outlineStyle`), 'none');

  await click('#radio-list-item-disable');
  await waitFor(`[...${query(fixture)}.querySelectorAll('input')].every(input => input.disabled)`);
  await click(`${rich} strong`);
  await count('express:2:1:1');
  await click(`${rich} a`);
  await count('express:2:1:2');
  await click('#radio-list-item-disable');
  await waitFor(`!${query(`${rich} input`)}.disabled`);
  await send('Emulation.setTouchEmulationEnabled', {enabled: true, maxTouchPoints: 1});
  await waitFor(`matchMedia('(pointer: coarse)').matches`);
  assert.deepEqual(await evaluate(`(() => {
    const input = ${query(`${rich} input`)}.getBoundingClientRect(), circle = ${indicator(rich)}.getBoundingClientRect();
    return [input.width, input.height, circle.width, circle.height, input.x - circle.x, input.y - circle.y];
  })()`), [24, 24, 20, 20, -2, -2]);
  assert.deepEqual(exceptions, []);
  console.log('RadioListItem: native activation and focus, Chrome AX, computed paint and coarse input geometry passed');
});
