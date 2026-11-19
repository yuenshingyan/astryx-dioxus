// DOM measurement helper only; no live-source requests or capture side effects.
// Select public-contract geometry/paint, not site chrome or no-op reset styles.
export function selectDividerMetrics(value) {
  const pick = (object, keys) => Object.fromEntries(keys.map(key => [key, object[key]]));
  const root = ['width','height','boxSizing','display','direction','flexDirection','flexGrow','flexShrink','flexBasis','alignItems','alignSelf','margin','padding'];
  const rule = ['width','height','backgroundColor','flexGrow','flexShrink','relativeToRootBorder','role','ariaHidden','tabIndex'];
  const label = ['text','width','height','padding','color','fontSize','lineHeight','fontWeight','firstFontFamily','flexShrink','relativeToRootBorder'];
  const frame = entry => entry ? {
    box: pick(entry.box,['width','height','contentWidth','contentHeight','padding','borderWidth','borderRadius','backgroundColor','rowGap','columnGap']),
    rootRelativeToBorder: entry.rootRelativeToBorder,
    rootRelativeToContent: entry.rootRelativeToContent,
  } : null;
  return {
    template:value.template,control:value.control,theme:value.theme,
    attributes:pick(value.attributes,['dataOrientation','dataVariant','role','ariaOrientation','ariaLabel','labelledByResolvesToLabel','labelText']),
    root:pick(value.root,root),ruleCount:value.ruleCount,
    rules:value.rules.map(value=>pick(value,rule)),label:value.label?pick(value.label,label):null,
    frames:{layout:frame(value.frames.layout),card:value.template==='DividerShowcase'?null:frame(value.frames.card),section:value.template==='DividerShowcase'?null:frame(value.frames.section)},
  };
}

