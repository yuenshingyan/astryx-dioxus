import assert from 'node:assert/strict';
import {withTarget} from './card_metrics.mjs';

await withTarget(async ({send, evaluate, waitFor, exceptions}) => {
  await send('Network.enable');
  await send('Network.setBlockedURLs', {urls: ['https://*']});
  await send('Page.navigate', {url: process.env.SHOWCASE_URL ?? 'http://127.0.0.1:8141'});
  await send('Page.bringToFront');
  const group = `document.getElementById('avatar-group-reviewers')`;
  const members = `${group}.querySelectorAll('[data-avatar-item]')`;
  const hint = `${group}.querySelector('[data-astryx-avatar-group-hint]')`;
  const description = `${group}.getAttribute('aria-describedby')`;
  await waitFor(`${group}?.getAttribute('aria-describedby')?.includes('astryx-avatar-group-hint')`);
  const hintId = await evaluate(`${hint}.id`);
  const callerId = 'avatar-group-description';
  const combinedDescription = `${callerId} ${hintId}`;
  assert.equal(await evaluate(`${hint}.textContent`), 'Use arrow keys to move between avatars');
  assert.equal(await evaluate(description), combinedDescription);
  assert.deepEqual(await evaluate(`[...${members}].map(node => [node.offsetWidth, node.tabIndex])`), [[52, 0], [52, -1], [52, -1], [52, -1]]);
  assert.deepEqual(await evaluate(`(() => {
    const [first, second] = ${members};
    const attendees = document.getElementById('avatar-group-attendees');
    return [first.offsetLeft - ${group}.offsetLeft, second.offsetLeft - first.offsetLeft,
      attendees.querySelector('.astryx-avatar').offsetWidth, attendees.querySelectorAll('[tabindex]').length,
      attendees.hasAttribute('aria-describedby'), !!attendees.querySelector('[data-astryx-avatar-group-hint]')];
  })()`), [0, 40, 28, 0, false, false]);
  const key = async (key, windowsVirtualKeyCode, text) => {
    await send('Input.dispatchKeyEvent', {type: 'keyDown', key, code: key, windowsVirtualKeyCode, text});
    await send('Input.dispatchKeyEvent', {type: 'keyUp', key, code: key, windowsVirtualKeyCode});
  };
  const checkFocus = async index => assert.equal(await evaluate(`document.activeElement === ${members}[${index}] && document.activeElement.tabIndex === 0`), true);
  await evaluate(`${members}[0].focus()`);
  await key('ArrowRight', 39);
  await checkFocus(1);
  await key('End', 35);
  await checkFocus(3);
  await key('Enter', 13, '\r');
  await waitFor(`!!document.getElementById('avatar-group-result')`);
  await key('ArrowRight', 39);
  await checkFocus(0);
  await evaluate(`${group}.dir = 'rtl'`);
  await key('ArrowLeft', 37);
  await checkFocus(1);
  await evaluate(`document.getElementById('avatar-group-toggle-keys').click()`);
  await key('ArrowRight', 39);
  await checkFocus(1);

  await evaluate(`${members}.forEach(member => { member.hidden = true; })`);
  await waitFor(`${hint} === null && ${description} === '${callerId}'`);
  await evaluate(`${members}.forEach(member => { member.hidden = false; })`);
  await waitFor(`${hint}?.id === '${hintId}' && ${description} === ${JSON.stringify(combinedDescription)}`);
  assert.deepEqual(exceptions, []);
  console.log('AvatarGroup: shared geometry, static members, keyboard hint, RTL roving and caller cancellation passed');
});
