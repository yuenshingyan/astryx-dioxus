const id = await dioxus.recv();
const root = document.querySelector(`[data-astryx-chat-scroll="${CSS.escape(id)}"]`);
const area = root?.querySelector(':scope > .astryx-chat-layout__messages');
const dock = root?.querySelector(':scope > .astryx-chat-layout__dock');
if (!root || !area || !dock) return;
let isPinned = true;
let lastReported;
const report = () => {
  isPinned = root.scrollHeight - root.clientHeight - root.scrollTop <= 1;
  if (lastReported !== isPinned) {
    lastReported = isPinned;
    dioxus.send(!isPinned);
  }
};
const jump = () => {
  root.scrollTo({top: root.scrollHeight, behavior: 'instant'});
  report();
};
const resize = () => {
  if (isPinned) jump();
  else report();
};
root.addEventListener('scroll', report, {passive: true});
jump();
const stops = [root, area, dock].map(node => observeResize(node, resize, {afterLayout: true}));
try {
  while (await dioxus.recv()) {
    if (dock.querySelector('.astryx-chat-layout-scroll-button')?.contains(document.activeElement)) {
      (area.querySelector('[role="log"]') ?? root).focus({preventScroll: true});
    }
    jump();
  }
} finally {
  root.removeEventListener('scroll', report);
  stops.forEach(stop => stop());
}
