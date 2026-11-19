// Read-only DOM measurements; no live-source requests or capture side effects.
export function selectKbdMetrics(value) {
  return {root:value.root,parentBackground:value.parentPresentation[0]?.backgroundColor??null,
    keys:value.keys.map(({text,ariaHidden,box,glyphBoxes,style})=>({text,ariaHidden,box,glyphBoxes,style}))};
}

export function kbdMetrics(root) {
  const round = n => Math.round(n * 1e6) / 1e6;
  const rect = (e, relativeTo = e) => {
    const a = e.getBoundingClientRect(), b = relativeTo.getBoundingClientRect();
    return [a.x-b.x,a.y-b.y,a.width,a.height].map(round);
  };
  const tokenNames = ['--color-neutral','--color-text-primary','--color-border-emphasized',
    '--radius-inner','--font-family-body','--font-weight-medium','--text-supporting-size',
    '--text-supporting-leading','--text-body-size','--text-body-leading','--spacing-1',
    '--spacing-2','--spacing-3','--spacing-4','--spacing-5','--radius-container',
    '--radius-element','--color-background-card','--color-border'];
  const tokens = e => {const s=getComputedStyle(e);return Object.fromEntries(tokenNames.map(n=>[n,s.getPropertyValue(n).trim()]));};
  const targets = e => [...e.classList].filter(s=>s.startsWith('astryx-'));
  const s=getComputedStyle(root), r=root.getBoundingClientRect(), rawTokens=tokens(root);
  const parentPresentation=[];
  for(let e=root.parentElement;e;e=e.parentElement){
    const p=getComputedStyle(e);
    if(p.backgroundColor!=='rgba(0, 0, 0, 0)') parentPresentation.push({tag:e.tagName.toLowerCase(),targets:targets(e),backgroundColor:p.backgroundColor});
    if(e.hasAttribute('data-astryx-theme'))break;
  }
  return {
    root:{role:root.getAttribute('role'),name:root.getAttribute('aria-label'),display:s.display,
      gap:s.gap,width:round(r.width),height:round(r.height),alignItems:s.alignItems,
      flexShrink:s.flexShrink,backgroundColor:s.backgroundColor,targets:targets(root)},
    rawTokens,parentPresentation,
    keys:[...root.querySelectorAll('kbd')].map(e=>{
      const s=getComputedStyle(e), range=document.createRange(); range.selectNodeContents(e);
      const b=e.getBoundingClientRect();
      const glyphBoxes=[...range.getClientRects()].map(a=>[a.x-b.x,a.y-b.y,a.width,a.height].map(round));
      const keyTokens=tokens(e);
      return {text:e.textContent,ariaHidden:e.getAttribute('aria-hidden'),targets:targets(e),
        box:rect(e,root),glyphBoxes,
        tokenOverrides:Object.fromEntries(tokenNames.filter(n=>keyTokens[n]!==rawTokens[n]).map(n=>[n,keyTokens[n]])),
        style:{display:s.display,firstFontFamily:s.fontFamily.split(',')[0].trim().replace(/^["']|["']$/g,''),
          fontSize:s.fontSize,fontWeight:s.fontWeight,lineHeight:s.lineHeight,color:s.color,
          backgroundColor:s.backgroundColor,borderBottomWidth:s.borderBottomWidth,
          borderBottomStyle:s.borderBottomStyle,borderBottomColor:s.borderBottomColor,
          borderRadius:s.borderRadius,minWidth:s.minWidth,height:s.height,minHeight:s.minHeight,
          paddingInlineStart:s.paddingInlineStart,paddingInlineEnd:s.paddingInlineEnd,
          userSelect:s.userSelect,boxSizing:s.boxSizing,alignItems:s.alignItems,justifyContent:s.justifyContent}
      };
    })
  };
}

export function expandControl(fixture, caseName, index) {
  const c=fixture.cases[caseName].controls[index];
  return {root:c.root,rawTokens:fixture.profiles.tokens[c.tokens],
    parentPresentation:fixture.profiles.parentPresentation[c.parentPresentation],
    keys:c.keys.map(({style,...k})=>({...k,style:fixture.profiles.keyStyles[style],tokenOverrides:{}}))};
}
