// sRGB source-token calculations shared by component contrast audits.
function splitTopLevel(value) {
  let depth = 0;
  for (let i = 0; i < value.length; i++) {
    if (value[i] === '(') depth++;
    else if (value[i] === ')') depth--;
    else if (value[i] === ',' && depth === 0) return [value.slice(0, i), value.slice(i + 1)];
  }
  throw new Error(`Expected a light/dark pair: ${value}`);
}
export function createColorResolver(tokens, mode) {
  function color(value) {
    if (typeof value !== 'string') throw new Error('Missing source color token');
    value = value.trim();
    if (value.startsWith('--')) return color(tokens[value]);
    if (value.startsWith('var(')) return color(tokens[value.slice(4, -1)]);
    if (value.startsWith('light-dark(')) return color(splitTopLevel(value.slice(11, -1))[mode]);
    if (value === 'transparent') return [0, 0, 0, 0];
    if (value.startsWith('#')) {
      let hex = value.slice(1);
      if (hex.length === 3 || hex.length === 4) hex = [...hex].map(character => character + character).join('');
      if (hex.length === 6) hex += 'ff';
      if (!/^[0-9a-f]{8}$/i.test(hex)) throw new Error(`Invalid source hex color: ${value}`);
      return [0, 2, 4, 6].map(index => parseInt(hex.slice(index, index + 2), 16) / 255);
    }
    if (/^rgba?\(/.test(value)) {
      const channels = value.slice(value.indexOf('(') + 1, -1).split(',').map(Number);
      if (![3, 4].includes(channels.length) || !channels.every(Number.isFinite)) throw new Error(`Invalid source RGB color: ${value}`);
      return [...channels.slice(0, 3).map(channel => channel / 255), channels[3] ?? 1];
    }
    throw new Error(`Unsupported source color: ${value}`);
  }
  return color;
}
export function over(foreground, background) {
  return [...foreground.slice(0, 3).map((channel, i) => channel * foreground[3] + background[i] * (1 - foreground[3])), 1];
}
function luminance(value) {
  const channels = value.slice(0, 3).map(channel => channel <= 0.04045 ? channel / 12.92 : ((channel + 0.055) / 1.055) ** 2.4);
  return channels.reduce((total, channel, i) => total + channel * [0.2126, 0.7152, 0.0722][i], 0);
}
export function contrast(first, second) {
  const [low, high] = [luminance(first), luminance(second)].sort((a, b) => a - b);
  return (high + 0.05) / (low + 0.05);
}
