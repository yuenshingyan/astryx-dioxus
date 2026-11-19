// Self-contained metrics and compact actual-source decoders, no network/side effects.
export function statusDotMetrics(root, nativeFrame = root.parentElement) {
  const round=n=>Math.round(n*1e6)/1e6;
  const box=(e,p)=>{const a=e.getBoundingClientRect(),b=p.getBoundingClientRect();return [a.x-b.x,a.y-b.y,a.width,a.height].map(round);};
  const s=getComputedStyle(root);
  const props=['display','boxSizing','width','height','alignItems','justifyContent','flexShrink','borderRadius',
    'backgroundColor','color','opacity','borderWidth','borderStyle','outlineWidth','outlineStyle','outlineColor','outlineOffset','forcedColorAdjust'];
  const names=['--color-icon-success','--color-icon-warning','--color-icon-error','--color-icon-secondary',
    '--color-background-surface','--color-accent','--color-on-accent','--color-text-success','--color-text-warning','--color-text-error',
    '--color-text-primary','--color-background-body','--color-background-card','--color-background-muted',
    '--color-success','--color-warning','--color-error','--spacing-2','--font-family-body','--text-body-size','--text-body-leading','--text-body-weight'];
  const parentPaint=[];
  for(let e=root.parentElement;e;e=e.parentElement){const c=getComputedStyle(e);parentPaint.push({tag:e.tagName.toLowerCase(),
    targets:[...e.classList].filter(n=>n.startsWith('astryx-')),backgroundColor:c.backgroundColor,opacity:c.opacity});
    if(e.hasAttribute('data-astryx-theme'))break;}
  return {
    root:{tag:root.tagName.toLowerCase(),targets:[...root.classList].filter(n=>n.startsWith('astryx-')),
      dataAttributes:Object.fromEntries([...root.attributes].filter(a=>a.name.startsWith('data-')).map(a=>[a.name,a.value])),
      role:root.getAttribute('role'),label:root.getAttribute('aria-label'),tabIndexAttribute:root.getAttribute('tabindex'),tabIndex:root.tabIndex,
      title:root.getAttribute('title'),ariaDescribedBy:root.getAttribute('aria-describedby'),ariaExpanded:root.getAttribute('aria-expanded'),
      childCount:root.childElementCount,text:root.textContent,rawStyle:root.getAttribute('style')},
    geometry:{relativeToParent:box(root,root.parentElement),relativeToFrame:box(root,nativeFrame)},
    style:Object.fromEntries(props.map(p=>[p,s[p]])),
    tokens:Object.fromEntries(names.map(n=>[n,s.getPropertyValue(n).trim()])),parentPaint,
    animation:{css:{name:s.animationName,duration:s.animationDuration,easing:s.animationTimingFunction,iterations:s.animationIterationCount,
      delay:s.animationDelay,direction:s.animationDirection,fill:s.animationFillMode,playState:s.animationPlayState},
      effects:root.getAnimations().map(a=>{const t=a.effect.getTiming();return {type:a.constructor.name,name:a.animationName??null,
        currentTime:a.currentTime,playState:a.playState,playbackRate:a.playbackRate,
        timing:{...t,iterations:t.iterations===Infinity?'infinite':t.iterations},
        keyframes:a.effect.getKeyframes().map(k=>({offset:k.offset,opacity:k.opacity,easing:k.easing,composite:k.composite}))};})},
  };
}

export function nativeFrameMetrics(nativeFrame, roots, scope = nativeFrame.closest('[data-astryx-theme]')) {
  const round=n=>Math.round(n*1e6)/1e6;
  const box=(e,p)=>{const a=e.getBoundingClientRect(),b=p.getBoundingClientRect();return [a.x-b.x,a.y-b.y,a.width,a.height].map(round);};
  const describe=e=>{
    const s=getComputedStyle(e),props=['display','boxSizing','width','height','minWidth','minHeight','maxWidth','maxHeight',
      'flexDirection','flexWrap','flexGrow','flexShrink','flexBasis','alignItems','justifyContent','alignSelf','rowGap','columnGap',
      'padding','margin','borderWidth','borderStyle','borderColor','borderRadius','backgroundColor','boxShadow','overflow','aspectRatio',
      'fontFamily','fontSize','fontWeight','lineHeight','letterSpacing','color','textAlign','textWrapMode','whiteSpace','verticalAlign'];
    const textBoxes=[];
    for(const n of e.childNodes)if(n.nodeType===Node.TEXT_NODE&&n.textContent){const r=document.createRange();r.selectNodeContents(n);
      const b=e.getBoundingClientRect();for(const a of r.getClientRects())textBoxes.push([a.x-b.x,a.y-b.y,a.width,a.height].map(round));}
    return {tag:e.tagName.toLowerCase(),targets:[...e.classList].filter(n=>n.startsWith('astryx-')),
      dataAttributes:Object.fromEntries([...e.attributes].filter(a=>a.name.startsWith('data-')).map(a=>[a.name,a.value])),
      box:box(e,nativeFrame),relativeToParent:e.parentElement===scope?null:box(e,e.parentElement),textBoxes,
      style:Object.fromEntries(props.map(p=>[p,s[p]]))};
  };
  const frames=[];
  const visit=(e,parent)=>{const index=frames.length,f={...describe(e),parent,children:[]};frames.push(f);
    f.children=[...e.childNodes].flatMap(n=>n.nodeType===Node.TEXT_NODE?[{text:n.textContent}]:n.nodeType===Node.ELEMENT_NODE?
      [roots.includes(n)?{control:roots.indexOf(n)}:{frame:visit(n,index)}]:[]);return index;};
  visit(nativeFrame,null);
  const presentation=[];
  for(let e=nativeFrame.parentElement;e&&e!==scope;e=e.parentElement)presentation.push(describe(e));
  const title=[...scope.querySelectorAll('.astryx-text')].find(e=>/^StatusDot — /.test(e.textContent));
  const caption=scope.querySelector('p.astryx-text');
  const outerCard=scope.querySelector('.astryx-card');
  return {theme:{name:scope.getAttribute('data-astryx-theme'),mode:scope.getAttribute('data-theme')},frames,presentation,
    title:title?{text:title.textContent,...describe(title)}:null,caption:caption?{text:caption.textContent,...describe(caption)}:null,
    outerCard:outerCard?describe(outerCard):null};
}

export function expandControl(fixture, mode, index) {
  const c=fixture.cases[mode].controls[index],p=fixture.profiles;
  return {root:c.root,geometry:c.geometry,style:p.styles[c.style],tokens:p.tokens[c.tokens],
    parentPaint:p.parentPaint[c.parentPaint],animation:p.animations[c.animation]};
}

export function expandContext(fixture, mode, index) {
  const c=fixture.cases[mode].contexts[index],p=fixture.profiles;
  const expand=f=>f?{...f,style:p.frameStyles[f.style]}:null;
  return {...c,frames:c.frames.map(expand),presentation:c.presentation.map(expand),title:expand(c.title),caption:expand(c.caption),outerCard:expand(c.outerCard)};
}
