import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {runInNewContext} from 'node:vm';

const source = await readFile(new URL('../src/carousel/carousel_scroll.js', import.meta.url), 'utf8');

class Scroller extends EventTarget {
  clientWidth = 100;
  scrollWidth = 100;
  scrollLeft = 0;
  tabIndex = -1;
  children = [];
  direction = 'ltr';
  attributes = new Set();
  scrolls = [];
  wheelOptions = [];

  hasAttribute(name) { return this.attributes.has(name); }
  toggleAttribute(name, force) {
    if (force) this.attributes.add(name);
    else this.attributes.delete(name);
  }
  addEventListener(type, listener, options) {
    if (type === 'wheel') this.wheelOptions.push(options?.passive);
    super.addEventListener(type, listener, options);
  }
  scrollBy({left, behavior}) { this.scrolls.push({left, behavior}); }
}

const scroller = new Scroller();
const root = new EventTarget();
root.querySelector = selector => selector === ':scope > .astryx-carousel__scroller' ? scroller : null;
root.querySelectorAll = () => [];
let resize;
let mutate;
let receive;
const run = runInNewContext(`(async () => { ${source} })()`, {
  dioxus: {recv: () => new Promise(resolve => { receive = resolve; })},
  document: {getElementById: id => id === 'carousel' ? root : null},
  getComputedStyle: node => ({direction: node.direction}),
  ResizeObserver: class {
    constructor(callback) { resize = callback; }
    observe() {}
    disconnect() {}
  },
  MutationObserver: class {
    constructor(callback) { mutate = callback; }
    observe() {}
    disconnect() {}
  },
});
receive('carousel');
await new Promise(setImmediate);

const wheel = ({shiftKey = true, deltaX = 0, deltaY = 25, deltaMode = 0, prevented = false} = {}) => {
  const event = Object.assign(new Event('wheel', {cancelable: true, bubbles: true}), {
    shiftKey, deltaX, deltaY, deltaMode,
  });
  if (prevented) event.preventDefault();
  event.stopPropagation = () => assert.fail('Shift+wheel must not stop propagation');
  scroller.dispatchEvent(event);
  return event.defaultPrevented;
};
const assertIgnored = (input, prevented = false) => {
  assert.equal(wheel(input), prevented);
  assert.deepEqual(scroller.scrolls.splice(0), []);
};
const assertMapped = (input, left) => {
  assert.equal(wheel(input), true);
  assert.deepEqual(scroller.scrolls.splice(0), [{left, behavior: 'auto'}]);
};

assert.deepEqual(scroller.wheelOptions, []);
assertIgnored();
scroller.scrollWidth = 300;
resize();
resize();
scroller.dispatchEvent(new Event('scroll'));
assert.deepEqual(scroller.wheelOptions, [false]);
assert.equal(scroller.tabIndex, 0);

assertIgnored({shiftKey: false});
assertIgnored({deltaX: 5, deltaY: 0, shiftKey: false});
assertIgnored({deltaX: 5});
assertIgnored({deltaY: 0});
assertIgnored({prevented: true}, true);

// Edge availability changes the fades and buttons, not wheel ownership.
assert.equal(scroller.hasAttribute('data-overflow-start'), false);
assertMapped({deltaY: -25}, -25);
assertMapped({deltaY: 3, deltaMode: 1}, 3);
scroller.scrollLeft = 200;
scroller.dispatchEvent(new Event('scroll'));
assert.equal(scroller.hasAttribute('data-overflow-end'), false);
assertMapped({deltaY: 25}, 25);
scroller.direction = 'rtl';
scroller.scrollLeft = -100;
scroller.dispatchEvent(new Event('scroll'));
assertMapped({deltaY: -25}, -25);
assertMapped({deltaY: 25}, 25);

scroller.scrollWidth = 100;
scroller.scrollLeft = 0;
resize();
assert.equal(scroller.tabIndex, -1);
assertIgnored();
scroller.scrollWidth = 300;
mutate();
assert.deepEqual(scroller.wheelOptions, [false, false]);
assertMapped({}, 25);
receive();
await run;
assertIgnored();

console.log('Carousel wheel: overflow lifecycle, input filtering, edge and RTL deltas, and cleanup passed');
