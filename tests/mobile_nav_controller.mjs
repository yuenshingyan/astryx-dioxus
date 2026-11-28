import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {runInNewContext} from 'node:vm';

const source = await readFile(new URL('../src/mobile_nav/mobile_nav.js', import.meta.url), 'utf8');

// Exercise the actual controller without assuming the browser restores focus on close.
async function mount({nativeRestore = false, startsOpen = false} = {}) {
  const document = {body: {focus: () => assert.fail('must not focus the body')}};
  const focusCalls = [];
  const makeOpener = name => ({
    isConnected: true,
    focus() {
      assert.equal(dialog.open, false, 'the opener is inert until the dialog closes');
      focusCalls.push(name);
      document.activeElement = this;
    },
  });
  const opener = makeOpener('opener');
  const nextOpener = makeOpener('next opener');
  const panel = {};
  let nativeOpener;
  const dialog = Object.assign(new EventTarget(), {
    ownerDocument: document,
    dataset: {open: String(startsOpen)},
    isConnected: true,
    open: false,
    contains: node => node === panel,
    showModal() {
      nativeOpener = document.activeElement;
      this.open = true;
      document.activeElement = panel;
    },
    close() {
      this.open = false;
      document.activeElement = nativeRestore ? nativeOpener : document.body;
    },
  });
  document.activeElement = opener;
  document.querySelector = () => dialog;
  let mutate;
  let receive;
  let disconnected = false;
  const requests = [];
  const run = runInNewContext(`(async () => { ${source} })()`, {
    document,
    CSS: {escape: id => id},
    dioxus: {
      recv: () => new Promise(resolve => { receive = resolve; }),
      send: value => requests.push(value),
    },
    MutationObserver: class {
      constructor(callback) { mutate = callback; }
      observe() {}
      disconnect() { disconnected = true; }
    },
  });
  receive('drawer');
  await new Promise(setImmediate);
  return {
    document, dialog, focusCalls, requests, opener, nextOpener, panel,
    setOpen(open) {
      dialog.dataset.open = String(open);
      mutate();
    },
    cancel(target = dialog) {
      const event = new Event('cancel', {bubbles: true, cancelable: true});
      // Node's EventTarget has no DOM ancestry; deliver the event as seen on the dialog.
      Object.defineProperty(event, 'target', {value: target});
      dialog.dispatchEvent(event);
      return event.defaultPrevented;
    },
    async unmount() {
      dialog.isConnected = false;
      receive();
      await run;
      assert.equal(disconnected, true);
      assert.equal(dialog.open, false);
      assert.equal(this.cancel(), false, 'teardown removes the cancel listener');
    },
  };
}

{
  const nav = await mount({startsOpen: true});
  assert.equal(nav.document.activeElement, nav.panel);
  assert.equal(nav.cancel({}), false, 'a descendant cancel is not a drawer close request');
  assert.deepEqual(nav.requests, []);
  assert.equal(nav.cancel(), true, 'the native cancel cannot bypass controlled state');
  assert.deepEqual(nav.requests, [true]);
  assert.equal(nav.dialog.open, true, 'the caller may decline the close request');
  assert.equal(nav.document.activeElement, nav.panel);
  nav.setOpen(true);
  nav.setOpen(false);
  assert.equal(nav.document.activeElement, nav.opener);
  assert.deepEqual(nav.focusCalls, ['opener']);

  nav.document.activeElement = nav.nextOpener;
  nav.setOpen(true);
  await nav.unmount();
  assert.equal(nav.document.activeElement, nav.nextOpener);
  assert.deepEqual(nav.focusCalls, ['opener', 'next opener']);
}

{
  const nav = await mount({startsOpen: true, nativeRestore: true});
  nav.setOpen(false);
  assert.equal(nav.document.activeElement, nav.opener);
  assert.deepEqual(nav.focusCalls, [], 'do not refocus when native restoration succeeded');
  await nav.unmount();
}

{
  const nav = await mount({startsOpen: true});
  nav.opener.isConnected = false;
  nav.setOpen(false);
  assert.deepEqual(nav.focusCalls, [], 'do not focus a detached opener');
  nav.setOpen(true); // No opener this time: the document body has focus.
  nav.setOpen(false);
  assert.deepEqual(nav.focusCalls, []);
  await nav.unmount();
}

console.log('MobileNav controller: controlled cancel origin, focus restore, reopen, and teardown passed');
