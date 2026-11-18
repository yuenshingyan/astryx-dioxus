// Source-color audit, not a substitute for renderer/assistive-technology tests.
// Palette and variant mappings derive from Meta Astryx; see THIRD_PARTY_NOTICES.md.
import { readFile } from 'node:fs/promises';

import { createColorResolver, over, contrast } from './contrast/colors.mjs';

const palette = JSON.parse(await readFile(new URL('./fixtures/button-core-palette.json', import.meta.url), 'utf8'));
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
  const resolve = createColorResolver(palette.tokens, modeIndex);
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
