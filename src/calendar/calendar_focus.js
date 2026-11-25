const id = await dioxus.recv();
const grid = document.getElementById(id);
if (!grid) return;
let pending;
const focusDate = date => grid.querySelector(`button[data-date="${date}"]:enabled`)?.focus();
const observer = new MutationObserver(() => {
  if (!pending) return;
  const date = pending;
  pending = undefined;
  if (grid.contains(document.activeElement) || document.activeElement === document.body) focusDate(date);
});
observer.observe(grid, {attributes: true, attributeFilter: ['data-month']});
const move = event => {
  const iso = event.target.dataset.date;
  const isPage = event.key === 'PageUp' || event.key === 'PageDown';
  if (!iso || event.altKey || event.metaKey || event.ctrlKey || (event.shiftKey && !isPage)) return;
  const rtl = getComputedStyle(grid).direction === 'rtl';
  const date = new Date(`${iso}T00:00:00Z`);
  const day = date.getUTCDate();
  const col = date.getUTCDay();
  const offsets = {ArrowLeft: rtl ? 1 : -1, ArrowRight: rtl ? -1 : 1, ArrowUp: -7, ArrowDown: 7, Home: -col, End: 6 - col};
  if (!isPage && !Object.hasOwn(offsets, event.key)) return;
  event.preventDefault();
  if (isPage) {
    const step = (event.key === 'PageUp' ? -1 : 1) * (event.shiftKey ? 12 : 1);
    date.setUTCDate(1);
    date.setUTCMonth(date.getUTCMonth() + step + 1, 0);
    date.setUTCDate(Math.min(day, date.getUTCDate()));
  } else {
    date.setUTCDate(day + offsets[event.key]);
  }
  if (date.getUTCFullYear() < 1 || date.getUTCFullYear() > 9999) return;
  let next = date.toISOString().slice(0, 10);
  const {min, max} = grid.dataset;
  if (event.key === 'Home' && min && next < min) next = min;
  if (event.key === 'End' && max && next > max) next = max;
  if ((min && next < min) || (max && next > max)) return;
  if (next.slice(0, 7) === iso.slice(0, 7)) focusDate(next);
  else {
    pending = next;
    dioxus.send(next);
  }
};
grid.addEventListener('keydown', move);
try { await dioxus.recv(); }
finally {
  observer.disconnect();
  grid.removeEventListener('keydown', move);
}
