// Offline Heading contract. Run with the shared browser lock; own target only.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {openTextPage} from './internal/text_cdp.mjs';
const page = await openTextPage();
const fixture = await openTextPage();
const url = process.env.HEADING_URL ?? 'http://127.0.0.1:8086';
const selectors = ['#heading-showcase', '#heading-page', '#heading-card', '#heading-truncation', '#text-heading-levels'];
const describe = `JSON.stringify([...document.querySelectorAll('${selectors.map(s=>s+' h1,'+s+' h2,'+s+' h3,'+s+' h4,'+s+' h5,'+s+' h6').join(',')}')].map(e=>{const s=getComputedStyle(e),r=e.getBoundingClientRect();return {text:e.textContent,tag:e.tagName,width:r.width,height:r.height,font:s.fontSize,leading:s.lineHeight,weight:s.fontWeight,family:s.fontFamily,color:s.color,display:s.display,wrap:s.overflowWrap,word:s.wordBreak,white:s.whiteSpace,ellipsis:s.textOverflow,clamp:s.webkitLineClamp}}))`;
try {
  await page.send('Emulation.setDeviceMetricsOverride', {width:1000,height:900,deviceScaleFactor:1,mobile:false});
  await fixture.send('Emulation.setDeviceMetricsOverride', {width:1000,height:900,deviceScaleFactor:1,mobile:false});
  await page.send('Page.addScriptToEvaluateOnNewDocument',{source:`window.headingWidthReads={};const d=Object.getOwnPropertyDescriptor(Element.prototype,'scrollWidth');Object.defineProperty(Element.prototype,'scrollWidth',{configurable:true,get(){if(this.id)window.headingWidthReads[this.id]=(window.headingWidthReads[this.id]||0)+1;return d.get.call(this)}});`});
  await page.send('Page.navigate', {url});
  await page.waitFor('document.querySelector("#heading-single") && document.fonts.status === "loaded"');
  let source = readFileSync(new URL('./fixtures/heading-source-reference.html', import.meta.url), 'utf8');
  const font = readFileSync(new URL('../assets/fonts/Figtree-latin.woff2', import.meta.url)).toString('base64');
  source = source.replace('</style>', `@font-face{font-family:Figtree;src:url(data:font/woff2;base64,${font});font-weight:300 900}</style>`);
  const tree = await fixture.send('Page.getFrameTree');
  await fixture.send('Page.setDocumentContent', {frameId:tree.frameTree.frame.id,html:source});
  await fixture.evaluate("document.fonts.load('600 24px Figtree').then(()=>document.fonts.ready).then(()=>true)");
  await page.evaluate("document.fonts.load('600 24px Figtree').then(()=>document.fonts.ready).then(()=>true)");
  for (const scheme of ['light','dark']) {
    await page.send('Emulation.setEmulatedMedia',{features:[{name:'prefers-color-scheme',value:scheme}]});
    await fixture.send('Emulation.setEmulatedMedia',{features:[{name:'prefers-color-scheme',value:scheme}]});
    await page.evaluate(`document.documentElement.style.colorScheme='${scheme}'`);
    await fixture.evaluate(`document.documentElement.style.colorScheme='${scheme}'`);
    const actual = JSON.parse(await page.evaluate(describe)), expected = JSON.parse(await fixture.evaluate(describe));
    assert.equal(actual.length,18); assert.equal(expected.length,18);
    for(let i=0;i<18;i++) {
      assert.ok(Math.abs(actual[i].width-expected[i].width)<.2,`${scheme} ${i} width ${JSON.stringify(actual[i])} != ${JSON.stringify(expected[i])}`);
      assert.ok(Math.abs(actual[i].height-expected[i].height)<.2,`${scheme} ${i} height`);
      delete actual[i].width;delete expected[i].width;delete actual[i].height;delete expected[i].height;
      assert.deepEqual(actual[i],expected[i],`${scheme} ${i}`);
    }
  }
  await page.evaluate(`document.documentElement.dataset.astryxTheme='neutral'`);
  assert.deepEqual(await page.evaluate(`Array.from(document.querySelectorAll('#heading-showcase .astryx-heading')).map(e=>getComputedStyle(e).fontWeight)`),['600','600','700','700','600','600']);
  await page.evaluate(`delete document.documentElement.dataset.astryxTheme;const s=document.createElement('style');s.id='consumer-heading-style';s.textContent='.astryx-heading[data-type="hero"]{font-size:50px}.astryx-heading[data-color="brand"]{color:rgb(80,40,120)}';document.head.append(s)`);
  assert.equal(await page.evaluate(`getComputedStyle(document.querySelector('#heading-custom')).fontSize`),'50px');
  assert.equal(await page.evaluate(`getComputedStyle(document.querySelector('#heading-custom')).color`),'rgb(80, 40, 120)');
  await page.evaluate(`document.querySelector('#consumer-heading-style').remove()`);
  await page.send('Page.bringToFront');
  await page.waitFor(`document.querySelector('#heading-single')[Symbol.for('astryx.dioxus.truncation')]?.isTruncated`);
  assert.equal(await page.evaluate(`document.querySelectorAll('[role=tooltip]').length`),0);
  assert.equal(await page.evaluate(`!!document.querySelector('#heading-single[title],#heading-single[tabindex],#heading-single[aria-describedby]')`),false);
  const paint = await page.evaluate(`(()=>{const s=id=>getComputedStyle(document.getElementById(id));return {sizes:[0,1,2].map(i=>parseFloat(s('display-'+i).fontSize)),weight:s('heading-bold').fontWeight,custom:s('heading-custom').fontSize,baseline:s('heading-single').fontSize,cap:s('heading-capsize').display,inline:s('heading-inline').display,align:s('heading-treatment').textAlign,decoration:s('heading-treatment').textDecorationLine}})()`);
  assert.ok(paint.sizes[0]>paint.sizes[1]&&paint.sizes[1]>paint.sizes[2]);
  assert.equal(paint.weight,'700');assert.equal(paint.custom,paint.baseline);
  assert.equal(paint.cap,'block');assert.equal(paint.inline,'inline');assert.equal(paint.align,'end');assert.equal(paint.decoration,'line-through');
  assert.equal(await page.evaluate(`document.querySelector('#heading-outline').getAttribute('aria-level')`),'4');
  assert.equal(await page.evaluate(`document.querySelector('#heading-raw-outline').getAttribute('aria-level')`),'5');
  const ax = await page.send('Accessibility.getFullAXTree');
  const heading = name=>ax.nodes.find(n=>n.role?.value==='heading'&&n.name?.value===name);
  for(let level=1;level<=6;level++) assert.equal(heading('Heading Level '+level).properties.find(p=>p.name==='level').value.value,level);
  assert.equal(heading('Outline override').properties.find(p=>p.name==='level').value.value,4);
  assert.equal(heading('Caller outline').properties.find(p=>p.name==='level').value.value,5);
  assert.equal(heading('Hero title').properties.find(p=>p.name==='level').value.value,2);
  // Caller handlers run and the native mount is stable across clamp changes.
  await page.evaluate(`(()=>{const e=document.querySelector('#heading-reactive');window.headingRoot=e;for(const [name,type] of [['click',MouseEvent],['wheel',WheelEvent],['copy',ClipboardEvent],['animationstart',AnimationEvent],['transitionend',TransitionEvent],['dragstart',DragEvent]])e.dispatchEvent(new type(name,{bubbles:true}));const input=document.querySelector('#heading-input');input.value='edited';input.focus();document.querySelector('#heading-change').click()})()`);
  await page.waitFor(`document.querySelector('#heading-reactive').textContent.startsWith('Changed')`);
  assert.equal(await page.evaluate(`document.querySelector('#heading-events').textContent`),'6');
  assert.equal(await page.evaluate(`document.querySelector('#heading-mounts').textContent`),'1');
  assert.equal(await page.evaluate(`window.headingRoot===document.querySelector('#heading-reactive')`),true);
  assert.equal(await page.evaluate(`document.querySelector('#heading-input').value`),'edited');
  assert.equal(await page.evaluate(`document.activeElement.id`),'heading-input');
  // Heading's own anchor wiring, not a second copy of Text measurement.
  await page.evaluate(`document.activeElement.blur();document.querySelector('#heading-single').scrollIntoView({block:'center'})`);
  const point = await page.evaluate(`(()=>{const r=document.querySelector('#heading-single').getBoundingClientRect();return {x:r.x+10,y:r.y+10}})()`);
  await page.send('Input.dispatchMouseEvent',{type:'mouseMoved',...point});
  await page.waitFor(`!!document.querySelector('[role=tooltip]')`).catch(async error=>{console.error(await page.evaluate(`(()=>{const e=document.querySelector('#heading-single');return {answer:e[Symbol.for('astryx.dioxus.truncation')],rect:e.getBoundingClientRect().toJSON(),hit:document.elementFromPoint(${point.x},${point.y})?.outerHTML,reads:window.headingWidthReads,observed:[...(document[Symbol.for('astryx.dioxus.sharedResizeObserver')]?.callbacks.keys()||[])].map(e=>({id:e.id,connected:e.isConnected,cache:e[Symbol.for('astryx.dioxus.truncation')]}))}})()`));throw error;});
  assert.equal(await page.evaluate(`document.querySelector('[role=tooltip]').textContent`),'Very Long Heading That Will Be Truncated To One Line With Ellipsis');
  await page.send('Input.dispatchMouseEvent',{type:'mouseMoved',x:999,y:0});
  await page.send('Input.dispatchKeyEvent',{type:'keyDown',key:'Escape',code:'Escape'});
  await page.send('Input.dispatchKeyEvent',{type:'keyUp',key:'Escape',code:'Escape'});
  await page.waitFor(`!document.querySelector('[role=tooltip]')`); // checkpoint bridge removes closed layers; documented shared gap
  assert.equal(await page.evaluate(`document.querySelector('#heading-multi')[Symbol.for('astryx.dioxus.truncation')].isTruncated`),true);
  assert.equal(await page.evaluate(`getComputedStyle(document.querySelector('#heading-multi')).webkitLineClamp`),'2');
  for(const dir of ['ltr','rtl']) {
    await page.evaluate(`document.querySelector('#heading-placement').style.marginLeft='400px';document.querySelector('#heading-placement').dir='${dir}';document.querySelector('#heading-placement').scrollIntoView({block:'center'})`);
    const anchor=await page.evaluate(`(()=>{const r=document.querySelector('#heading-placement').getBoundingClientRect();return {x:r.x+10,y:r.y+10}})()`);
    await page.send('Input.dispatchMouseEvent',{type:'mouseMoved',...anchor});
    await page.waitFor(`!!document.querySelector('[role=tooltip]')`);
    const rects=await page.evaluate(`(()=>{const a=document.querySelector('#heading-placement').getBoundingClientRect(),t=document.querySelector('[role=tooltip]').getBoundingClientRect();return {anchor:{left:a.left,right:a.right},tip:{left:t.left,right:t.right}}})()`);
    assert.ok(dir==='ltr'?rects.tip.left>rects.anchor.right:rects.tip.right<rects.anchor.left,`${dir} logical end: ${JSON.stringify(rects)}`);
    await page.send('Input.dispatchMouseEvent',{type:'mouseMoved',x:999,y:0});
    await page.send('Input.dispatchKeyEvent',{type:'keyDown',key:'Escape',code:'Escape'});
    await page.send('Input.dispatchKeyEvent',{type:'keyUp',key:'Escape',code:'Escape'});
    await page.waitFor(`!document.querySelector('[role=tooltip]')`);
  }
  assert.equal(await page.evaluate(`window.headingWidthReads['heading-disabled-tip']||0`),0);
  assert.ok(await page.evaluate(`window.headingWidthReads['heading-single']||0`)>0);
  assert.equal(await page.evaluate(`!!document.querySelector('#heading-disabled-tip')[Symbol.for('astryx.dioxus.truncation')]`),false);
  // Ordinary-flow reflow: isolate token to avoid fixed-width template frames.
  await page.send('Emulation.setDeviceMetricsOverride',{width:320,height:800,deviceScaleFactor:1,mobile:false});
  await page.evaluate(`document.querySelectorAll('#heading-patterns>section').forEach(e=>{if(e.id!=='heading-probes')e.hidden=true});document.querySelectorAll('#heading-probes>*').forEach(e=>{if(e.id!=='heading-token')e.style.display='none'});document.querySelector('#heading-probes').style.maxWidth='100%';document.querySelector('#heading-patterns').style.padding='8px'`);
  assert.equal(await page.evaluate(`document.documentElement.scrollWidth<=320`),true);
  const before = await page.evaluate(`document.querySelector('#heading-token').getBoundingClientRect().height`);
  await page.evaluate(`document.documentElement.style.fontSize='20px'`);
  assert.ok(await page.evaluate(`document.querySelector('#heading-token').getBoundingClientRect().height`)>before);
  assert.equal(await page.evaluate(`document.documentElement.scrollWidth<=320`),true);
  assert.deepEqual(page.errors,[]);assert.deepEqual(fixture.errors,[]);
  console.log('PASS Heading: 18 roots × light/dark source-derived geometry/paint; levels/AX, precedence, native events/mount stability, reactive clamp/input, hover wiring, 320px reflow/root scaling');
} finally {await page.close();await fixture.close();}
