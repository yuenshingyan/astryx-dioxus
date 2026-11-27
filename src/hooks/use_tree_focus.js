// Adapted from Astryx hooks/useTreeFocus.ts. See THIRD_PARTY_NOTICES.md.
const createTreeTypeahead = () => {
  let buffer = '';
  let lastKeyAt = -Infinity;
  return (event, items, index) => {
    if (event.key.length !== 1) return;
    const now = performance.now();
    const query = nextTypeaheadBuffer(now - lastKeyAt < 500 ? buffer : '', event.key);
    if (query === null) return;
    buffer = query;
    lastKeyAt = now;
    const start = index < 0 ? 0 : index + Number(query.length === 1);
    const ordered = [...items.slice(start), ...items.slice(0, start)]
      .filter(item => item.getAttribute('aria-disabled') !== 'true');
    return findTypeaheadMatch(ordered, item => item.getAttribute('aria-label') ?? '', query);
  };
};
