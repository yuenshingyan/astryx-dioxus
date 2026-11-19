// Deterministic Web Animations/RAF boundary tests; real DOM, no npm packages.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
const endpoint=process.env.CDP_URL??'http://127.0.0.1:9222';
const target=await(await fetch(`${endpoint}/json/new?about:blank`,{method:'PUT'})).json();
const ws=new WebSocket(target.webSocketDebuggerUrl);await new Promise(r=>ws.onopen=r);
let id=0;const requests=new Map();ws.onmessage=({data})=>{const m=JSON.parse(data),p=requests.get(m.id);if(p){requests.delete(m.id);m.error?p.reject(Error(JSON.stringify(m.error))):p.resolve(m.result)}};
const send=(method,params={})=>new Promise((resolve,reject)=>{requests.set(++id,{resolve,reject});ws.send(JSON.stringify({id,method,params}))});
try {
 const source=await readFile(new URL('../src/spinner/spinner_rotor.js',import.meta.url),'utf8');
 const result=await send('Runtime.evaluate',{awaitPromise:true,returnByValue:true,expression:`(async()=>{
  const frames=new Map(),calls=[];let count=0;
  window.requestAnimationFrame=fn=>{const key=++count;frames.set(key,fn);return key};window.cancelAnimationFrame=key=>frames.delete(key);
  const run=new (Object.getPrototypeOf(async function(){}).constructor)('dioxus',${JSON.stringify(source)});
  async function attach(name,hasApi=true){const element=document.createElement('span');element.setAttribute('data-astryx-spinner-rotor-id',name);document.body.append(element);if(hasApi)element.getAnimations=()=>{if(!element.isConnected)throw Error('touched disconnected rotor');calls.push('read:'+name);return [{set startTime(t){calls.push('write:'+name+':'+t)}}]};else element.getAnimations=undefined;
   let first=true,resolve;const done=run({recv:()=>first?(first=false,Promise.resolve(name)):new Promise(r=>resolve=r)});await Promise.resolve();await Promise.resolve();return {element,dispose:async(value='dispose')=>{resolve?.(value);await done}};
  }
  const a=await attach('a'),b=await attach('b'),c=await attach('c');
  const firstFrameCount=frames.size;b.element.remove();await b.dispose();
  for(const fn of frames.values())fn();frames.clear();
  const batchCalls=[...calls];calls.length=0;
  const d=await attach('d');d.element.remove();await d.dispose();const cancelled=frames.size;
  const noApi=await attach('no-api',false);await noApi.dispose();const noApiFrames=frames.size;
  await a.dispose(null);await c.dispose();
  const state=document[Symbol.for('astryx.dioxus.spinner.rotors')];return {firstFrameCount,batchCalls,cancelled,noApiFrames,pending:state.pending.size,frame:state.frame};
 })()`});
 assert.equal(result.exceptionDetails,undefined,JSON.stringify(result.exceptionDetails));
 assert.deepEqual(result.result.value,{firstFrameCount:1,batchCalls:['read:a','read:c','write:a:0','write:c:0'],cancelled:0,noApiFrames:0,pending:0,frame:null});
 console.log('Spinner rotor boundary checks passed: one frame, all reads before writes, disconnected/drop cancellation, no Web Animations fallback, null-channel cleanup.');
} finally {ws.close();await fetch(`${endpoint}/json/close/${target.id}`)}
