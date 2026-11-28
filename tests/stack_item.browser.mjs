import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8115'});
  await waitFor('!!document.getElementById("stack-item-scroll")');
  const announced = async id => {
    const {root} = await send('DOM.getDocument', {depth: 0});
    const {nodeId} = await send('DOM.querySelector', {nodeId: root.nodeId, selector: `#${id}`});
    const {nodes: [node]} = await send('Accessibility.getPartialAXTree', {nodeId, fetchRelatives: false});
    return [node?.role?.value, node?.name?.value ?? ''];
  };
  assert.deepEqual(await evaluate(`['showcase','fill'].map(id => [...document.getElementById('stack-item-'+id).children].map(item => getComputedStyle(item).flexGrow))`), [['0','1','0'],['0','1']]);
  assert.deepEqual(await evaluate(`['showcase','fill','source-example'].map(id => {
    const root = document.getElementById('stack-item-'+id);
    const width = [...root.children].reduce((sum,item) => sum + item.getBoundingClientRect().width, 0);
    return Math.abs(width + (root.children.length - 1) * parseFloat(getComputedStyle(root).columnGap) - root.getBoundingClientRect().width) <= 1;
  })`), [true,true,true]);
  assert.deepEqual(await evaluate(`(() => {
    const row = document.getElementById('stack-item-source-example');
    return [...row.children].map(item => [item.textContent, getComputedStyle(item).flexShrink]);
  })()`), [['Logo','0'], ['Content','1'], ['Actions','0']]);
  assert.deepEqual(await evaluate(`(() => {
    const parent = document.getElementById('stack-item-align');
    const {left, right} = parent.getBoundingClientRect();
    const [unwrapped, start, center, end, stretch] = [...parent.children].map(el => el.getBoundingClientRect());
    const close = (a, b) => Math.abs(a - b) <= 1;
    return [close(unwrapped.left + unwrapped.right, left + right), close(start.left, left),
      close(center.left + center.right, left + right), close(end.right, right),
      close(stretch.left, left) && close(stretch.right, right)];
  })()`), [true,true,true,true,true]);
  await waitFor('document.querySelectorAll("[data-stack-item-mounted=true]").length >= 3');
  assert.deepEqual(await evaluate(`(() => {
    const roots = [...document.querySelectorAll('[data-stack-item-mounted=true]')];
    window.stackItemMountedRoots = roots;
    return roots.map(root => [root.id, root.localName, root.classList.contains('astryx-stack-item'),
      root.children.length, root.firstElementChild?.textContent]);
  })()`), [
    ['stack-item-scroll', 'div', true, 1, 'Content'],
    ['stack-item-scroll-span', 'span', true, 1, 'Wide content'],
    ['stack-item-scroll-article', 'article', true, 1, 'Native article'],
  ]);
  const assertSameMountedRoots = async () => assert.equal(await evaluate(`
    window.stackItemMountedRoots.every(root => root === document.getElementById(root.id))
  `), true, 'overflow changes must not replace a mounted root');
  await waitFor('document.getElementById("stack-item-scroll").getAttribute("role") === "group"');
  await assertSameMountedRoots();
  assert.deepEqual(await announced('stack-item-scroll'), ['group', 'Scrollable example']);
  await waitFor('document.getElementById("stack-item-scroll-span").getAttribute("role") === "group"');
  assert.deepEqual(await announced('stack-item-scroll-span'), ['group', 'Wide content']);
  assert.deepEqual(await evaluate(`(() => {
    const item = document.getElementById('stack-item-scroll-span');
    item.scrollLeft = 30;
    return [item.localName, item.getAttribute('aria-label'), item.scrollLeft, item.hasAttribute('tabindex')];
  })()`), ['span', 'Wide content', 30, false]);
  assert.deepEqual(await evaluate(`(() => {
    const item = document.getElementById('stack-item-scroll-article');
    item.scrollTop = 30;
    return [item.localName, item.getAttribute('role'), item.getAttribute('aria-label'), item.scrollTop];
  })()`), ['article', null, null, 30]);
  assert.equal(await evaluate(`(() => {const item=document.getElementById('stack-item-scroll'); item.scrollTop=30; return item.scrollTop;})()`), 30);
  await evaluate('document.getElementById("stack-item-toggle-overflow").click()');
  await waitFor('!document.getElementById("stack-item-scroll").hasAttribute("role") && !document.getElementById("stack-item-scroll").hasAttribute("aria-label")');
  await assertSameMountedRoots();
  await evaluate('document.getElementById("stack-item-toggle-overflow").click()');
  await waitFor('document.getElementById("stack-item-scroll").getAttribute("role") === "group"');
  await assertSameMountedRoots();
  assert.equal(exceptions.length,0);
  console.log('StackItem: named patterns, source example, cross-axis geometry, mounted roots, native scrolling, and overflow naming passed');
});
