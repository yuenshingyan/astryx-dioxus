// Internal Tooltip/StatusDot owner-boundary regression probes. No preview needed.
// Uses the real shared manager scripts in a dedicated blank CDP page. The
// StatusDot showcase suite separately covers the Rust renderer event boundary.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

const endpoint = process.env.CDP_URL ?? 'http://127.0.0.1:9222';
const target = await (await fetch(`${endpoint}/json/new?about:blank`, { method: 'PUT' })).json();
const socket = new WebSocket(target.webSocketDebuggerUrl);
await new Promise((resolve, reject) => {
  socket.addEventListener('open', resolve, { once: true });
  socket.addEventListener('error', reject, { once: true });
});
let sequence = 0;
const requests = new Map(), errors = [];
socket.addEventListener('message', ({ data }) => {
  const message = JSON.parse(data);
  if (message.method === 'Runtime.exceptionThrown') errors.push(message.params.exceptionDetails);
  const request = requests.get(message.id);
  if (!request) return;
  requests.delete(message.id); clearTimeout(request.timer);
  if (message.error) request.reject(new Error(message.error.message));
  else request.resolve(message.result);
});
function send(method, params = {}) {
  return new Promise((resolve, reject) => {
    const id = ++sequence;
    const timer = setTimeout(() => { requests.delete(id); reject(new Error(`CDP timeout: ${method}`)); }, 10000);
    requests.set(id, { resolve, reject, timer }); socket.send(JSON.stringify({ id, method, params }));
  });
}
async function evaluate(expression) {
  const result = await send('Runtime.evaluate', { expression, returnByValue: true, awaitPromise: true });
  assert.equal(result.exceptionDetails, undefined, JSON.stringify(result.exceptionDetails));
  return result.result.value;
}
async function waitFor(expression) {
  const deadline = Date.now() + 5000;
  while (Date.now() < deadline) {
    if (await evaluate(expression)) return;
    await new Promise(resolve => setTimeout(resolve, 20));
  }
  assert.fail(`Timed out: ${expression}`);
}
const scripts = await Promise.all([
  'src/hooks/use_announce/use_announce.js',
  'src/utils/hit_area.js', 'src/utils/touch_hit_area.js',
  'src/status_dot/status_dot.js', 'src/tooltip/use_tooltip.js',
  'src/hooks/use_press_feedback/press_feedback.js',
].map(path => readFile(path, 'utf8')));
try {
  await send('Runtime.enable');
  await send('Page.enable');
  await send('Page.bringToFront');
  await send('Emulation.setDeviceMetricsOverride', { width: 900, height: 900, deviceScaleFactor: 1, mobile: false });
  await evaluate(`(() => {
    ${scripts.join('\n')}
    document.body.innerHTML = '<div id="plain" role="img" aria-label="Static"></div><div id="room" style="display:flex;gap:48px;margin:60px"><span id="one" role="img" aria-label="Ready"></span><span id="two" role="img" aria-label="Ready"></span><button id="neighbor">Action</button></div><button id="host">Save <span id="nested" role="img" aria-label="Ready"></span><span id="duplicate" role="img" aria-label="Ready"></span></button><div id="dynamic">Dynamic </div><button id="explicit" aria-label="Caller host name"><span id="explicit-dot" role="img" aria-label="Ready"></span></button><div id="custom" role="note" tabindex="-1" aria-expanded="false" aria-controls="caller-control" aria-describedby="caller-description" aria-label="Caller dot name"></div><p id="caller-description">Caller description</p>';
    const style = document.createElement('style');
    style.textContent = 'span[aria-label],#custom {display:inline-flex;position:relative;width:8px;height:8px;background:green} [role=tooltip] {position:fixed;inset:auto;margin:0;padding:4px} @media(pointer:coarse) {[data-astryx-hit-area=true]::after {content:"";position:absolute;inset:calc(var(--astryx-hit-inset-top,0px)*-1) calc(var(--astryx-hit-inset-right,0px)*-1) calc(var(--astryx-hit-inset-bottom,0px)*-1) calc(var(--astryx-hit-inset-left,0px)*-1)}}';
    document.head.append(style);
    installAnnounce();
    installPressFeedback();
    window.runtime = document[runtimeKey]; window.handles = {};
    window.attach = (id, tooltip = 'Connection verified', callerAttributeNames = []) => {
      const element = document.getElementById(id); element.dataset.astryxStatusTooltipId = id;
      const tip = document.createElement('div'); tip.id = id + '-tooltip'; tip.setAttribute('role','tooltip'); tip.setAttribute('popover','manual'); tip.textContent = tooltip; element.after(tip);
      const handle = runtime.attach(element, { id, label: element.getAttribute('aria-label'), tooltip, callerAttributeNames, tooltipPolicyKey:'status-dot', tooltipPolicyFactory:createStatusDotTooltipSupport, holdHitArea: holdTouchHitArea });
      handles[id] = handle;
      return true;
    };
    ['one','two','nested','duplicate','explicit-dot'].forEach(id => attach(id));
    attach('custom','Caller-owned detail',['role','tabindex','aria-expanded','aria-controls','aria-describedby','aria-label']);
    window.dot = document.getElementById('one'); window.host = document.getElementById('host');
    return true;
  })()`);
  assert.equal(await evaluate('runtime.records.has(document.getElementById("plain"))'), false);
  assert.deepEqual(await evaluate('[dot.getAttribute("role"),dot.getAttribute("tabindex"),dot.getAttribute("aria-expanded"),dot.getAttribute("aria-label")]'), ['button','0','false','Ready']);
  assert.deepEqual(await evaluate('[document.getElementById("nested").getAttribute("role"),document.getElementById("nested").hasAttribute("tabindex")]'), ['img',false]);
  assert.equal(await evaluate('host.querySelectorAll("[data-astryx-status-host-name]").length'), 1, 'same tooltip in same host is named once');
  assert.equal(await evaluate('document.getElementById("explicit").querySelectorAll("[data-astryx-status-host-name]").length'), 0);
  assert.equal(await evaluate('document.getElementById("custom").outerHTML.includes(\'aria-describedby="caller-description"\')'), true);
  const tree = await send('Accessibility.getFullAXTree');
  assert.ok(tree.nodes.some(node => node.role?.value === 'button' && node.name?.value.includes('Save Ready Ready Connection verified')), 'host AX name carries tooltip text outside role img');

  await evaluate('(()=>{const label=document.createElement("span");label.id="hidden-name-reference";label.hidden=true;label.textContent="Connection verified";document.body.append(label);document.getElementById("nested").setAttribute("aria-labelledby",label.id)})()');
  await waitFor('!host.querySelector("[data-astryx-status-host-name]")');
  const referencedTree = await send('Accessibility.getFullAXTree');
  assert.equal(referencedTree.nodes.find(node=>node.role?.value==='button'&&node.name?.value.startsWith('Save '))?.name?.value.split('Connection verified').length-1,1,'hidden labelledby references already contribute the host name');
  await evaluate('document.getElementById("nested").removeAttribute("aria-labelledby");document.getElementById("hidden-name-reference").remove()');
  await waitFor('host.querySelectorAll("[data-astryx-status-host-name]").length===1');
  await evaluate('document.getElementById("nested").dispatchEvent(new PointerEvent("pointerover",{bubbles:true,pointerType:"mouse"}))');
  await waitFor('document.getElementById("nested-tooltip").matches(":popover-open")');
  const hoveredTree = await send('Accessibility.getFullAXTree');
  const hoveredName = hoveredTree.nodes.find(node => node.role?.value === 'button' && node.name?.value.startsWith('Save '))?.name?.value;
  assert.equal(hoveredName?.split('Connection verified').length - 1, 1, 'visible surface must not duplicate host name augmentation');
  await evaluate('handles.nested.hide();document.getElementById("explicit-dot").dispatchEvent(new PointerEvent("pointerover",{bubbles:true,pointerType:"mouse"}))');
  await waitFor('document.getElementById("explicit-dot-tooltip").matches(":popover-open")');
  await evaluate('document.getElementById("explicit-dot-tooltip").remove();handles["explicit-dot"].hide()');
  assert.equal(await evaluate('document.getElementById("explicit-dot-tooltip") === null'), true, 'hide must not resurrect a surface the renderer removed');
  await evaluate('dot.focus()');
  await waitFor('document.getElementById("one-tooltip").matches(":popover-open")');
  assert.equal(await evaluate('dot.getAttribute("aria-expanded")'), 'false', 'focus previews without pin');
  await evaluate('handles.one.command("enter")');
  assert.equal(await evaluate('dot.getAttribute("aria-expanded")'), 'true');
  await waitFor('document.querySelector("[data-astryx-live-region=polite]")?.textContent==="Connection verified"');
  await evaluate('handles.two.hide()');
  assert.equal(await evaluate('dot.getAttribute("aria-expanded")'),'true','another owner hide cannot dismiss this pin');
  await evaluate('handles.one.update({tooltip:"Updated while held"})');
  assert.equal(await evaluate('dot.getAttribute("aria-expanded")'),'true','text update preserves hold');
  assert.equal(await evaluate('document.getElementById("one-tooltip").textContent'),'Updated while held');
  await evaluate('handles.one.update({tooltip:"Connection verified"})');
  await evaluate('dot.dispatchEvent(new PointerEvent("pointerout",{bubbles:true,pointerType:"mouse",relatedTarget:document.body}))');
  await new Promise(resolve => setTimeout(resolve, 250));
  assert.equal(await evaluate('document.getElementById("one-tooltip").matches(":popover-open")'), true, 'pin survives leave');
  await evaluate('document.getElementById("two").dispatchEvent(new PointerEvent("pointerover",{bubbles:true,pointerType:"mouse"}))');
  await new Promise(resolve => setTimeout(resolve, 250));
  assert.equal(await evaluate('dot.getAttribute("aria-expanded")'), 'true', 'another hover cannot evict pin');
  await evaluate('document.dispatchEvent(new KeyboardEvent("keydown",{key:"Escape",bubbles:true,cancelable:true,isComposing:true}))');
  assert.equal(await evaluate('dot.getAttribute("aria-expanded")'), 'true');
  await evaluate('document.dispatchEvent(new KeyboardEvent("keydown",{key:"Escape",bubbles:true,cancelable:true}))');
  assert.equal(await evaluate('dot.getAttribute("aria-expanded")'), 'false');
  await evaluate('handles.one.command("space-down");handles.one.command("space-up")');
  assert.equal(await evaluate('dot.getAttribute("aria-expanded")'), 'true');
  await evaluate('document.body.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,isPrimary:true,button:0,pointerType:"mouse"}))');
  assert.equal(await evaluate('dot.getAttribute("aria-expanded")'), 'false');

  await evaluate('host.append(dot)');
  await waitFor('dot.dataset.astryxTooltipPlacement === "nested" && !dot.hasAttribute("tabindex")');
  assert.equal(await evaluate('dot.getAttribute("role")'), 'img');
  assert.equal(await evaluate('dot.hasAttribute("aria-expanded")'), false);
  assert.equal(await evaluate('host.querySelectorAll("[data-astryx-status-host-name]").length'), 1);
  await evaluate('handles.one.command("enter")');
  assert.equal(await evaluate('document.getElementById("one-tooltip").matches(":popover-open")'), false, 'nested cannot pin');
  await evaluate('host.setAttribute("aria-labelledby","caller-description")');
  await waitFor('!host.querySelector("[data-astryx-status-host-name]")');
  assert.equal(await evaluate('host.getAttribute("aria-labelledby")'), 'caller-description');
  await evaluate('host.removeAttribute("aria-labelledby")');
  await waitFor('host.querySelectorAll("[data-astryx-status-host-name]").length === 1');
  await evaluate('document.getElementById("dynamic").append(dot)');
  await waitFor('dot.dataset.astryxTooltipPlacement === "standalone"');
  await evaluate('document.getElementById("dynamic").setAttribute("role","button")');
  await waitFor('dot.dataset.astryxTooltipPlacement === "nested"');
  await evaluate('document.getElementById("dynamic").removeAttribute("role")');
  await waitFor('dot.dataset.astryxTooltipPlacement === "standalone"');

  await send('Emulation.setTouchEmulationEnabled', { enabled: true, maxTouchPoints: 1 });
  await waitFor('matchMedia("(pointer:coarse)").matches');
  await evaluate('document.getElementById("room").prepend(dot)');
  await waitFor('Number.parseFloat(dot.style.getPropertyValue("--astryx-hit-inset-left")) === 18');
  assert.equal(await evaluate('dot.getBoundingClientRect().width + Number.parseFloat(dot.style.getPropertyValue("--astryx-hit-inset-left")) + Number.parseFloat(dot.style.getPropertyValue("--astryx-hit-inset-right"))'), 44);
  await evaluate('document.getElementById("room").style.gap="8px"');
  await waitFor('Number.parseFloat(dot.style.getPropertyValue("--astryx-hit-inset-right")) === 4');
  assert.equal(await evaluate('Number.parseFloat(document.getElementById("two").style.getPropertyValue("--astryx-hit-inset-left"))'), 4, 'both targets stop at midpoint');
  await evaluate('dot.setAttribute("aria-disabled","true")');
  await waitFor('!dot.hasAttribute("data-astryx-hit-area") && !dot.style.getPropertyValue("--astryx-hit-inset-left")');
  await evaluate('dot.focus();handles.one.command("enter")');
  assert.equal(await evaluate('dot.getAttribute("aria-expanded")'), 'false');
  await evaluate('dot.removeAttribute("aria-disabled");document.getElementById("room").inert=true');
  await waitFor('!dot.hasAttribute("data-astryx-hit-area")');
  await evaluate('document.getElementById("room").inert=false');
  await waitFor('dot.getAttribute("data-astryx-hit-area") === "true"');

  await evaluate('(()=>{const two=document.getElementById("two");two.setAttribute("role","status");two.setAttribute("tabindex","3");two.setAttribute("aria-controls","native-controls");})()');
  await waitFor('document.getElementById("two").getAttribute("role") === "status"');
  await evaluate('handles.two.update({label:"Updated label"})');
  assert.deepEqual(await evaluate('(()=>{const two=document.getElementById("two");return [two.getAttribute("role"),two.getAttribute("tabindex"),two.getAttribute("aria-controls")];})()'), ['status','3','native-controls'], 'native caller attrs are not fought by the placement observer');
  await evaluate('handles.nested.update({tooltip:"Updated explanation"})');
  assert.equal(await evaluate('host.querySelectorAll("[data-astryx-status-host-name]").length'), 2);
  await evaluate('handles.duplicate.update({tooltip:"Updated explanation"})');
  assert.equal(await evaluate('host.querySelectorAll("[data-astryx-status-host-name]").length'), 1, 'text update releases old name and dedupes new name');
  await evaluate('handles.duplicate.dispose()');
  assert.equal(await evaluate('host.querySelectorAll("[data-astryx-status-host-name]").length'), 1);
  await evaluate('handles.nested.dispose()');
  await waitFor('!host.querySelector("[data-astryx-status-host-name]")');
  await evaluate('Object.entries(handles).filter(([id]) => !["duplicate","nested"].includes(id)).forEach(([,handle])=>handle.dispose())');
  await waitFor('!document[Symbol.for("astryx.dioxus.utils.hit-area")] && !document[Symbol.for("astryx.dioxus.utils.touch-hit-area")]');
  assert.equal(await evaluate('document[Symbol.for("astryx.dioxus.button.runtime")] === undefined'), true);
  assert.deepEqual(await evaluate('[dot.getAttribute("role"),dot.hasAttribute("tabindex"),dot.hasAttribute("aria-expanded"),dot.style.getPropertyValue("--astryx-hit-inset-left")]'), ['img',false,false,'']);
  assert.deepEqual(await evaluate('(()=>{const dot=document.getElementById("custom");return [dot.getAttribute("role"),dot.getAttribute("tabindex"),dot.getAttribute("aria-expanded"),dot.getAttribute("aria-controls"),dot.getAttribute("aria-describedby"),dot.getAttribute("aria-label")];})()'), ['note','-1','false','caller-control','caller-description','Caller dot name']);
  assert.deepEqual(await evaluate('(()=>{const two=document.getElementById("two");return [two.getAttribute("role"),two.getAttribute("tabindex"),two.getAttribute("aria-controls")];})()'), ['status','3','native-controls'], 'cleanup preserves native caller attrs');
  assert.deepEqual(errors, []);
  console.log('StatusDot internal Tooltip native-placement, pin lifecycle, AX names, shared hit-area and cleanup probes passed.');
} finally {
  await send('Page.close').catch(() => {});
  socket.close();
}
