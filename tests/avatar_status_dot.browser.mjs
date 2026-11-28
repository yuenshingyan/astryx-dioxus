import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8123'});
  await waitFor('!!document.getElementById("avatar-dot-busy")');
  for (const scheme of ['light','dark']) {
    const actual = await evaluate(`(() => {
      document.getElementById('avatar-status-dot-examples').style.colorScheme = '${scheme}';
      return ['online','away','busy'].map(id => {
        const root = document.getElementById('avatar-dot-'+id);
        const style = getComputedStyle(root);
        const glyph = root.querySelector('svg');
        return [root.offsetWidth,root.offsetHeight,style.borderTopWidth,style.backgroundColor,
          glyph?.dataset.shape ?? null,glyph?.getAttribute('aria-hidden') ?? null];
      });
    })()`);
    const paint = scheme === 'light' ? ['rgb(25, 129, 0)','rgb(255, 255, 255)','rgb(201, 48, 58)'] : ['rgb(100, 175, 76)','rgb(38, 38, 38)','rgb(255, 112, 93)'];
    assert.deepEqual(actual, [[10,10,'1px',paint[0],null,null],[10,10,'1px',paint[1],'ring','true'],[10,10,'1px',paint[2],'minus','true']]);
  }
  for (const [id, names, labels] of [
    ['showcase', ['Online User', 'Away User', 'Busy User'], ['Online', 'Away', 'Busy']],
    ['variants', ['Ana Silva', 'Ben Okafor', 'Cleo Marsh'], ['Online', 'Away', 'Do not disturb']],
  ]) {
    const row = `document.getElementById('avatar-dot-${id}')`;
    await waitFor(`${row}.lastElementChild?.getAttribute('aria-label') === '${names[2]}, ${labels[2]}'`);
    const metrics = await evaluate(`(() => {
      const row = ${row};
      return {
        gap: getComputedStyle(row).columnGap,
        alignment: getComputedStyle(row).alignItems,
        avatars: Array.from(row.children, avatar => {
          const dot = avatar.querySelector('.astryx-avatar__status .astryx-avatar-status-dot');
          const glyph = dot.querySelector('svg');
          return [avatar.getAttribute('aria-label'), avatar.offsetWidth, dot.offsetWidth, dot.dataset.variant,
            glyph?.dataset.shape ?? null, glyph?.firstElementChild.getAttribute('stroke-width') ?? null];
        }),
      };
    })()`);
    assert.equal(metrics.alignment, 'center');
    assert.equal(metrics.gap, await evaluate('getComputedStyle(document.documentElement).fontSize'));
    assert.deepEqual(metrics.avatars, [
      [`${names[0]}, ${labels[0]}`, 128, 32, 'success', null, null],
      [`${names[1]}, ${labels[1]}`, 128, 32, 'neutral', 'ring', '2'],
      [`${names[2]}, ${labels[2]}`, 128, 32, 'error', 'minus', '2'],
    ]);
  }
  const tierMetrics = await evaluate(`['small', 'medium', 'large'].map(tier => {
    const dot = document.getElementById('avatar-dot-' + tier);
    const icon = dot.querySelector('.astryx-avatar-status-dot-icon');
    return [dot.offsetWidth, dot.offsetHeight, getComputedStyle(dot).borderTopWidth,
      dot.querySelector('[data-shape]')?.dataset.shape ?? null, icon?.offsetWidth ?? null,
      icon?.offsetHeight ?? null, icon?.getAttribute('aria-hidden') ?? null];
  })`);
  assert.deepEqual(tierMetrics, [
    [10, 10, '1px', 'ring', null, null, null],
    [20, 20, '2px', null, 12, 12, 'true'],
    [32, 32, '4px', null, 18, 18, 'true'],
  ]);
  await send('Accessibility.enable');
  const {nodes} = await send('Accessibility.getFullAXTree');
  for (const name of ['Online', 'Away', 'Do not disturb', 'Ana Silva, Online', 'Ben Okafor, Away', 'Cleo Marsh, Do not disturb']) {
    assert.ok(nodes.some(node => !node.ignored && node.role?.value === 'image' && node.name?.value === name), name);
  }
  for (const name of ['Ana Silva, Online', 'Ben Okafor, Away', 'Cleo Marsh, Do not disturb']) {
    const avatar = nodes.find(node => !node.ignored && node.role?.value === 'image' && node.name?.value === name);
    const descendants = new Set(avatar.childIds ?? []);
    const byId = new Map(nodes.map(node => [node.nodeId, node]));
    for (const id of descendants) {
      const node = byId.get(id);
      assert.ok(node?.ignored, `${name}: an unexpected accessible descendant`);
      for (const child of node.childIds ?? []) descendants.add(child);
    }
  }
  assert.deepEqual(exceptions, []);
  console.log('AvatarStatusDot: standalone paint, template composition, icon tiers, and accessible names passed');
});
