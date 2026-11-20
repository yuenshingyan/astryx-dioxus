// Pinned theme/tokens.stylex.ts defaults; source palette is intentionally unchanged.
import assert from 'node:assert/strict';
const rgb=hex=>hex.match(/../g).map(v=>parseInt(v,16)/255);
const lum=c=>c.map(v=>v<=.04045?v/12.92:((v+.055)/1.055)**2.4).reduce((n,v,i)=>n+v*[.2126,.7152,.0722][i],0);
const contrast=(a,b)=>{const x=lum(a),y=lum(b);return (Math.max(x,y)+.05)/(Math.min(x,y)+.05)};
const mix=(a,b,t)=>a.map((v,i)=>v*t+b[i]*(1-t));
let failures=0;
for(const [mode,primary,secondary,muted,alpha,body,surface] of [
 ['light','0a1317','4e606f','053659',12/255,'f1f4f7','ffffff'],
 ['dark','dfe2e5','aaafb5','111112',127/255,'111112','1f1f22'],
])for(const [backdrop,color] of [['body',body],['surface',surface]])for(const [axis,ink] of [['primary',primary],['secondary',secondary]]){
 const ratio=contrast(rgb(ink),mix(rgb(muted),rgb(color),alpha));
 const has_passed=ratio>=4.5;if(!has_passed)failures++;
 console.log(`${mode}/${backdrop}/${axis}: ${ratio.toFixed(3)}:1 ${has_passed?'PASS':'FAIL'} (normal text 4.5:1)`);
}
assert.equal(failures,0,'conditional source-palette Code text contrast failures');
console.log('8 modeled default-palette cases passed. Inherited/custom color, arbitrary product backdrops, platform rendering and forced-color visual distinctions are not certified. Compact background is decorative, not an essential control boundary.');
