const id = await dioxus.recv();
const item = document.querySelector(`[data-astryx-breadcrumb-item="${CSS.escape(id)}"]`);
const list = item?.parentElement;
if (!list?.matches('ol')) return;
let owned;
const update = () => {
  const hasExplicit = [...list.children].some(child => child.dataset.current === 'current');
  const next = item.parentElement === list && item === list.lastElementChild
    && item.dataset.current === 'auto' && !hasExplicit
    ? item.querySelector(':scope > .astryx-breadcrumb-item__content') : null;
  if (owned && owned !== next && owned.parentElement?.dataset.current !== 'current') {
    owned.removeAttribute('aria-current');
  }
  owned = next;
  if (owned && owned.getAttribute('aria-current') !== 'page') owned.setAttribute('aria-current', 'page');
};
const observer = new MutationObserver(update);
observer.observe(list, {childList: true, subtree: true, attributes: true, attributeFilter: ['data-current']});
try {
  update();
  await dioxus.recv();
} finally {
  observer.disconnect();
  if (owned && owned.parentElement?.dataset.current !== 'current') owned.removeAttribute('aria-current');
}
