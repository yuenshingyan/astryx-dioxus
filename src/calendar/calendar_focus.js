const id = await dioxus.recv();
const grid = document.getElementById(id);
if (!grid) return;
const move = event => {
  const day = Number(event.target.dataset.day);
  if (!day || event.altKey || event.metaKey || event.ctrlKey || event.shiftKey) return;
  const rtl = getComputedStyle(grid).direction === 'rtl';
  const cells = grid.querySelectorAll('button[data-day]:enabled');
  if (!cells.length) return;
  const first = Number(cells[0].dataset.day);
  const last = Number(cells[cells.length - 1].dataset.day);
  const col = event.target.closest('td').cellIndex;
  const next = {
    ArrowLeft: day + (rtl ? 1 : -1), ArrowRight: day + (rtl ? -1 : 1),
    ArrowUp: day - 7, ArrowDown: day + 7, Home: Math.max(first, day - col), End: Math.min(last, day + 6 - col),
  }[event.key];
  if (typeof next !== 'number') return;
  event.preventDefault();
  grid.querySelector(`button[data-day="${next}"]:enabled`)?.focus();
};
grid.addEventListener('keydown', move);
try { await dioxus.recv(); }
finally { grid.removeEventListener('keydown', move); }
