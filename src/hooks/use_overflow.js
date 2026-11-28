const id = await dioxus.recv();
const root = document.getElementById(id);
if (!root) return;
const measure = root.querySelector(':scope > .astryx-overflow-list__measure');
const visible = root.querySelector(':scope > .astryx-overflow-list__row');
let previous;
const update = () => {
  const nodes = Array.from(measure.children);
  const indicator = nodes.at(-1)?.hasAttribute('data-overflow-indicator') ? nodes.pop() : null;
  const widths = nodes.map(node => node.getBoundingClientRect().width);
  const style = getComputedStyle(visible);
  const gap = Number.parseFloat(style.columnGap) || 0;
  const available = Number.parseFloat(style.width) || 0;
  const floor = Math.min(widths.length, Number(root.dataset.min));
  let count = Math.min(widths.length, Math.max(floor, Number(root.dataset.max)));
  const fromStart = root.dataset.collapse === 'start';
  if (fromStart) widths.reverse();
  const indicatorWidth = indicator?.getBoundingClientRect().width ?? 0;
  let width = widths.slice(0, count).reduce((sum, item) => sum + item, 0);
  while (count > floor) {
    const hasIndicator = indicator && count < widths.length;
    const gaps = Math.max(0, count + Number(Boolean(hasIndicator)) - 1);
    if (width + gaps * gap + (hasIndicator ? indicatorWidth : 0) <= available + 0.01) break;
    width -= widths[--count];
  }
  const revision = Number(root.dataset.overflowRevision ?? 0);
  if (previous?.[0] !== count || previous?.[1] !== revision) {
    previous = [count, revision];
    dioxus.send(previous);
  }
};
const resize = new ResizeObserver(update);
const observe = () => {
  resize.disconnect();
  resize.observe(visible);
  for (const node of measure.children) resize.observe(node);
  update();
};
const mutations = new MutationObserver(observe);
mutations.observe(root, {attributes: true, attributeFilter: ['data-min', 'data-max', 'data-collapse', 'data-overflow-revision', 'style']});
mutations.observe(measure, {childList: true});
try {
  observe();
  await dioxus.recv();
} finally {
  mutations.disconnect();
  resize.disconnect();
}
