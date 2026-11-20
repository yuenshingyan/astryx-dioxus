// Actual-source DOM measurements and compact fixture decoders. No live requests.
const partNames = ['root','header','label','value','trackContainer','track','fill'];
export function progressBarMetrics(root, nativeFrame = root) {
  const round=n=>Math.round(n*1e6)/1e6;
  const box=(e,p=nativeFrame)=>{const a=e.getBoundingClientRect(),b=p.getBoundingClientRect();return [a.x-b.x,a.y-b.y,a.width,a.height].map(round);};
  const props=['display','position','boxSizing','width','height','minWidth','maxWidth','flexDirection','alignItems','justifyContent',
    'rowGap','columnGap','padding','margin','backgroundColor','color','opacity','borderRadius','borderWidth','borderStyle','borderColor',
    'outlineWidth','outlineStyle','outlineColor','outlineOffset','boxShadow','overflow','overflowX','overflowY','clip','clipPath',
    'transform','transformOrigin','willChange','direction','fontFamily','fontSize','fontWeight','lineHeight','letterSpacing','fontVariantNumeric',
    'whiteSpace','transitionProperty','transitionDuration','transitionTimingFunction','forcedColorAdjust'];
  const attrs=e=>Object.fromEntries([...e.attributes].filter(a=>a.name==='role'||a.name==='tabindex'||a.name.startsWith('aria-')||a.name.startsWith('data-')).map(a=>[a.name,a.value]));
  const node=e=>{
    if(!e)return null;
    const s=getComputedStyle(e),textRects=[];
    for(const n of e.childNodes)if(n.nodeType===Node.TEXT_NODE&&n.textContent){const r=document.createRange();r.selectNodeContents(n);
      const b=nativeFrame.getBoundingClientRect();for(const a of r.getClientRects())textRects.push([a.x-b.x,a.y-b.y,a.width,a.height].map(round));}
    return {tag:e.tagName.toLowerCase(),targets:[...e.classList].filter(n=>n.startsWith('astryx-')),attributes:attrs(e),
      text:e.textContent,rawStyle:e.getAttribute('style'),box:box(e),relativeToParent:box(e,e.parentElement),
      offsetSize:[e.offsetWidth,e.offsetHeight],textRects,style:Object.fromEntries(props.map(p=>[p,s[p]]))};
  };
  const track=root.querySelector('[role="progressbar"]'),fill=track?.querySelector('.astryx-progress-bar-fill');
  const label=document.getElementById(track?.getAttribute('aria-labelledby'));
  const header=label?.parentElement,value=header?.children.length>1?header.children[1]:null;
  const tokens=['--color-background-muted','--color-border-emphasized','--color-track','--color-background-body','--color-background-card','--color-background-surface',
    '--color-accent','--color-success','--color-warning','--color-error','--color-text-primary','--color-text-secondary','--color-text-disabled',
    '--color-on-accent','--color-on-success','--color-on-warning','--color-on-error','--spacing-1','--spacing-4','--radius-full',
    '--font-family-body','--text-body-size','--text-body-leading','--text-supporting-size','--text-supporting-leading',
    '--font-weight-medium','--font-weight-normal','--duration-medium','--ease-standard','--_astryx-motion-tempo'];
  const cs=getComputedStyle(root),paintAncestry=[];
  for(let e=root.parentElement;e;e=e.parentElement){const s=getComputedStyle(e);paintAncestry.push({tag:e.tagName.toLowerCase(),
    targets:[...e.classList].filter(n=>n.startsWith('astryx-')),backgroundColor:s.backgroundColor,opacity:s.opacity});if(e.hasAttribute('data-astryx-theme'))break;}
  const fs=getComputedStyle(fill),a=fill.getBoundingClientRect(),b=track.getBoundingClientRect();
  const x=Math.max(a.x,b.x),y=Math.max(a.y,b.y),right=Math.min(a.right,b.right),bottom=Math.min(a.bottom,b.bottom);
  return {root:node(root),header:node(header),label:node(label),value:node(value),trackContainer:node(track.parentElement),track:node(track),fill:node(fill),
    marks:[...root.querySelectorAll('.astryx-progress-bar-mark')].map(node),
    geometry:{fillRelativeToTrack:box(fill,track),fillTrackRectIntersection:[x-b.x,y-b.y,Math.max(0,right-x),Math.max(0,bottom-y)].map(round),
      intersectionIsNotRoundedPixelMask:true,clipOwner:'track; overflow hidden; rounded border box'},
    tokens:Object.fromEntries(tokens.map(n=>[n,cs.getPropertyValue(n).trim()])),paintAncestry,
    animation:{css:{name:fs.animationName,duration:fs.animationDuration,easing:fs.animationTimingFunction,iterations:fs.animationIterationCount,
      delay:fs.animationDelay,direction:fs.animationDirection,fill:fs.animationFillMode,playState:fs.animationPlayState},
      effects:fill.getAnimations().map(a=>{const t=a.effect.getTiming();return {type:a.constructor.name,name:a.animationName??null,
        currentTime:a.currentTime,playState:a.playState,playbackRate:a.playbackRate,
        timing:{...t,iterations:t.iterations===Infinity?'infinite':t.iterations},computedTiming:{progress:a.effect.getComputedTiming().progress,currentIteration:a.effect.getComputedTiming().currentIteration},
        keyframes:a.effect.getKeyframes().map(k=>({offset:k.offset,computedOffset:k.computedOffset,opacity:k.opacity??null,transform:k.transform??null,easing:k.easing,composite:k.composite}))};})}};
}

