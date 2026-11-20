// Offline original-source measurement and fixture helpers.
export function cardMetrics(root,nativeFrame=root,isShallow=false){
 const round=n=>Math.round(n*1e6)/1e6;
 const box=(e,p=nativeFrame)=>{const a=e.getBoundingClientRect(),b=p.getBoundingClientRect();return [a.x-b.x,a.y-b.y,a.width,a.height].map(round)};
 const props=['display','position','boxSizing','width','height','minWidth','minHeight','maxWidth','maxHeight','flexDirection','flexWrap','flexGrow','flexShrink','flexBasis','alignItems','alignSelf','justifyContent','gridTemplateColumns','gridTemplateRows','rowGap','columnGap','padding','paddingInlineStart','paddingInlineEnd','paddingBlockStart','paddingBlockEnd','margin','backgroundColor','backgroundImage','color','opacity','borderRadius','borderWidth','borderStyle','borderColor','outlineWidth','outlineStyle','outlineColor','outlineOffset','boxShadow','overflow','overflowX','overflowY','overflowWrap','clip','clipPath','transform','direction','fontFamily','fontSize','fontWeight','lineHeight','letterSpacing','fontVariantNumeric','whiteSpace','textOverflow','textAlign','transitionProperty','transitionDuration','transitionTimingFunction','animationName','animationDuration','forcedColorAdjust','colorScheme'];
 const nodes=[];
 const visit=e=>{const i=nodes.length,s=getComputedStyle(e),textRects=[],textRuns=[];
 for(const n of e.childNodes)if(n.nodeType===Node.TEXT_NODE&&n.textContent){const r=document.createRange();r.selectNodeContents(n);const b=nativeFrame.getBoundingClientRect();const rectangles=[...r.getClientRects()].map(a=>[a.x-b.x,a.y-b.y,a.width,a.height].map(round));textRects.push(...rectangles);textRuns.push({text:n.textContent,rectangles});}
 const tokens=Object.fromEntries([...s].filter(k=>k.startsWith('--')&&/^--(?:color-|spacing-|radius-|shadow-|border-|font-|text-|astryx-card-|container-|layout-padding-|_card-|_container-)/.test(k)).map(k=>[k,s.getPropertyValue(k).trim()]));
 const node={tag:e.tagName.toLowerCase(),targets:[...e.classList].filter(n=>n.startsWith('astryx-')),attributes:Object.fromEntries([...e.attributes].filter(a=>a.name!=='class'&&a.name!=='style').map(a=>[a.name,a.value])),rawStyle:e.getAttribute('style'),box:box(e),relativeToParent:e.parentElement?box(e,e.parentElement):null,clientSize:[e.clientWidth,e.clientHeight],scrollSize:[e.scrollWidth,e.scrollHeight],textRects,textRuns,style:Object.fromEntries(props.map(p=>[p,s[p]])),tokens,children:[]};nodes.push(node);
 node.children=isShallow?[]:[...e.childNodes].flatMap(n=>n.nodeType===Node.TEXT_NODE?[{text:n.textContent}]:n.nodeType===Node.ELEMENT_NODE?[{node:visit(n)}]:[]);return i;};
 return {tree:{node:visit(root)},nodes};
}

export function expandTree(fixture,entry){return {...entry,nodes:entry.nodes.map(n=>({...n,style:fixture.profiles.styles[n.style],tokens:fixture.profiles.tokens[n.tokens]})),...(entry.presentation?{presentation:entry.presentation.map(n=>({...n,style:fixture.profiles.styles[n.style],tokens:fixture.profiles.tokens[n.tokens]}))}:{}),...(entry.catalog?{catalog:Object.fromEntries(Object.entries(entry.catalog).map(([k,n])=>[k,n?{...n,style:fixture.profiles.styles[n.style],tokens:fixture.profiles.tokens[n.tokens]}:null]))}:{})};}

export function expandControl(fixture,mode,index){return expandTree(fixture,fixture.cases[mode].controls[index]);}

export function expandContext(fixture,mode,index){return expandTree(fixture,fixture.cases[mode].contexts[index]);}

export async function withTarget(run) {
  const endpoint='http://127.0.0.1:9222',browser=await (await fetch(endpoint+'/json/version')).json();
  const target=await (await fetch(endpoint+'/json/new?about:blank',{method:'PUT'})).json();
  const ws=new WebSocket(target.webSocketDebuggerUrl);
  let id=0;const pending=new Map(),exceptions=[];
  ws.onmessage=({data})=>{const m=JSON.parse(data);if(pending.has(m.id)){const p=pending.get(m.id);pending.delete(m.id);clearTimeout(p.timer);m.error?p.reject(new Error(JSON.stringify(m.error))):p.resolve(m.result);}if(m.method==='Runtime.exceptionThrown')exceptions.push(m.params.exceptionDetails);};
  const send=(method,params={})=>new Promise((resolve,reject)=>{const n=++id,timer=setTimeout(()=>{pending.delete(n);reject(new Error('CDP timeout '+method));},30000);pending.set(n,{resolve,reject,timer});ws.send(JSON.stringify({id:n,method,params}));});
  const evaluate=async expression=>{const r=await send('Runtime.evaluate',{expression,returnByValue:true,awaitPromise:true});if(r.exceptionDetails)throw new Error(JSON.stringify(r.exceptionDetails));return r.result.value;};
  const pause=ms=>new Promise(r=>setTimeout(r,ms));
  const waitFor=async expression=>{for(let i=0;i<150;i++){if(await evaluate(expression))return;await pause(200);}throw new Error('Wait failed '+expression);};
  try {
    await new Promise((resolve,reject)=>{ws.onopen=resolve;ws.onerror=reject;});
    await send('Page.enable');await send('Runtime.enable');
    await send('Emulation.setDeviceMetricsOverride',{width:1200,height:1600,deviceScaleFactor:1,mobile:false});
    return await run({browser,target,send,evaluate,pause,waitFor,exceptions});
  } finally {
    ws.close();await fetch(endpoint+'/json/close/'+target.id);
    for(let i=0;i<50;i++){if(!(await (await fetch(endpoint+'/json/list')).json()).some(t=>t.id===target.id))break;await pause(100);}
    if((await (await fetch(endpoint+'/json/list')).json()).some(t=>t.id===target.id))throw new Error('Own target not closed');
  }
}
