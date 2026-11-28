import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8145'});
  const labels = `document.getElementById('form-layout-horizontal-labels')`;
  const horizontal = `document.getElementById('form-layout-horizontal')`;
  await waitFor(`!!${labels}?.querySelector('textarea')`);
  const resize = width => send('Emulation.setDeviceMetricsOverride', {width, height: 900, deviceScaleFactor: 1, mobile: false});

  // Computed layout and bounding boxes, not a raster comparison.
  await resize(481);
  assert.deepEqual(await evaluate(`(() => {
    const el = ${labels};
    const css = getComputedStyle(el);
    const pairs = () => [...el.querySelectorAll('label')].map(label => [label.getBoundingClientRect(), label.control.getBoundingClientRect()]);
    const ltr = pairs();
    el.dir = 'rtl';
    const rtl = pairs();
    el.removeAttribute('dir');
    return [css.display, css.rowGap, css.columnGap, css.alignItems,
      ltr.every(([label, control]) => label.right <= control.left),
      rtl.every(([label, control]) => label.left >= control.right)];
  })()`), ['grid', '12px', '16px', 'start', true, true]);
  await resize(480);
  assert.deepEqual(await evaluate(`(() => {
    const el = ${labels};
    const css = getComputedStyle(el);
    const stacked = [...el.querySelectorAll('label')].every(label =>
      label.getBoundingClientRect().bottom <= label.control.getBoundingClientRect().top);
    return [css.display, css.rowGap, css.alignItems, stacked];
  })()`), ['flex', '16px', 'stretch', true]);
  assert.deepEqual(await evaluate(`(() => {
    const el = ${horizontal};
    const [first, last] = el.children;
    const forward = first.offsetLeft < last.offsetLeft;
    el.dir = 'rtl';
    const result = [getComputedStyle(el).display, forward, first.offsetLeft > last.offsetLeft, first.offsetWidth === last.offsetWidth];
    el.removeAttribute('dir');
    return result;
  })()`), ['grid', true, true, true]);
  assert.deepEqual(await evaluate(`(() => {
    const nested = document.getElementById('form-layout-nested');
    const css = getComputedStyle(nested);
    return [css.display, css.flexDirection, css.rowGap, ...[...nested.children].map(child => child.dataset.direction ?? 'field')];
  })()`), ['flex', 'column', '16px', 'horizontal', 'field', 'horizontal']);

  // DOM and Chrome AX, not screen-reader speech.
  assert.deepEqual(await evaluate(`(() => {
    const el = ${labels};
    const label = el.querySelector('label');
    label.click();
    return [el.tagName, el.hasAttribute('role'), el.closest('form'), label.control === document.activeElement, label.control.value];
  })()`), ['DIV', false, null, true, 'Jane Doe']);
  await send('Accessibility.enable');
  const {root} = await send('DOM.getDocument');
  for (const [selector, role, name] of [
    ['#form-layout-horizontal-labels input', 'textbox', 'Display Name'],
    ['#form-layout-horizontal-labels select', 'combobox', 'Timezone'],
    ['#form-layout-horizontal-labels textarea', 'textbox', 'Bio'],
    ['#form-layout-mixed-controls [role="group"]', 'group', 'Notifications'],
  ]) {
    const {nodeId} = await send('DOM.querySelector', {nodeId: root.nodeId, selector});
    const {nodes: [ax]} = await send('Accessibility.getPartialAXTree', {nodeId, fetchRelatives: false});
    assert.equal(ax.role.value, role, selector);
    assert.equal(ax.name.value, name, selector);
  }
  assert.deepEqual(exceptions, []);
  console.log('FormLayout: computed geometry, root/label DOM, and Chrome AX names passed');
});
