const id = await dioxus.recv();
const root = document.getElementById(id);
const scroller = root?.querySelector(':scope > .astryx-carousel__scroller');
if (!scroller) return;

const rtl = () => getComputedStyle(scroller).direction === 'rtl';
const update = () => {
  const max = scroller.scrollWidth - scroller.clientWidth;
  const position = Math.max(0, rtl() ? -scroller.scrollLeft : scroller.scrollLeft);
  scroller.tabIndex = max > 1 ? 0 : -1;
  for (const button of root.querySelectorAll(':scope > [data-carousel-step]')) {
    const canScroll = button.dataset.carouselStep === 'previous' ? position > 1 : position < max - 1;
    if (!canScroll && document.activeElement === button) scroller.focus({preventScroll: true});
    button.hidden = !canScroll;
  }
};
const click = event => {
  const button = event.target.closest('[data-carousel-step]');
  if (!button || button.parentElement !== root || button.hidden) return;
  const direction = button.dataset.carouselStep === 'previous' ? -1 : 1;
  const behavior = matchMedia('(prefers-reduced-motion: reduce)').matches ? 'instant' : 'smooth';
  scroller.scrollBy({left: direction * (rtl() ? -1 : 1) * scroller.clientWidth, behavior});
};
const resize = new ResizeObserver(update);
const observe = () => {
  resize.disconnect();
  resize.observe(scroller);
  for (const slide of scroller.children) resize.observe(slide);
  update();
};
const mutations = new MutationObserver(observe);
mutations.observe(scroller, {childList: true});
mutations.observe(root, {childList: true});
scroller.addEventListener('scroll', update, {passive: true});
root.addEventListener('click', click);
try {
  observe();
  await dioxus.recv();
} finally {
  resize.disconnect();
  mutations.disconnect();
  scroller.removeEventListener('scroll', update);
  root.removeEventListener('click', click);
}
