import test, { after } from 'node:test';
import assert from 'node:assert/strict';
import { mkdirSync } from 'node:fs';
import { chromium } from 'playwright';
import AxeBuilder from '@axe-core/playwright';
import { startSite } from './helpers.mjs';

const site = await startSite();
const browser = await chromium.launch({ headless: true });
mkdirSync('.factory/evidence', { recursive: true });
after(async () => {
  await browser.close();
  await site.close();
});

test('all routes have unique titles, one main, one h1, and no serious accessibility errors', async () => {
  const routes = [
    ['/', 'API Example Linter — Lint API examples'],
    ['/demo/', 'Demo — API Example Linter'],
    ['/privacy/', 'Privacy — API Example Linter'],
    ['/terms/', 'Terms — API Example Linter'],
    ['/404.html', 'Page not found — API Example Linter']
  ];
  const context = await browser.newContext();
  const page = await context.newPage();
  const errors = [];
  page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  page.on('pageerror', error => errors.push(error.message));
  for (const [route, title] of routes) {
    const response = await page.goto(site.origin + route);
    assert.equal(response.status(), 200);
    assert.equal(await page.title(), title);
    assert.equal(await page.locator('main').count(), 1);
    assert.equal(await page.locator('h1').count(), 1);
    assert.equal(await page.locator('html').getAttribute('lang'), 'en');
    const results = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa']).analyze();
    const serious = results.violations.filter(item => ['serious', 'critical'].includes(item.impact));
    assert.deepEqual(serious, [], JSON.stringify(serious, null, 2));
  }
  assert.deepEqual(errors, []);
  await context.close();
});

test('mobile first screen, query demo, controls, and keyboard focus work without overflow', async () => {
  const context = await browser.newContext({ viewport: { width: 390, height: 844 }, reducedMotion: 'reduce' });
  const page = await context.newPage();
  await page.goto(site.origin + '/');
  await page.keyboard.press('Tab');
  assert.equal(await page.locator(':focus').textContent(), 'Skip to content');
  await page.keyboard.press('Enter');
  assert.equal(await page.locator(':focus').getAttribute('id'), 'main');
  assert.equal(await page.evaluate(() => document.documentElement.scrollWidth), 390);
  await page.screenshot({ path: '.factory/evidence/home-mobile.png', fullPage: true });
  await page.getByRole('link', { name: 'Try it with sample data' }).click();
  await page.waitForURL(/\/demo\/\?demo=1/);
  assert.equal(await page.title(), 'Demo — API Example Linter');
  await page.getByText('Demo — sample data, nothing is saved').waitFor();
  await page.getByText('2 example(s) checked · 1 passed · 1 failed').waitFor();
  await page.getByRole('button', { name: 'Restart recording' }).click();
  await page.getByText('Ready to check two bundled examples.').waitFor();
  await page.getByRole('button', { name: 'Play recording' }).click();
  await page.getByText('2 example(s) checked · 1 passed · 1 failed').waitFor();
  assert.equal(await page.evaluate(() => document.documentElement.scrollWidth), 390);
  await page.screenshot({ path: '.factory/evidence/demo-mobile.png', fullPage: true });
  const axe = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa']).analyze();
  assert.deepEqual(axe.violations.filter(item => ['serious', 'critical'].includes(item.impact)), []);
  await context.close();
});

test('unknown routes return a designed 404 response', async () => {
  const context = await browser.newContext();
  const page = await context.newPage();
  const response = await page.goto(site.origin + '/no-such-page');
  assert.equal(response.status(), 404);
  await page.getByRole('heading', { level: 1, name: 'Page not found' }).waitFor();
  await page.getByRole('link', { name: 'Return home' }).waitFor();
  await page.screenshot({ path: '.factory/evidence/404-desktop.png', fullPage: true });
  await context.close();
});

test('route navigation and Back focus and announce the destination heading', async () => {
  const context = await browser.newContext();
  const page = await context.newPage();
  await page.goto(site.origin + '/');
  await page.getByRole('link', { name: 'Demo', exact: true }).click();
  await page.getByRole('heading', { level: 1, name: 'Run the bundled linter sample.' }).waitFor();
  await page.waitForFunction(() => document.activeElement?.tagName === 'H1');
  assert.match(await page.locator('.route-announcer').textContent(), /Run the bundled linter sample/);
  await page.goBack();
  await page.getByRole('heading', { level: 1, name: 'Lint API examples against OpenAPI.' }).waitFor();
  await page.waitForFunction(() => document.activeElement?.tagName === 'H1');
  assert.match(await page.locator('.route-announcer').textContent(), /Lint API examples against OpenAPI/);
  await context.close();
});

test('mobile links and buttons meet the 44px touch target baseline', async () => {
  const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
  const page = await context.newPage();
  for (const route of ['/', '/demo/', '/privacy/', '/terms/', '/404.html']) {
    await page.goto(site.origin + route);
    const targets = await page.locator('a:not(.skip-link), button').evaluateAll(elements => elements.map(element => {
      const style = getComputedStyle(element);
      const rect = element.getBoundingClientRect();
      return { text: element.textContent?.trim(), width: rect.width, height: rect.height, visible: style.display !== 'none' && style.visibility !== 'hidden' && rect.width > 0 && rect.height > 0 };
    }).filter(target => target.visible));
    for (const target of targets) assert.ok(target.width >= 44 && target.height >= 44, `${route}: ${target.text} is ${target.width}×${target.height}`);
  }
  await context.close();
});

test('every internal link resolves and legal links appear in every footer', async () => {
  const context = await browser.newContext();
  const page = await context.newPage();
  const routes = ['/', '/demo/', '/privacy/', '/terms/', '/404.html'];
  for (const route of routes) {
    await page.goto(site.origin + route);
    const footerLinks = await page.locator('footer a').evaluateAll(links => links.map(link => link.getAttribute('href')));
    assert.ok(footerLinks.includes('/privacy/'));
    assert.ok(footerLinks.includes('/terms/'));
    const internal = await page.locator('a[href^="/"]').evaluateAll(links => [...new Set(links.map(link => link.getAttribute('href')))]);
    for (const href of internal) {
      const url = new URL(href, site.origin);
      const response = await context.request.get(url.origin + url.pathname);
      assert.equal(response.status(), 200, href);
    }
  }
  await context.close();
});
