// Adapted from Astryx utils/isApplePlatform.ts. See THIRD_PARTY_NOTICES.md.
function isApplePlatform() {
  if (typeof navigator === 'undefined') return false;
  const data = 'userAgentData' in navigator ? navigator.userAgentData : null;
  if (data && typeof data === 'object' && 'platform' in data) {
    const named = typeof data.platform === 'string' ? data.platform.trim() : '';
    if (named !== '' && named.toLowerCase() !== 'unknown') return /mac/i.test(named);
  }
  return /Mac|iPhone|iPad|iPod/.test(navigator.platform ?? '');
}
