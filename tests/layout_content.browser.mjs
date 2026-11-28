import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8126'});
  await waitFor('document.getElementById("layout-content-scroll")?.getAttribute("role") === "group"');

  await send('Accessibility.enable');
  const {root: document} = await send('DOM.getDocument');
  async function ax(id) {
    const {nodeId} = await send('DOM.querySelector', {nodeId: document.nodeId, selector: `#layout-content-${id}`});
    const {nodes: [node]} = await send('Accessibility.getPartialAXTree', {nodeId, fetchRelatives: false});
    return [node.role?.value, node.name?.value];
  }
  assert.deepEqual(await ax('basic'), ['region', 'Main Content']);
  assert.deepEqual(await ax('showcase'), ['region', 'Main Content Area']);
  assert.deepEqual(await ax('scroll'), ['group', 'Scrollable example']);
  assert.equal(await evaluate(`(() => {
    const root = document.getElementById('layout-content-scroll');
    root.scrollTop = 30;
    return root.scrollTop === 30 && !root.hasAttribute('tabindex');
  })()`), true);
  await evaluate('document.getElementById("layout-content-toggle").click()');
  await waitFor(`!document.getElementById('layout-content-scroll').hasAttribute('role') && !document.getElementById('layout-content-scroll').hasAttribute('aria-label')`);
  const [fittingRole, fittingName] = await ax('scroll');
  assert.notEqual(fittingRole, 'group');
  assert.ok(!fittingName);

  // The bands and panel are siblings of the scrollport, not scrolling descendants.
  for (const id of ['basic', 'showcase']) {
    assert.deepEqual(await evaluate(`(() => {
      const root = document.getElementById('layout-content-${id}');
      const layout = root.closest('.astryx-layout');
      const bands = [...layout.querySelectorAll('.astryx-layout-header, .astryx-layout-footer, .astryx-layout-panel')];
      const before = bands.map(band => band.getBoundingClientRect().top);
      const bodyBefore = root.firstElementChild.getBoundingClientRect().top;
      root.scrollTop = 10;
      return [bands.length, root.scrollTop > 0, root.firstElementChild.getBoundingClientRect().top < bodyBefore,
        bands.every((band, i) => band.getBoundingClientRect().top === before[i])];
    })()`), [id === 'basic' ? 1 : 3, true, true, true], id);
  }

  // Resolve the published values as lengths; computed custom properties may still contain rem units.
  const geometry = await evaluate(`(() => {
    const edges = ['inline-start', 'inline-end', 'block-start', 'block-end'];
    const ids = ['no-panels', 'start-panel', 'end-panel', 'both-panels', 'explicit', 'bleed', 'descendant', 'nested-layout'];
    const cases = ids.map(id => {
      const root = document.getElementById('layout-content-' + id);
      const css = getComputedStyle(root);
      const probe = document.createElement('div');
      probe.style.cssText = 'position:absolute;visibility:hidden';
      root.append(probe);
      const published = edges.map(edge => {
        probe.style.width = 'var(--container-padding-' + edge + ')';
        return parseFloat(getComputedStyle(probe).width);
      });
      probe.remove();
      const box = root.getBoundingClientRect();
      const layout = root.closest('.astryx-layout').getBoundingClientRect();
      const child = root.firstElementChild?.getBoundingClientRect();
      root.scrollTop = 10;
      return [id, {
        padding: edges.map(edge => parseFloat(css.getPropertyValue('padding-' + edge))), published,
        physical: [parseFloat(css.paddingLeft), parseFloat(css.paddingRight)],
        box: [box.left - layout.left, layout.right - box.right, box.height],
        child: child && [child.left - box.left, box.right - child.right],
        overflow: css.overflow, scrollTop: root.scrollTop, overflows: root.scrollHeight > root.clientHeight,
      }];
    });
    return {cases: Object.fromEntries(cases), rem: parseFloat(getComputedStyle(document.documentElement).fontSize)};
  })()`);
  const {cases, rem} = geometry;
  const outer = rem / 2;
  const override = 3 * rem / 4;
  for (const [id, padding, published, box] of [
    ['no-panels', [120 + outer, 120 + outer, 0, 0], [outer, outer, 0, 0], [0, 0, 120]],
    ['start-panel', [20, 120, 0, 0], [20, 0, 0, 0], [220, 0, 120]],
    ['end-panel', [120 + outer, 20, 12, 12], [outer, 20, 12, 12], [220, 0, 120]],
    ['both-panels', [20, 20, 0, 0], [20, 20, 0, 0], [220, 220, 120]],
    ['explicit', [120 + override, 120 + override, override, override], Array(4).fill(override), [0, 0, 120]],
    ['bleed', [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 120]],
  ]) {
    assert.deepEqual(cases[id].box, box, id);
    assert.deepEqual(cases[id].padding, padding, id);
    assert.deepEqual(cases[id].published, published, id);
    const physical = id === 'end-panel' ? [padding[1], padding[0]] : padding.slice(0, 2);
    assert.deepEqual(cases[id].physical, physical, id);
    assert.deepEqual(cases[id].child, physical, id);
  }
  assert.deepEqual(cases.descendant.padding, [outer, outer, 0, 0], 'descendant does not repeat the alignment gutter');
  assert.deepEqual(cases['nested-layout'].padding, [rem, rem, 12, 12], 'nested Layout resets the outer collapse and edge override');
  assert.deepEqual(cases['nested-layout'].published, [rem, rem, 12, 12]);
  assert.equal(cases.bleed.overflow, 'clip');
  assert.equal(cases.bleed.overflows, true);
  assert.equal(cases.bleed.scrollTop, 0);
  assert.deepEqual(exceptions, []);
  console.log('LayoutContent: scroll ownership, Chrome AX, and logical inset geometry passed');
});
