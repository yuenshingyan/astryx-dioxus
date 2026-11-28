import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8080'});
  await waitFor('document.querySelector("#grid-align-stretch")');
  assert.deepEqual(await evaluate(`['fixed','gallery','auto-fit','dashboard','with-span'].map(id => document.getElementById('grid-'+id).children.length)`), [12, 6, 6, 6, 9]);
  const tracks = id => evaluate(`getComputedStyle(document.getElementById('${id}')).gridTemplateColumns.split(' ').length`);
  assert.equal(await tracks('grid-fixed'), 3);
  assert.equal(await tracks('grid-gallery'), 2);
  assert.equal(await tracks('grid-auto-fit'), 2);
  await evaluate(`const range = document.getElementById('grid-panel-width'); range.value = 280; range.dispatchEvent(new Event('input',{bubbles:true}));`);
  await waitFor(`getComputedStyle(document.getElementById('grid-auto-fit')).gridTemplateColumns.split(' ').length === 1`);
  assert.deepEqual(await evaluate(`['grid-dashboard','grid-with-span'].map(id=>{const grid=document.getElementById(id);return grid.getBoundingClientRect().width===grid.lastElementChild.getBoundingClientRect().width})`), [true,true]);
  await evaluate(`document.getElementById('grid-fixed').dir='rtl'`);
  assert.equal(await evaluate(`(()=>{const grid=document.getElementById('grid-fixed');return grid.children[0].textContent==='Item 1' && grid.children[0].getBoundingClientRect().x>grid.children[1].getBoundingClientRect().x})()`), true);
  const alignment = name => evaluate(`(() => {
    const grid = document.getElementById('grid-align-${name}');
    const {alignItems, justifyItems} = getComputedStyle(grid);
    const root = grid.getBoundingClientRect();
    const item = grid.firstElementChild.getBoundingClientRect();
    return {
      values: [alignItems, justifyItems],
      reflected: [grid.getAttribute('data-align'), grid.getAttribute('data-justify')],
      rect: [item.x - root.x, item.y - root.y, item.width, item.height],
    };
  })()`);
  const cases = [
    ['normal', ['normal', 'normal'], [0, 0, 160, 80], [0, 0, 160, 80]],
    ['start', ['start', 'end'], [120, 0, 40, 20], [0, 0, 40, 20]],
    ['center', ['center', 'start'], [0, 30, 40, 20], [120, 30, 40, 20]],
    ['end', ['end', 'center'], [60, 60, 40, 20], [60, 60, 40, 20]],
    ['stretch', ['stretch', 'stretch'], [0, 0, 160, 80], [0, 0, 160, 80]],
  ];
  for (const direction of ['ltr', 'rtl']) {
    await evaluate(`document.getElementById('grid-alignment-examples').dir = '${direction}'`);
    for (const [name, values, ltrRect, rtlRect] of cases) {
      assert.deepEqual(await alignment(name), {
        values,
        reflected: name === 'normal' ? [null, null] : values,
        rect: direction === 'ltr' ? ltrRect : rtlRect,
      }, `${direction}: ${name} item alignment`);
    }
  }

  await evaluate(`document.head.insertAdjacentHTML('beforeend', \`<style>
    @media (max-width: 600px) {
      #grid-fixed { grid-template-columns: repeat(2, 1fr); }
      #grid-align-start { align-items: end; justify-items: start; }
    }
  </style>\`)`);
  await send('Emulation.setDeviceMetricsOverride', {width: 600,height: 900,deviceScaleFactor: 1,mobile: false});
  assert.equal(await tracks('grid-fixed'), 2, 'unlayered responsive caller CSS overrides fixed props');
  assert.deepEqual(await alignment('start'), {
    values: ['end', 'start'],
    reflected: ['start', 'end'],
    rect: [120, 60, 40, 20],
  }, 'RTL caller media rule overrides both axes without changing prop metadata');
  console.log('Grid: five patterns, native tracks, resizing, full spans, LTR/RTL alignment and CSS overrides passed');
});
