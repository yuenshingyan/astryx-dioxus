import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls:['https://*']});
  await send('Page.navigate', {url:process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8292'});
  const trigger = `document.querySelector('#popover-examples [aria-haspopup=dialog]')`;
  const panel = `document.getElementById(${trigger}.getAttribute('aria-controls'))`;
  await waitFor(`${trigger}?.getAttribute('aria-expanded') === 'false'`);
  await evaluate(`${trigger}.scrollIntoView({block:'center'}); ${trigger}.focus(); ${trigger}.click()`);
  await waitFor(`${panel}.matches(':popover-open')`);
  assert.equal(await evaluate(`${trigger}.getAttribute('aria-expanded')`), 'true');
  assert.ok(await evaluate(`Math.abs(${panel}.getBoundingClientRect().top - ${trigger}.getBoundingClientRect().bottom - 4) < 1`));
  const {nodes} = await send('Accessibility.getFullAXTree');
  assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'dialog' && node.name?.value === 'Settings'));
  await evaluate(`${panel}.querySelector('button').focus()`);
  await send('Input.dispatchKeyEvent', {type:'keyDown', key:'Escape', code:'Escape', windowsVirtualKeyCode:27});
  await send('Input.dispatchKeyEvent', {type:'keyUp', key:'Escape', code:'Escape', windowsVirtualKeyCode:27});
  await waitFor(`!${panel}.matches(':popover-open') && document.activeElement === ${trigger}`);
  await evaluate(`${trigger}.click()`);
  await waitFor(`${panel}.matches(':popover-open')`);
  await evaluate(`document.getElementById('popover-preview-outside').click()`);
  await waitFor(`!${panel}.matches(':popover-open')`);
  // Use a fresh anchor for the viewport cases so the browser has no prior
  // anchor-scroll adjustment from the normal in-flow example.
  await send('Page.navigate', {url: new URL('?fixture=popover-flips', process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8292').href});
  await waitFor(`location.search === '?fixture=popover-flips' && ${trigger}?.getAttribute('aria-expanded') === 'false'`);
  // Give this fixture a preferred side on each axis. Leave Popover's fallback list intact.
  const cornerCases = [
    {name: 'room on both axes', inset: '40vh auto auto 40vw', flipBlock: false, flipInline: false},
    {name: 'bottom edge', inset: 'auto auto 4px 40vw', flipBlock: true, flipInline: false},
    {name: 'right edge', inset: '40vh 4px auto auto', flipBlock: false, flipInline: true},
    {name: 'bottom-right corner', inset: 'auto 4px 4px auto', flipBlock: true, flipInline: true},
  ];
  for (const {name, inset, flipBlock, flipInline} of cornerCases) {
    await evaluate(`
      Object.assign(${trigger}.style, {position:'fixed', inset:${JSON.stringify(inset)}});
      ${panel}.style.positionArea = 'block-end inline-end';
      ${trigger}.click();
    `);
    await waitFor(`${panel}.matches(':popover-open')`);
    const {anchor, surface, viewport} = await evaluate(`({
      anchor: ${trigger}.getBoundingClientRect().toJSON(),
      surface: ${panel}.getBoundingClientRect().toJSON(),
      viewport: {width: document.documentElement.clientWidth, height: document.documentElement.clientHeight},
    })`);
    const blockGap = flipBlock ? anchor.top - surface.bottom : surface.top - anchor.bottom;
    const inlineGap = flipInline ? anchor.left - surface.right : surface.left - anchor.right;
    assert.ok(Math.abs(blockGap - 4) < 1, `${name}: expected 4px block gap, got ${blockGap}px`);
    assert.ok(Math.abs(inlineGap) < 1, `${name}: expected adjacent inline edges, got ${inlineGap}px`);
    assert.ok(Math.abs(surface.width - 280) < 1, `${name}: expected 280px width, got ${surface.width}px`);
    assert.ok(
      surface.top >= 0 && surface.left >= 0 &&
        surface.bottom <= viewport.height && surface.right <= viewport.width,
      `${name}: surface outside viewport: ${JSON.stringify(surface)}`,
    );
    await evaluate(`${trigger}.click()`);
    await waitFor(`!${panel}.matches(':popover-open')`);
  }

  const autoTrigger = `document.querySelector('#popover-auto-width-example [aria-haspopup=dialog]')`;
  const autoPanel = `document.getElementById(${autoTrigger}.getAttribute('aria-controls'))`;
  const rect = selector => `${selector}.getBoundingClientRect()`;
  await evaluate(`${autoTrigger}.scrollIntoView({block:'center'}); ${autoTrigger}.click()`);
  await waitFor(`${autoPanel}.matches(':popover-open')`);
  assert.ok(await evaluate(`Math.abs(${rect(autoPanel)}.width - ${rect(autoTrigger)}.width) < 1`));
  await evaluate(`Object.assign(${autoTrigger}.style, {position:'fixed', bottom:'4px', left:'35vw', inlineSize:'300px'})`);
  await waitFor(`${rect(autoPanel)}.bottom < ${rect(autoTrigger)}.top && Math.abs(${rect(autoPanel)}.width - 300) < 1`);
  assert.deepEqual(exceptions, []);
  console.log('Popover: anchor placement and flips, accessible name, Escape, outside click and focus return passed');
});
