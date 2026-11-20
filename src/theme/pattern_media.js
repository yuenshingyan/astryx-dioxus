// Private measured MediaTheme slice for the Toast demo. Semantic overrides are
// owned by theme/onMediaTokens; no Toast-specific palette is introduced.
const applyDemoToastMedia = (card, media) => {
  delete media.dataset.astryxMedia;
  const resolve = value => {
    const probe = document.createElement('span');
    probe.style.color = value;
    card.append(probe);
    const rgb = getComputedStyle(probe).color.match(/[\d.]+/g)?.map(Number);
    probe.remove();
    return rgb?.length >= 3 ? rgb : null;
  };
  const luminance = rgb => rgb.slice(0,3).map(v => {
    const c = v / 255;
    return c <= .04045 ? c / 12.92 : ((c + .055) / 1.055) ** 2.4;
  }).reduce((n,c,i) => n + c * [.2126,.7152,.0722][i], 0);
  const contrast = (a,b) => (Math.max(a,b)+.05)/(Math.min(a,b)+.05);
  const style = getComputedStyle(card);
  const bg = style.backgroundColor.match(/[\d.]+/g)?.map(Number);
  const dark = resolve('var(--color-on-dark)'), light = resolve('var(--color-on-light)');
  const ambient = resolve('inherit');
  if (!bg || bg.length > 3 && bg[3] < 1 || !dark || !light || !ambient) return;
  const surface = luminance(bg);
  const darkRatio = contrast(surface,luminance(dark));
  const lightRatio = contrast(surface,luminance(light));
  if (contrast(surface,luminance(ambient)) >= Math.max(darkRatio,lightRatio)) return;
  media.dataset.astryxMedia = darkRatio > lightRatio ? 'dark' : 'light';
};