export function dividerMetrics(root, identity = {}) {
  const round = value => Math.round(value * 1e6) / 1e6;
  const numeric = value => Number.parseFloat(value) || 0;
  const logical = (style, prefix, suffix = '') => Object.fromEntries(
    ['InlineStart', 'InlineEnd', 'BlockStart', 'BlockEnd'].map(edge => [
      edge[0].toLowerCase() + edge.slice(1), style[prefix + edge + suffix],
    ]),
  );
  const tokens = element => {
    const style = getComputedStyle(element);
    const names = [
      '--color-border', '--color-border-emphasized', '--color-text-secondary',
      '--color-text-primary', '--color-background-card', '--color-background-muted',
      '--color-background-surface', '--border-width', '--spacing-1', '--spacing-3',
      '--spacing-4', '--text-supporting-size', '--text-supporting-leading',
      '--radius-container', '--radius-element', '--astryx-card-padding',
      '--astryx-section-padding', '--container-padding-inline-start',
      '--container-padding-inline-end', '--container-padding-block-start',
      '--container-padding-block-end', '--layout-padding-inner-x',
      '--layout-padding-inner-y', '--layout-padding-outer-x', '--layout-padding-outer-y',
    ];
    return Object.fromEntries(names.map(name => [name, style.getPropertyValue(name).trim()]));
  };
  const box = element => {
    const style = getComputedStyle(element);
    const rect = element.getBoundingClientRect();
    return {
      width: round(rect.width), height: round(rect.height),
      contentWidth: round(rect.width - numeric(style.borderLeftWidth) - numeric(style.borderRightWidth)
        - numeric(style.paddingLeft) - numeric(style.paddingRight)),
      contentHeight: round(rect.height - numeric(style.borderTopWidth) - numeric(style.borderBottomWidth)
        - numeric(style.paddingTop) - numeric(style.paddingBottom)),
      cssWidth: style.width, cssHeight: style.height, maxWidth: style.maxWidth,
      minWidth: style.minWidth, minHeight: style.minHeight, boxSizing: style.boxSizing,
      display: style.display, direction: style.direction, writingMode: style.writingMode,
      flexDirection: style.flexDirection, flexGrow: style.flexGrow, flexShrink: style.flexShrink,
      flexBasis: style.flexBasis, flexWrap: style.flexWrap, alignItems: style.alignItems,
      alignSelf: style.alignSelf, justifyContent: style.justifyContent,
      rowGap: style.rowGap, columnGap: style.columnGap,
      margin: logical(style, 'margin'), padding: logical(style, 'padding'),
      borderWidth: logical(style, 'border', 'Width'), borderStyle: logical(style, 'border', 'Style'),
      borderColor: logical(style, 'border', 'Color'), borderRadius: style.borderRadius,
      color: style.color, backgroundColor: style.backgroundColor, boxShadow: style.boxShadow,
      overflowX: style.overflowX, overflowY: style.overflowY,
      fontFamily: style.fontFamily,
      firstFontFamily: style.fontFamily.split(',')[0].trim().replace(/^["']|["']$/g, ''),
      fontSize: style.fontSize, lineHeight: style.lineHeight, fontWeight: style.fontWeight,
      letterSpacing: style.letterSpacing,
    };
  };
  const relative = (element, container, edge) => {
    const a = element.getBoundingClientRect();
    const b = container.getBoundingClientRect();
    const style = getComputedStyle(container);
    const content = edge === 'content';
    const left = b.left + (content ? numeric(style.borderLeftWidth) + numeric(style.paddingLeft) : 0);
    const right = b.right - (content ? numeric(style.borderRightWidth) + numeric(style.paddingRight) : 0);
    const top = b.top + (content ? numeric(style.borderTopWidth) + numeric(style.paddingTop) : 0);
    const bottom = b.bottom - (content ? numeric(style.borderBottomWidth) + numeric(style.paddingBottom) : 0);
    const rtl = style.direction === 'rtl';
    return {
      inlineStart: round(rtl ? right - a.right : a.left - left),
      inlineEnd: round(rtl ? a.left - left : right - a.right),
      blockStart: round(a.top - top), blockEnd: round(bottom - a.bottom),
    };
  };
  const describe = element => ({
    tag: element.tagName.toLowerCase(),
    target: [...element.classList].find(name => name.startsWith('astryx-')) || null,
    variant: element.getAttribute('data-variant'), direction: element.getAttribute('data-direction'),
    gap: element.getAttribute('data-gap'), size: element.getAttribute('data-size'),
    inlineStyle: element.getAttribute('style'),
  });
  const parent = root.parentElement;
  const theme = root.closest('[data-astryx-theme]');
  const ancestors = [];
  for (let element = parent; element && element !== theme; element = element.parentElement) {
    ancestors.push(element);
  }
  const card = ancestors.find(element => element.classList.contains('astryx-card')
    || element.getAttribute('data-divider-reference-frame') === 'card') || null;
  const section = ancestors.find(element => element.classList.contains('astryx-section')
    || element.getAttribute('data-divider-reference-frame') === 'section') || null;
  const frames = {layout: parent, card, section, sectionOuter: section?.parentElement || null};
  const label = root.children.length === 3 ? root.children[1] : null;
  const rules = [...root.children].filter(element => element !== label);
  const labelledBy = root.getAttribute('aria-labelledby');
  return {
    ...identity,
    theme: {name: theme?.getAttribute('data-astryx-theme') || null,
      mode: theme?.getAttribute('data-theme') || null, colorScheme: getComputedStyle(root).colorScheme},
    attributes: {
      dataOrientation: root.getAttribute('data-orientation'), dataVariant: root.getAttribute('data-variant'),
      dataLabel: root.getAttribute('data-label'), dataFullBleed: root.getAttribute('data-full-bleed'),
      role: root.getAttribute('role'), ariaOrientation: root.getAttribute('aria-orientation'),
      ariaLabel: root.getAttribute('aria-label'), ariaLabelledBy: labelledBy,
      labelledByResolvesToLabel: Boolean(label && labelledBy && labelledBy.split(/\s+/).some(id => document.getElementById(id) === label)),
      labelText: label?.textContent || null,
    },
    root: box(root), tokens: tokens(root), ruleCount: rules.length,
    rules: rules.map(element => ({
      ...box(element), relativeToRootBorder: relative(element, root, 'border'),
      relativeToRootContent: relative(element, root, 'content'),
      role: element.getAttribute('role'), ariaHidden: element.getAttribute('aria-hidden'),
      tabIndex: element.getAttribute('tabindex'),
    })),
    label: label ? {
      text: label.textContent, ...box(label),
      relativeToRootBorder: relative(label, root, 'border'),
      relativeToRootContent: relative(label, root, 'content'),
    } : null,
    frames: Object.fromEntries(Object.entries(frames).map(([name, element]) => [name, element ? {
      ...describe(element), box: box(element), tokens: tokens(element),
      rootRelativeToBorder: relative(root, element, 'border'),
      rootRelativeToContent: relative(root, element, 'content'),
    } : null])),
    // Includes source presentation wrappers, only up to the local theme boundary.
    ancestorFrames: ancestors.map(element => ({
      ...describe(element), box: box(element),
      rootRelativeToBorder: relative(root, element, 'border'),
      rootRelativeToContent: relative(root, element, 'content'),
    })),
    layoutChildren: [...parent.children].map(element => ({
      ...describe(element), text: element.textContent, box: box(element),
      relativeToLayoutContent: relative(element, parent, 'content'),
    })),
    nestedStacks: card ? [...card.querySelectorAll('.astryx-stack, [data-divider-reference-stack]')].map(element => ({
      ...describe(element), box: box(element), relativeToCardContent: relative(element, card, 'content'),
    })) : [],
  };
}
