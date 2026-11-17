// Source-color audit, not a substitute for renderer/assistive-technology tests.
// Palette and variant mappings derive from Meta Astryx; see THIRD_PARTY_NOTICES.md.
import { readFile } from 'node:fs/promises';

const palette = JSON.parse(await readFile(new URL('./fixtures/button-core-palette.json', import.meta.url), 'utf8'));
function splitTopLevel(value) {
  let depth = 0;
  for (let i = 0; i < value.length; i++) {
    if (value[i] === '(') depth++;
    else if (value[i] === ')') depth--;
    else if (value[i] === ',' && depth === 0) return [value.slice(0, i), value.slice(i + 1)];
  }
  throw new Error(`Expected a light/dark pair: ${value}`);
}
function color(value, mode) {
  value = value.trim();
  if (value.startsWith('var(')) return color(palette.tokens[value.slice(4, -1)], mode);
  if (value.startsWith('light-dark(')) return color(splitTopLevel(value.slice(11, -1))[mode], mode);
  if (value === 'transparent') return [0, 0, 0, 0];
  if (value.startsWith('#')) {
    let hex = value.slice(1);
    if (hex.length === 3 || hex.length === 4) hex = [...hex].map(character => character + character).join('');
    if (hex.length === 6) hex += 'ff';
    return [0, 2, 4, 6].map(index => parseInt(hex.slice(index, index + 2), 16) / 255);
  }
  if (/^rgba?\(/.test(value)) {
    const channels = value.slice(value.indexOf('(') + 1, -1).split(',').map(Number);
    return [...channels.slice(0, 3).map(channel => channel / 255), channels[3] ?? 1];
  }
  throw new Error(`Unsupported source color: ${value}`);
}
function over(foreground, background) {
  return [...foreground.slice(0, 3).map((channel, i) => channel * foreground[3] + background[i] * (1 - foreground[3])), 1];
}
function luminance(value) {
  const channels = value.slice(0, 3).map(channel => channel <= 0.04045 ? channel / 12.92 : ((channel + 0.055) / 1.055) ** 2.4);
  return channels.reduce((total, channel, i) => total + channel * [0.2126, 0.7152, 0.0722][i], 0);
}
function contrast(first, second) {
  const [low, high] = [luminance(first), luminance(second)].sort((a, b) => a - b);
  return (high + 0.05) / (low + 0.05);
}
const buttons = {
  primary: ['--color-accent', '--color-on-accent'],
  secondary: ['--color-neutral', '--color-text-primary'],
  ghost: ['transparent', '--color-text-primary'],
  destructive: ['--color-error', '--color-on-error'],
};
const badges = {
  neutral: ['--color-neutral', '--color-text-primary'],
  info: ['--color-info', '--color-on-info'],
  success: ['--color-success', '--color-on-success'],
  warning: ['--color-warning', '--color-on-warning'],
  error: ['--color-error', '--color-on-error'],
};
['blue', 'cyan', 'green', 'orange', 'pink', 'purple', 'red', 'teal', 'yellow'].forEach(name => {
  badges[name] = [`--color-background-${name}`, `--color-text-${name}`];
});
const report = {
  revision: palette.revision,
  scope: 'Upstream core default tokens and Button/Badge variant paint; no disabled states, custom themes, custom content, or font rasterization claims.',
  badgeCases: 0,
  badgeFailures: [],
  buttonLabelCases: 0,
  buttonLabelFailures: [],
};
['light', 'dark'].forEach((mode, modeIndex) => {
  const resolve = name => color(name.startsWith('--') ? palette.tokens[name] : name, modeIndex);
  ['--color-background-body', '--color-background-surface'].forEach(canvasName => {
    const canvas = resolve(canvasName);
    Object.entries(buttons).forEach(([button, [fill, foreground]]) => {
      const base = over(resolve(fill), canvas);
      ['rest', 'hover', 'pressed'].forEach(state => {
        const background = state === 'rest' ? base : over(resolve(`--color-overlay-${state}`), base);
        report.buttonLabelCases++;
        const labelRatio = contrast(resolve(foreground), background);
        const context = { mode, canvas: canvasName, button, state };
        if (labelRatio < 4.5) report.buttonLabelFailures.push({ ...context, ratio: +labelRatio.toFixed(4) });
        Object.entries(badges).forEach(([badge, [badgeFill, text]]) => {
          report.badgeCases++;
          const ratio = contrast(resolve(text), over(resolve(badgeFill), background));
          if (ratio < 4.5) report.badgeFailures.push({ ...context, badge, ratio: +ratio.toFixed(4) });
        });
      });
    });
  });
});
if (process.argv.includes('--json')) console.log(JSON.stringify(report, null, 2));
else {
  console.log(`Core source palette: ${report.badgeFailures.length}/${report.badgeCases} Badge combinations and ${report.buttonLabelFailures.length}/${report.buttonLabelCases} Button-label combinations fall below 4.5:1.`);
  report.buttonLabelFailures.forEach(failure => console.log(JSON.stringify(failure)));
  console.log('This reports upstream color conflicts; it does not claim the standalone Badge has been ported. Use --json for every failing combination.');
}
if (report.badgeFailures.length || report.buttonLabelFailures.length) process.exitCode = 1;