export function nativeFrameMetrics(nativeFrame, roots, scope = nativeFrame.closest('[data-astryx-theme]')) {
  const round=n=>Math.round(n*1e6)/1e6;
  const box=(e,p=nativeFrame)=>{const a=e.getBoundingClientRect(),b=p.getBoundingClientRect();return [a.x-b.x,a.y-b.y,a.width,a.height].map(round);};
  const describe=e=>{
    const s=getComputedStyle(e),props=['display','boxSizing','width','height','minWidth','minHeight','maxWidth','maxHeight',
      'flexDirection','flexWrap','flexGrow','flexShrink','flexBasis','alignItems','justifyContent','alignSelf','rowGap','columnGap',
      'padding','margin','borderWidth','borderStyle','borderColor','borderRadius','backgroundColor','boxShadow','overflow','aspectRatio',
      'fontFamily','fontSize','fontWeight','lineHeight','letterSpacing','color','textAlign','textWrapMode','whiteSpace','verticalAlign'];
    const textRects=[];
    for(const n of e.childNodes)if(n.nodeType===Node.TEXT_NODE&&n.textContent){const r=document.createRange();r.selectNodeContents(n);const b=nativeFrame.getBoundingClientRect();
      for(const a of r.getClientRects())textRects.push([a.x-b.x,a.y-b.y,a.width,a.height].map(round));}
    return {tag:e.tagName.toLowerCase(),targets:[...e.classList].filter(n=>n.startsWith('astryx-')),
      dataAttributes:Object.fromEntries([...e.attributes].filter(a=>a.name.startsWith('data-')).map(a=>[a.name,a.value])),rawStyle:e.getAttribute('style'),
      box:box(e),relativeToParent:box(e,e.parentElement),textRects,style:Object.fromEntries(props.map(p=>[p,s[p]]))};
  };
  const frames=[];
  const visit=e=>{
    if(roots.includes(e))return {control:roots.indexOf(e)};
    const i=frames.length,f={...describe(e),children:[]};frames.push(f);
    f.children=[...e.childNodes].flatMap(n=>n.nodeType===Node.TEXT_NODE?[{text:n.textContent}]:n.nodeType===Node.ELEMENT_NODE?[visit(n)]:[]);
    return {frame:i};
  };
  const tree=visit(nativeFrame),presentation=[];
  for(let e=nativeFrame.parentElement;e&&e!==scope;e=e.parentElement)presentation.push(describe(e));
  const title=[...scope.querySelectorAll('.astryx-text')].find(e=>/^ProgressBar — /.test(e.textContent)),caption=scope.querySelector('p.astryx-text'),card=scope.querySelector('.astryx-card');
  return {theme:{name:scope.getAttribute('data-astryx-theme'),mode:scope.getAttribute('data-theme')},tree,frames,presentation,
    title:title?{text:title.textContent,...describe(title)}:null,caption:caption?{text:caption.textContent,...describe(caption)}:null,outerCard:card?describe(card):null};
}

export function expandControl(fixture, mode, index) {
  const c=fixture.cases[mode].controls[index],p=fixture.profiles;
  const node=n=>n?{...n,style:p.styles[n.style]}:null;
  return {...c,...Object.fromEntries(partNames.map(k=>[k,node(c[k])])),marks:c.marks.map(node),tokens:p.tokens[c.tokens],paintAncestry:p.paintAncestry[c.paintAncestry],animation:p.animations[c.animation]};
}

export function expandContext(fixture, mode, index) {
  const c=fixture.cases[mode].contexts[index],p=fixture.profiles,frame=f=>f?{...f,style:p.frameStyles[f.style]}:null;
  return {...c,frames:c.frames.map(frame),presentation:c.presentation.map(frame),title:frame(c.title),caption:frame(c.caption),outerCard:frame(c.outerCard)};
}
