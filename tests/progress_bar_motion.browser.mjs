// Native ProgressBarTouch boundary checks against current adapter files.
// No preview/build dependency. Geometry below is an integration-contract fixture,
// NOT proof of the parent component's rendering, accessibility or final paint.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

const endpoint = process.env.CDP_URL ?? 'http://127.0.0.1:9222';
const target = await (await fetch(`${endpoint}/json/new?about:blank`, { method: 'PUT' })).json();
const ws = new WebSocket(target.webSocketDebuggerUrl);
await new Promise(resolve => { ws.onopen = resolve; });
let id = 0;
const pending = new Map();
ws.onmessage = ({ data }) => {
  const message = JSON.parse(data), request = pending.get(message.id);
  if (!request) return;
  pending.delete(message.id);
  if (message.error) request.reject(Error(JSON.stringify(message.error)));
  else request.resolve(message.result);
};
const send = (method, params = {}) => new Promise((resolve, reject) => {
  pending.set(++id, { resolve, reject });
  ws.send(JSON.stringify({ id, method, params }));
});
const evaluate = async expression => {
  const result = await send('Runtime.evaluate', { expression, awaitPromise: true, returnByValue: true });
  assert.equal(result.exceptionDetails, undefined, JSON.stringify(result.exceptionDetails));
  return result.result.value;
};
const near = (actual, expected, why, tolerance = .1) => assert.ok(Math.abs(actual - expected) < tolerance, `${why}: ${actual} vs ${expected}`);
const media = async (coarse, reduced = false, forced = false) => {
  await send('Emulation.setTouchEmulationEnabled', { enabled: coarse, maxTouchPoints: 1 });
  await send('Emulation.setEmulatedMedia', { features: [
    { name: 'prefers-reduced-motion', value: reduced ? 'reduce' : 'no-preference' },
    { name: 'forced-colors', value: forced ? 'active' : 'none' },
  ] });
  const actual = await evaluate(`new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(() => resolve([matchMedia('(pointer: coarse)').matches, matchMedia('(prefers-reduced-motion: reduce)').matches, matchMedia('(forced-colors: active)').matches]))))`);
  assert.deepEqual(actual, [coarse, reduced, forced]);
};

