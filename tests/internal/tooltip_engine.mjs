// Optional external engine harness; no project/runtime dependency is added.
import {openTextPage} from './text_cdp.mjs';
export async function openTooltipPage() {
  if (!process.env.TOOLTIP_ENGINE) {
    const page = await openTextPage();
    console.log('Engine chromium:', (await page.send('Browser.getVersion')).product);
    return page;
  }
  if (!process.env.PLAYWRIGHT_MODULE) throw Error('Set PLAYWRIGHT_MODULE to a temporary playwright index.mjs');
  const {firefox, webkit} = await import(process.env.PLAYWRIGHT_MODULE);
  const engine = {firefox, webkit}[process.env.TOOLTIP_ENGINE];
  if (!engine) throw Error('TOOLTIP_ENGINE must be firefox or webkit');
  const browser = await engine.launch({headless: true});
  const context = await browser.newContext({viewport: {width: 1000, height: 700}});
  const page = await context.newPage(), errors = [];
  page.on('pageerror', error => errors.push(error.message));
  const evaluate = expression => page.evaluate(expression);
  const waitFor = async expression => {
    const deadline = Date.now() + 10000;
    while (Date.now() < deadline) {
      if (await evaluate(expression)) return;
      await new Promise(resolve => setTimeout(resolve, 25));
    }
    throw Error(`Timed out: ${expression}`);
  };
  const send = async (method, params = {}) => {
    switch (method) {
      case 'Network.enable': case 'Runtime.enable': case 'Page.enable': return;
      case 'Network.setBlockedURLs':
        await context.route('https://**', route => route.abort()); return;
      case 'Page.navigate': await page.goto(params.url); return;
      case 'Input.dispatchKeyEvent': await page.keyboard.down(params.key); await page.keyboard.up(params.key); return;
      case 'Emulation.setDeviceMetricsOverride': await page.setViewportSize({width: params.width, height: params.height}); return;
      case 'Emulation.setEmulatedMedia': {
        const reducedMotion = params.features.find(f => f.name === 'prefers-reduced-motion')?.value || 'no-preference';
        await page.emulateMedia({reducedMotion}); return;
      }
      default: throw Error(`Unsupported engine command: ${method}`);
    }
  };
  console.log(`Engine ${process.env.TOOLTIP_ENGINE}: ${browser.version()}; ${await evaluate('navigator.userAgent')}`);
  console.log('Native support', await evaluate('({anchor:CSS.supports("anchor-name: --probe"),area:CSS.supports("position-area: self-block-start"),fallback:CSS.supports("position-try-fallbacks: flip-block"),discrete:CSS.supports("transition: display 1s allow-discrete")})'));
  return {send, evaluate, waitFor, errors, close: () => browser.close()};
}
