const [id, initialFallback] = await dioxus.recv();
const root = document.querySelector(`[data-astryx-media-owner="${CSS.escape(id)}"]`);
const surface = root?.parentElement;
if (!surface) return;
let fallback = initialFallback;
let frame = null;
const context = document.createElement('canvas').getContext('2d', {willReadFrequently: true});
const parse = color => {
  if (!context || !color || !CSS.supports('color', color)) return null;
  context.clearRect(0, 0, 1, 1);
  context.fillStyle = color;
  context.fillRect(0, 0, 1, 1);
  return Array.from(context.getImageData(0, 0, 1, 1).data, channel => channel / 255);
};
const over = (top, base) => base.map((value, i) => i === 3 ? 1 : top[i] * top[3] + value * (1 - top[3]));
const luminance = rgb => rgb.slice(0, 3).map(c => c <= .04045 ? c / 12.92 : ((c + .055) / 1.055) ** 2.4)
  .reduce((sum, c, i) => sum + c * [.2126, .7152, .0722][i], 0);
const contrast = (ink, background) => {
  const a = luminance(over(ink, background)), b = luminance(background);
  return (Math.max(a, b) + .05) / (Math.min(a, b) + .05);
};
const backdrop = () => {
  const layers = [];
  for (let element = surface; element; element = element.parentElement) {
    const style = getComputedStyle(element);
    if (style.backgroundImage !== 'none') return null;
    const color = parse(style.backgroundColor);
    if (color?.[3] === 1) return layers.reduceRight((base, top) => over(top, base), color);
    if (color?.[3]) layers.push(color);
  }
  return null;
};
const pick = () => {
  const background = backdrop();
  if (!background) return fallback;
  const probe = document.createElement('span');
  probe.hidden = true;
  surface.append(probe);
  try {
    const [ambient, dark, light] = ['--color-text-primary', '--color-on-dark', '--color-on-light'].map(token => {
      probe.style.color = `var(${token})`;
      return parse(getComputedStyle(probe).color);
    });
    if (!dark || !light) return fallback;
    if (ambient && contrast(ambient, background) >= 3) return 'off';
    return contrast(dark, background) >= contrast(light, background) ? 'dark' : 'light';
  } finally {
    probe.remove();
  }
};
const apply = mode => {
  if (mode === 'off') root.removeAttribute('data-astryx-media');
  else if (root.dataset.astryxMedia !== mode) root.dataset.astryxMedia = mode;
};
const measure = () => {
  frame = null;
  if (root.isConnected && root.dataset.astryxMediaMode === 'auto') apply(pick());
};
const schedule = () => { if (frame === null) frame = requestAnimationFrame(measure); };
const observer = new MutationObserver(schedule);
for (let node = surface; node; node = node.parentElement) {
  observer.observe(node, {attributes: true});
}
const scheme = matchMedia('(prefers-color-scheme: dark)');
scheme.addEventListener('change', schedule);
window.addEventListener('resize', schedule);
surface.addEventListener('transitionend', schedule);
try {
  measure();
  while (true) {
    const next = await dioxus.recv();
    if (next === false) break;
    fallback = next;
    schedule();
  }
} finally {
  if (frame !== null) cancelAnimationFrame(frame);
  observer.disconnect();
  scheme.removeEventListener('change', schedule);
  window.removeEventListener('resize', schedule);
  surface.removeEventListener('transitionend', schedule);
  const mode = root.dataset.astryxMediaMode;
  if (root.isConnected && mode !== 'auto') apply(mode);
}
