// Source-token modeled contrast, not published measurements or an AX certification.
import assert from 'node:assert/strict';
import {coreTokens,neutralTokens} from './blockquote_source.mjs';
const paint=(value,mode)=>{const m=value.match(/light-dark\((#[0-9a-f]+),\s*(#[0-9a-f]+)\)/i);assert.ok(m,value);return m[mode+1];};
const luminance=hex=>{const c=hex.slice(1).match(/../g).slice(0,3).map(v=>parseInt(v,16)/255).map(v=>v<=.04045?v/12.92:((v+.055)/1.055)**2.4);return .2126*c[0]+.7152*c[1]+.0722*c[2];};
const ratio=(a,b)=>{const x=luminance(a),y=luminance(b);return(Math.max(x,y)+.05)/(Math.min(x,y)+.05);};
let failures=0;const rows=[];
for(const [theme,tokens]of Object.entries({core:coreTokens,neutral:neutralTokens}))for(const [modeIndex,mode]of ['light','dark'].entries())for(const surface of ['body','surface','card']){
 const background=paint(tokens['--color-background-'+surface],modeIndex),text=ratio(paint(tokens['--color-text-secondary'],modeIndex),background),rule=ratio(paint(tokens['--color-border-emphasized'],modeIndex),background);
 if(text<4.5)failures++;
 rows.push({theme,mode,surface,text:+text.toFixed(3),decorativeRule:+rule.toFixed(3)});
}
console.table(rows);
console.log(`Blockquote: ${rows.length-failures}/${rows.length} modeled secondary quote/cite text cases meet 4.5:1. Rule ratios are informational: the source uses a supporting decorative rule, not an essential control boundary. Caller content/themes remain unaudited.`);
process.exitCode=failures?1:0;
