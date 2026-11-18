// Mathematical source-palette audit, not a browser/AT compliance certificate.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

import { contrast, createColorResolver, over } from './contrast/colors.mjs';

const palette = JSON.parse(await readFile(new URL('./fixtures/button-core-palette.json', import.meta.url), 'utf8'));
assert.equal(contrast([0, 0, 0, 1], [1, 1, 1, 1]), 21);
const variants = {
  primary: ['--color-accent', '--color-on-accent'],
  secondary: ['--color-neutral', '--color-text-primary'],
  ghost: ['transparent', '--color-text-primary'],
  destructive: ['--color-error', '--color-on-error'],
};
const report = {
  revision: palette.revision,
  scope: 'Unchanged core tokens on page/surface, light/dark, rest/hover/pressed. Disabled exempt; arbitrary application content/themes excluded. Divider target applies only when needed to distinguish controls.',
  labelCases: 0, labelFailures: [],
  essentialIconAndSpinnerCases: 0, essentialIconAndSpinnerFailures: [],
  focusCases: 0, focusFailures: [],
  dividerCases: 0, dividersBelowConditionalTarget: [],
};
['light', 'dark'].forEach((mode, modeIndex) => {
  const resolve = createColorResolver(palette.tokens, modeIndex);
  ['--color-background-body', '--color-background-surface'].forEach(canvas => {
    const backdrop = resolve(canvas);
    Object.entries(variants).forEach(([variant, [fill, foreground]]) => {
      const base = over(resolve(fill), backdrop);
      const focusRatio = contrast(resolve(variant === 'destructive' ? '--color-error' : '--color-accent'), backdrop);
      report.focusCases++;
      if (focusRatio < 3) report.focusFailures.push({ mode, canvas, variant, ratio: +focusRatio.toFixed(4) });
      ['rest', 'hover', 'pressed'].forEach(state => {
        const background = state === 'rest' ? base : over(resolve(`--color-overlay-${state}`), base);
        const context = { mode, canvas, variant, state };
        const ratio = contrast(resolve(foreground), background);
        report.labelCases++;
        if (ratio < 4.5) report.labelFailures.push({ ...context, ratio: +ratio.toFixed(4) });
        report.essentialIconAndSpinnerCases++;
        if (ratio < 3) report.essentialIconAndSpinnerFailures.push({ ...context, ratio: +ratio.toFixed(4) });
        const divider = resolve(['primary', 'destructive'].includes(variant) ? '--color-on-accent' : '--color-border');
        const dividerRatio = contrast(over(divider, background), background);
        report.dividerCases++;
        if (dividerRatio < 3) report.dividersBelowConditionalTarget.push({ ...context, ratio: +dividerRatio.toFixed(4) });
      });
    });
  });
});
if (process.argv.includes('--json')) console.log(JSON.stringify(report, null, 2));
else {
  console.log(`ButtonGroup core palette: ${report.labelFailures.length}/${report.labelCases} text cases below 4.5:1; ${report.essentialIconAndSpinnerFailures.length}/${report.essentialIconAndSpinnerCases} inherited icon/arc cases below 3:1; ${report.focusFailures.length}/${report.focusCases} focus/backdrop cases below 3:1.`);
  console.log(`${report.dividersBelowConditionalTarget.length}/${report.dividerCases} divider cases below conditional 3:1. If a divider is necessary, the application must strengthen its theme; original paint is retained, not certified compliant.`);
}
if (report.labelFailures.length || report.essentialIconAndSpinnerFailures.length || report.focusFailures.length || report.dividersBelowConditionalTarget.length) process.exitCode = 1;
