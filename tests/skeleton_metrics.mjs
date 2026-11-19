// Self-contained browser measurements and compact source decoder.
export function skeletonMetrics(root, frame = root.parentElement) {
  const round = n => Math.round(n * 1e6) / 1e6;
  const rect = (e, relativeTo) => {const a=e.getBoundingClientRect(),b=relativeTo.getBoundingClientRect();return [a.x-b.x,a.y-b.y,a.width,a.height].map(round);};
  const s = getComputedStyle(root);
  const names = ['--color-skeleton','--color-text-primary','--duration-medium-max',
    '--_astryx-pre-tempo-duration-medium-max','--_astryx-motion-tempo','--_astryx-reduced-motion',
    '--radius-none','--radius-inner','--radius-element','--radius-container','--radius-page','--radius-chat','--radius-full',
    '--spacing-1','--spacing-2','--spacing-3','--spacing-4','--border-width','--color-border',
    '--color-background-card','--color-background-muted','--color-background-surface','--astryx-card-padding',
    '--container-padding-inline-start','--container-padding-inline-end','--container-padding-block-start','--container-padding-block-end'];
  const effects = root.getAnimations().map(a => {
    const t=a.effect.getTiming();
    return {currentTime:a.currentTime,playState:a.playState,playbackRate:a.playbackRate,
      timing:{...t,iterations:t.iterations===Infinity?'infinite':t.iterations},
      keyframes:a.effect.getKeyframes().map(k=>({offset:round(k.offset),opacity:k.opacity,easing:k.easing,composite:k.composite}))};
  });
  const delayMs=Number.parseFloat(s.animationDelay)*1000;
  return {
    root:{tag:root.tagName.toLowerCase(),targets:[...root.classList].filter(n=>n.startsWith('astryx-')),
      ariaHidden:root.getAttribute('aria-hidden'),childCount:root.childElementCount},
    geometry:{relativeToParent:rect(root,root.parentElement),relativeToFrame:rect(root,frame)},
    style:{display:s.display,boxSizing:s.boxSizing,flexShrink:s.flexShrink,borderRadius:s.borderRadius,
      backgroundColor:s.backgroundColor,opacity:s.opacity},
    tokens:Object.fromEntries(names.map(n=>[n,s.getPropertyValue(n).trim()])),
    animation:{css:{name:s.animationName,duration:s.animationDuration,direction:s.animationDirection,
      iterations:s.animationIterationCount,easing:s.animationTimingFunction,fill:s.animationFillMode},
      cssDelay:s.animationDelay,delayMs,indexInferred:(delayMs-1000)/100,cssPlayState:s.animationPlayState,effects},
    provenance:{rawStyle:root.getAttribute('style'),dimensions:{width:root.style.getPropertyValue('--x-width').trim()||root.style.width,
      height:root.style.getPropertyValue('--x-height').trim()||root.style.height},
      animationDelay:root.style.getPropertyValue('--x-animationDelay').trim()||root.style.animationDelay},
  };
}

export function contextMetrics(scope, nativeRoot, blocks) {
  const round=n=>Math.round(n*1e6)/1e6;
  const rect=(e,p)=>{const a=e.getBoundingClientRect(),b=p.getBoundingClientRect();return [a.x-b.x,a.y-b.y,a.width,a.height].map(round);};
  const describe=e=>{
    const s=getComputedStyle(e);
    const properties=['display','boxSizing','width','height','minWidth','minHeight','maxWidth','maxHeight',
      'flexDirection','flexWrap','flexGrow','flexShrink','flexBasis','alignItems','justifyContent','alignSelf',
      'rowGap','columnGap','padding','margin','borderTopWidth','borderRightWidth','borderBottomWidth','borderLeftWidth',
      'borderTopStyle','borderRightStyle','borderBottomStyle','borderLeftStyle','borderTopColor','borderRightColor',
      'borderBottomColor','borderLeftColor','borderRadius','backgroundColor','boxShadow','overflowX','overflowY','aspectRatio'];
    return {tag:e.tagName.toLowerCase(),targets:[...e.classList].filter(n=>n.startsWith('astryx-')),
      attributes:Object.fromEntries(['data-direction','data-gap','data-wrap','data-variant','data-elevation'].filter(n=>e.hasAttribute(n)).map(n=>[n,e.getAttribute(n)])),
      rawStyle:e.getAttribute('style'),box:rect(e,nativeRoot),relativeToParent:rect(e,e.parentElement),
      style:Object.fromEntries(properties.map(n=>[n,s[n]])),
      tokens:Object.fromEntries(['--astryx-card-padding','--container-padding-inline-start','--container-padding-inline-end',
        '--container-padding-block-start','--container-padding-block-end','--spacing-1','--spacing-2','--spacing-3','--spacing-4',
        '--border-width','--radius-container','--color-border','--color-background-card','--color-background-muted'].map(n=>[n,s.getPropertyValue(n).trim()]))};
  };
  const frames=[];
  const visit=(e,parent)=>{
    const index=frames.length;
    const f={...describe(e),parent,children:[]};frames.push(f);
    f.children=[...e.children].map(c=>blocks.includes(c)?{block:blocks.indexOf(c)}:{frame:visit(c,index)});
    return index;
  };
  visit(nativeRoot,null);
  const presentation=[];
  for(let e=nativeRoot.parentElement;e&&e!==scope;e=e.parentElement)presentation.push(describe(e));
  return {theme:{name:scope.getAttribute('data-astryx-theme'),mode:scope.getAttribute('data-theme')},
    title:scope.querySelector('.astryx-text')?.textContent??null,
    caption:scope.querySelector('p.astryx-text')?.textContent??null,
    frames,presentation};
}

export function expandBlock(fixture, mode, index) {
  const b=fixture.cases[mode].blocks[index],p=fixture.profiles,a=b.animation,shared=p.animations[a.profile];
  return {root:p.roots[b.root],geometry:b.geometry,style:p.styles[b.style],tokens:p.tokens[b.tokens],
    provenance:b.provenance,animation:{css:shared.css,cssDelay:a.cssDelay,delayMs:a.delayMs,indexInferred:a.indexInferred,
      cssPlayState:a.cssPlayState,effects:shared.effects.map((e,i)=>({...e,timing:{...e.timing,delay:a.effectStates[i].delay},
        currentTime:a.effectStates[i].currentTime,playState:a.effectStates[i].playState,playbackRate:a.effectStates[i].playbackRate}))}};
}

export function selectSkeletonMetrics(value) {
 const {tokens,provenance,...result}=value;
 const {name,...css}=result.animation.css;
 return {...result,animation:{...result.animation,css}};
}
