import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8113'});
  await waitFor('!!document.getElementById("grid-span-columns")');
  for (const dir of ['ltr', 'rtl']) {
    assert.deepEqual(await evaluate(`(() => {
      const showcase = document.getElementById('grid-span-showcase');
      const columns = document.getElementById('grid-span-columns');
      const explicit = document.getElementById('grid-span-explicit');
      for (const grid of [showcase, columns, explicit]) grid.dir = '${dir}';
      const edge = '${dir === 'rtl' ? 'right' : 'left'}';
      const box = e => e.getBoundingClientRect();
      const close = (a, b) => Math.abs(a - b) < .1;
      const tracks = e => getComputedStyle(e).gridTemplateColumns.split(' ').map(parseFloat);
      const gap = e => parseFloat(getComputedStyle(e).columnGap);
      const [wide, single, tall, adjacent, finalSingle, finalWide] = showcase.children;
      const [first, second, third, fourth] = columns.children;
      const [implicit, full] = explicit.children;
      const [link, button] = full.children;
      const css = getComputedStyle(full);
      const [track1, track2, track3] = tracks(explicit);
      const [column1, column2, column3] = tracks(columns);
      return {
        showcase: showcase.children.length === 6 && tracks(showcase).length === 4 &&
          box(wide).width > box(single).width && box(wide)[edge] === box(showcase)[edge] &&
          box(wide).top === box(single).top && box(tall)[edge] === box(showcase)[edge] &&
          box(tall).top === box(adjacent).top && box(tall).bottom === box(finalSingle).bottom &&
          box(finalSingle).top === box(finalWide).top && box(tall).height > box(adjacent).height,
        columns: columns.children.length === 4 && tracks(columns).length === 3 &&
          close(box(first).width, column1 + column2 + gap(columns)) &&
          close(box(fourth).width, column2 + column3 + gap(columns)) &&
          box(first).top === box(second).top && box(third).top === box(fourth).top &&
          box(first)[edge] === box(columns)[edge] && box(third)[edge] === box(columns)[edge],
        explicitOnly: track3 === 40 && close(box(implicit).width, box(explicit).width) &&
          close(box(full).width, track1 + track2 + gap(explicit)) && box(full)[edge] === box(explicit)[edge],
        hostStyle: full.classList.contains('astryx-grid-span') && full.classList.contains('grid-span-caller') &&
          full.style.gridColumn === '1 / -1' && full.style.gridRow === '2' && full.style.color === 'inherit',
        fullHeight: css.height === '80px' && box(full).height === 80,
        native: full.tagName === 'DIV' && !full.hasAttribute('role') && !full.hasAttribute('tabindex') &&
          !full.hasAttribute('aria-label') && link.tagName === 'A' && button.tagName === 'BUTTON',
        shrinkable: css.display === 'grid' && css.minWidth === '0px' && tracks(full).length === 1 &&
          close(tracks(full)[0], box(full).width) && close(box(link).width, box(full).width) &&
          link.scrollWidth > link.clientWidth,
      };
    })()`), {
      showcase: true, columns: true, explicitOnly: true, hostStyle: true, fullHeight: true,
      native: true, shrinkable: true,
    }, dir);
  }
  await evaluate('document.getElementById("grid-span-next-control").click()');
  await waitFor('!!document.getElementById("grid-span-full")?.dataset.mountedSize');
  assert.equal(await evaluate(`(() => {
    const span = document.getElementById('grid-span-full');
    const [width, height] = span.dataset.mountedSize.split(',').map(Number);
    const rect = span.getBoundingClientRect();
    return Math.abs(width - rect.width) < .1 && Math.abs(height - rect.height) < .1;
  })()`), true, 'mounted measurement belongs to the wrapper');
  assert.equal(await evaluate(`(() => {
    const style = document.createElement('style');
    style.textContent = '.grid-span-caller { min-width: 1px; }';
    document.head.append(style);
    const minimum = getComputedStyle(document.getElementById('grid-span-full')).minWidth;
    style.remove();
    return minimum;
  })()`), '1px', 'caller class can override base styling');
  await evaluate('document.getElementById("grid-span-long-link").focus()');
  for (const type of ['keyDown', 'keyUp']) {
    await send('Input.dispatchKeyEvent', {type, key: 'Tab', code: 'Tab', windowsVirtualKeyCode: 9});
  }
  assert.equal(await evaluate('document.activeElement.id'), 'grid-span-next-control');
  console.log('GridSpan: both pinned layouts, explicit and implicit tracks, RTL, wrapper mount and child tab order passed');
});