try {
  const files = [
    '../src/motion/uikit/progress_bar.js', '../src/motion/uikit/timing.js',
    '../src/motion/theme_timing.js', '../src/utils/touch_queries.js',
    '../src/utils/shared_resize_observer.js', '../src/progress_bar/progress_bar_touch.js',
  ];
  const source = (await Promise.all(files.map(file => readFile(new URL(file, import.meta.url), 'utf8')))).join('\n');
  await evaluate(`(() => {
    document.head.innerHTML = '<style>' + ${JSON.stringify(`
      .astryx-progress-bar { width:200px; --duration-medium:calc(300ms * 2); --ease-standard:cubic-bezier(.24,1,.4,1); }
      .astryx-progress-bar-track {height:8px;overflow:hidden;border-radius:8px;background:gray;}
      .astryx-progress-bar-fill { height:100%;width:100%;background:blue;outline:1px solid black;outline-offset:-1px;border-radius:8px;will-change:transform;
        --drawn:var(--_progress-bar-drawn,var(--_progress-bar-fill,0%));--length:var(--drawn);transform:translateX(calc(var(--length) - 100%)); }
      .astryx-progress-bar[data-astryx-progress-bar-direction="rtl"] .astryx-progress-bar-fill {transform:translateX(calc(100% - var(--length)));}
      .astryx-progress-bar[data-astryx-progress-bar-empty="true"][data-mode="determinate"] .astryx-progress-bar-fill {opacity:0;}
      @media(pointer:coarse) {.astryx-progress-bar-fill {--length:min(100%,max(var(--drawn),var(--_progress-bar-min,16px)));}}
      .astryx-progress-bar[data-mode="indeterminate"] .astryx-progress-bar-fill {width:40%;animation:slide 1500ms ease-in-out infinite;}
      .astryx-progress-bar[data-mode="indeterminate"][data-astryx-progress-bar-direction="rtl"] .astryx-progress-bar-fill {animation-name:slide-rtl;}
      @keyframes slide {from{transform:translateX(-100%);}to{transform:translateX(250%);}}
      @keyframes slide-rtl {from{transform:translateX(100%);}to{transform:translateX(-250%);}}
      @keyframes pulse {from{opacity:1;}to{opacity:var(--_progress-bar-pulse-floor,.84);}}
      @media(prefers-reduced-motion:reduce) {
        .astryx-progress-bar[data-mode="indeterminate"] .astryx-progress-bar-fill {width:100%;transform:none;animation:pulse var(--_progress-bar-pulse,2000ms) var(--_progress-bar-pulse-curve,cubic-bezier(.42,0,.58,1)) infinite alternate;}
      }
      @media(forced-colors:active) {
        .astryx-progress-bar[data-mode="determinate"] .astryx-progress-bar-fill {transform:none;width:var(--length);will-change:auto;outline-color:CanvasText;}
      }
    `)} + '</style>';
    const run = new (Object.getPrototypeOf(async function(){}).constructor)('dioxus', ${JSON.stringify(source)});
    window.bars = new Map();
    window.attach = async (id, percent, hasMarks = false) => {
      const root = document.createElement('div');
      root.className = 'astryx-progress-bar'; root.setAttribute('data-astryx-progress-bar-id', id);
      root.dataset.mode = 'determinate'; root.style.setProperty('color','rgb(1, 2, 3)'); root.setAttribute('data-caller','preserved');
      root.innerHTML = '<div class="astryx-progress-bar-track"><div class="astryx-progress-bar-fill"></div></div><span class="astryx-progress-bar-mark" data-astryx-mark-percent="5" data-placement="track"></span><span class="astryx-progress-bar-mark" data-astryx-mark-percent="20" data-placement="track"></span>';
      const fill = root.querySelector('.astryx-progress-bar-fill'), track = root.querySelector('.astryx-progress-bar-track');
      fill.style.setProperty('--_progress-bar-fill', percent + '%'); document.body.append(root);
      let resolve, first = true;
      const record = {root, fill, track, push:value=>resolve(value)};
      record.done = run({recv:()=>first ? (first=false,Promise.resolve([id,percent,'determinate',hasMarks])) : new Promise(r=>resolve=r)});
      bars.set(id,record); await Promise.resolve(); await Promise.resolve();
    };
    window.update = async (id, percent, mode='determinate', hasMarks=false) => {
      const b=bars.get(id); b.root.dataset.mode=mode;
      // Dioxus replaces the fill's whole style attribute; private state must
      // survive this render rather than starting at the freshly rendered value.
      b.fill.setAttribute('style','--_progress-bar-fill:'+percent+'%');
      b.push([id,percent,mode,hasMarks]);
      await Promise.resolve(); await Promise.resolve(); await Promise.resolve();
    };
    window.disposeBar = async (id, value='dispose') => {const b=bars.get(id); b.push(value); await b.done; b.root.remove(); bars.delete(id);};
    window.edge = id => {const b=bars.get(id),t=b.track.getBoundingClientRect(),f=b.fill.getBoundingClientRect();return getComputedStyle(b.track).direction==='rtl'?t.right-f.left:f.right-t.left;};
  })()`);

  for (const coarse of [false, true]) {
    await media(coarse);
    const key = coarse ? 'coarse' : 'fine';
    const mount = await evaluate(`(async()=>{await attach('${key}',10); const b=bars.get('${key}');return {count:b.fill.getAnimations().length,edge:edge('${key}')}})()`);
    assert.equal(mount.count, 0, 'SSR target has no mount animation'); near(mount.edge, 20, 'mount target');
    const move = await evaluate(`(async()=>{
      await update('${key}',90); const b=bars.get('${key}'),a=b.fill.getAnimations()[0];a.pause();a.currentTime=0;
      const start=edge('${key}'); a.currentTime=300;
      const middle=edge('${key}'); const probe=new Animation(new KeyframeEffect(null,null,{duration:600,easing:a.effect.getTiming().easing,fill:'both'}));probe.currentTime=300;
      return {start,middle,eased:probe.effect.getComputedTiming().progress,duration:a.effect.getTiming().duration,easing:a.effect.getTiming().easing,keys:Object.keys(a.effect.getKeyframes()[0]),outline:getComputedStyle(b.fill).outlineWidth};
    })()`);
    assert.equal(move.duration, 600); assert.equal(move.easing, 'cubic-bezier(0.24, 1, 0.4, 1)');
    near(move.start, 20, 'move starts at drawn edge'); near(move.middle, 20 + 160 * move.eased, 'theme easing');
    assert.equal(move.outline, '1px'); assert.ok(!move.keys.some(key => /outline|border|width/.test(key)), 'only transform animated');
    const retarget = await evaluate(`(async()=>{
      const b=bars.get('${key}'),before=edge('${key}');await update('${key}',10);const a=b.fill.getAnimations()[0];a.pause();a.currentTime=0;
      return {before,after:edge('${key}'),duration:a.effect.getTiming().duration,count:b.fill.getAnimations().length};
    })()`);
    near(retarget.after, retarget.before, 'return starts from current drawing'); assert.equal(retarget.duration, 600, 'no reverse shortening'); assert.equal(retarget.count, 1);
    const same = await evaluate(`(async()=>{const b=bars.get('${key}'),a=b.fill.getAnimations()[0];await update('${key}',10);return b.fill.getAnimations()[0]===a})()`);
    assert.equal(same, true, 'same target preserves the existing move');
    await media(coarse, true);
    const reduce = await evaluate(`(async()=>{await update('${key}',80);return {count:bars.get('${key}').fill.getAnimations().length,edge:edge('${key}')}})()`);
    assert.equal(reduce.count, 0); near(reduce.edge, 160, 'reduced motion immediate');
    await dispose(key);
  }

  await media(true, true);
  await evaluate(`attach('marks',1,true)`);
  const minimum = await evaluate(`({edge:edge('marks'),placements:[...bars.get('marks').root.querySelectorAll('[data-placement]')].map(m=>m.dataset.placement)})`);
  near(minimum.edge, 16, 'coarse 16px minimum'); assert.deepEqual(minimum.placements, ['fill','track']);
  const markOnly = await evaluate(`(async()=>{const b=bars.get('marks'),m=b.root.querySelectorAll('.astryx-progress-bar-mark')[1];m.setAttribute('data-astryx-mark-percent','6');m.dataset.placement='track';await Promise.resolve();await Promise.resolve();return m.dataset.placement})()`);
  assert.equal(markOnly, 'fill', 'mark-only render updates minimum coverage without a progress change');
  await evaluate(`bars.get('marks').root.style.width='8px';new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve)))`);
  const capped = await evaluate(`({edge:edge('marks'),placements:[...bars.get('marks').root.querySelectorAll('[data-placement]')].map(m=>m.dataset.placement)})`);
  near(capped.edge, 8, 'minimum capped to track'); assert.deepEqual(capped.placements, ['fill','fill']);
  const zero = await evaluate(`(async()=>{await update('marks',0,'determinate',true);return {opacity:getComputedStyle(bars.get('marks').fill).opacity,placements:[...bars.get('marks').root.querySelectorAll('[data-placement]')].map(m=>m.dataset.placement)}})()`);
  assert.equal(zero.opacity, '0'); assert.deepEqual(zero.placements, ['track','track']);
  await dispose('marks');

  for (const coarse of [false, true]) {
    await media(coarse, false, true);
    await evaluate(`attach('forced',10)`);
    const forced = await evaluate(`(async()=>{await update('forced',1);const b=bars.get('forced');return {width:b.fill.getBoundingClientRect().width,transform:getComputedStyle(b.fill).transform,outline:getComputedStyle(b.fill).outlineWidth,count:b.fill.getAnimations().length}})()`);
    near(forced.width, coarse ? 16 : 2, 'forced colors sized fill'); assert.equal(forced.transform, 'none'); assert.equal(forced.outline, '1px'); assert.equal(forced.count, 0);
    await dispose('forced');
  }

  await media(false);
  const direction = await evaluate(`(async()=>{
    document.body.dir='rtl';await attach('direction',25);const b=bars.get('direction');b.root.dir='ltr';await update('direction',25);
    const ltr={direction:b.root.getAttribute('data-astryx-progress-bar-direction'),edge:edge('direction')};
    b.root.dir='rtl';await update('direction',50);for(const a of b.fill.getAnimations())a.finish();await Promise.resolve();
    const rtl={direction:b.root.getAttribute('data-astryx-progress-bar-direction'),edge:edge('direction')};
    b.root.dir='ltr';await Promise.resolve();const live=b.root.getAttribute('data-astryx-progress-bar-direction');
    document.body.dir='ltr';return {ltr,rtl,live};
  })()`);
  assert.equal(direction.ltr.direction, 'ltr'); near(direction.ltr.edge, 50, 'LTR inside RTL');
  assert.equal(direction.rtl.direction, 'rtl'); near(direction.rtl.edge, 100, 'RTL own region'); assert.equal(direction.live, 'ltr', 'direction updates without a new value');
  await dispose('direction');

  for (const coarse of [false, true]) {
    await media(coarse, true);
    await evaluate(`attach('pulse',30,true)`);
    const pulse = await evaluate(`(async()=>{
      await update('pulse',90,'indeterminate');const b=bars.get('pulse');b.root.style.setProperty('--_astryx-motion-tempo','2');
      const a=b.fill.getAnimations()[0];a.pause();const samples=[0,500,1000,1999.99,3000,3999.99].map(t=>{a.currentTime=t;const s=getComputedStyle(b.fill),r=b.fill.getBoundingClientRect();return {opacity:Number(s.opacity),width:r.width,transform:s.transform}});
      const timing=a.effect.getTiming();return {duration:timing.duration,easing:getComputedStyle(b.fill).animationTimingFunction,direction:timing.direction,samples,keys:Object.keys(a.effect.getKeyframes()[0])};
    })()`);
    assert.equal(pulse.duration, 2000); assert.equal(pulse.easing, 'cubic-bezier(0.42, 0, 0.58, 1)'); assert.equal(pulse.direction, 'alternate');
    for (const sample of pulse.samples) { near(sample.width, 200, 'full-width pulse'); assert.equal(sample.transform, 'none'); assert.ok(sample.opacity >= .84 && sample.opacity <= 1); }
    near(pulse.samples[0].opacity, 1, 'pulse full', .001); near(pulse.samples[2].opacity, .92, 'pulse midpoint', .001); near(pulse.samples[3].opacity, .84, 'pulse floor', .001); near(pulse.samples[5].opacity, 1, 'pulse return', .001);
    assert.ok(!pulse.keys.some(key=>/transform|width|outline|border/.test(key)), 'pulse animates opacity only');
    await media(coarse);
    const segment = await evaluate(`(()=>{const b=bars.get('pulse'),a=b.fill.getAnimations()[0];a.pause();a.currentTime=500;return {width:b.fill.getBoundingClientRect().width,duration:a.effect.getTiming().duration,transform:getComputedStyle(b.fill).transform}})()`);
    near(segment.width, 80, '40% segment'); assert.equal(segment.duration, 1500); assert.notEqual(segment.transform, 'none');
    await dispose('pulse');
  }

  // Direct shared-utility boundary: independent duplicate registrations and
  // one observer, without relying on asynchronous browser resize delivery.
  const resizeSource = await readFile(new URL('../src/utils/shared_resize_observer.js', import.meta.url), 'utf8');
  const shared = await evaluate(`(async()=>{
    const create = new Function(${JSON.stringify(resizeSource + '\nreturn observeResize;')});const observe=create();
    const original=window.ResizeObserver;let created=0,disconnected=0;
    window.ResizeObserver=class {constructor(){created++;}observe(){}unobserve(){}disconnect(){disconnected++;}};
    const el=document.createElement('div');let calls=0;const callback=()=>calls++;
    const a=observe(el,callback),b=observe(el,callback),c=create()(el,()=>calls++);
    a();a();b();const afterTwo=disconnected;c();window.ResizeObserver=original;
    return {created,disconnected,afterTwo,calls,state:!!document[Symbol.for('astryx.dioxus.sharedResizeObserver')]};
  })()`);
  assert.deepEqual(shared, {created:1,disconnected:1,afterTwo:0,calls:3,state:false});

  await media(false);
  const resources = await evaluate(`(async()=>{
    const add=MediaQueryList.prototype.addEventListener,remove=MediaQueryList.prototype.removeEventListener;
    const Observer=window.MutationObserver;
    let listeners=0,observers=0;
    MediaQueryList.prototype.addEventListener=function(...args){listeners++;return add.apply(this,args);};
    MediaQueryList.prototype.removeEventListener=function(...args){listeners--;return remove.apply(this,args);};
    window.MutationObserver=class extends Observer {observe(...args){observers++;return super.observe(...args);}disconnect(){observers--;return super.disconnect();}};
    await attach('resources',10);const mounted={listeners,observers};const b=bars.get('resources');
    let writes=0;const set=CSSStyleDeclaration.prototype.setProperty,attr=Element.prototype.setAttribute;
    CSSStyleDeclaration.prototype.setProperty=function(...args){if(this===b.root.style||this===b.fill.style)writes++;return set.apply(this,args);};
    Element.prototype.setAttribute=function(...args){if(this===b.root||this===b.fill)writes++;return attr.apply(this,args);};
    await update('resources',90);const afterUpdate=writes;
    for(let i=0;i<10;i++)await new Promise(requestAnimationFrame);
    const frameWrites=writes-afterUpdate;
    CSSStyleDeclaration.prototype.setProperty=set;Element.prototype.setAttribute=attr;
    await update('resources',50,'indeterminate');const switched={listeners,observers};
    await update('resources',20,'determinate');const resumed={listeners,observers};
    await disposeBar('resources');const dropped={listeners,observers};
    MediaQueryList.prototype.addEventListener=add;MediaQueryList.prototype.removeEventListener=remove;window.MutationObserver=Observer;
    await attach('fallback',10);const f=bars.get('fallback');f.fill.animate=undefined;await update('fallback',80);
    const fallback={edge:edge('fallback'),count:f.fill.getAnimations().length};await disposeBar('fallback');
    return {mounted,switched,resumed,dropped,frameWrites,fallback};
  })()`);
  assert.deepEqual(resources, {mounted:{listeners:3,observers:1},switched:{listeners:0,observers:1},resumed:{listeners:3,observers:1},dropped:{listeners:0,observers:0},frameWrites:0,fallback:{edge:160,count:0}});

  await media(true);
  const lifecycle = await evaluate(`(async()=>{
    await attach('lifecycle',10,true);await update('lifecycle',90,'determinate',true);const b=bars.get('lifecycle');
    const before={resize:document[Symbol.for('astryx.dioxus.sharedResizeObserver')].callbacks.size,animations:b.fill.getAnimations().length};
    await update('lifecycle',40,'indeterminate');
    const mode={resize:!!document[Symbol.for('astryx.dioxus.sharedResizeObserver')],native:b.fill.getAnimations().filter(a=>a instanceof Animation && !(a instanceof CSSAnimation)).length};
    await update('lifecycle',70,'determinate',true);b.root.remove();await b.done;
    const removed={resize:!!document[Symbol.for('astryx.dioxus.sharedResizeObserver')],private:b.root.hasAttribute('data-astryx-progress-bar-motion'),color:b.root.style.color,caller:b.root.dataset.caller,drawn:b.root.style.getPropertyValue('--_progress-bar-drawn')};
    bars.delete('lifecycle');await attach('closed',30,true);const c=bars.get('closed');c.push(null);await c.done;
    const closed={resize:!!document[Symbol.for('astryx.dioxus.sharedResizeObserver')],private:c.root.hasAttribute('data-astryx-progress-bar-motion')};c.root.remove();bars.delete('closed');return {before,mode,removed,closed};
  })()`);
  assert.deepEqual(lifecycle, {
    before:{resize:1,animations:1},mode:{resize:false,native:0},
    removed:{resize:false,private:false,color:'rgb(1, 2, 3)',caller:'preserved',drawn:''},closed:{resize:false,private:false},
  });
  console.log('ProgressBarTouch native boundary checks passed: theme timing/retarget/return, reduced motion, minimum/marks/resize, own-region direction, forced-color timing exception, CSS pulse/slide fixture, resource cleanup. No preview or public component E2E claims.');
} finally {
  ws.close();
  await fetch(`${endpoint}/json/close/${target.id}`);
}

async function dispose(key) { await evaluate(`disposeBar(${JSON.stringify(key)})`); }
