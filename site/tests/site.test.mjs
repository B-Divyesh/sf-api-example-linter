import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, statSync } from 'node:fs';

const pages = [
  ['site/index.html', 'API Example Linter — Lint API examples'],
  ['site/demo/index.html', 'Demo — API Example Linter'],
  ['site/privacy/index.html', 'Privacy — API Example Linter'],
  ['site/terms/index.html', 'Terms — API Example Linter'],
  ['site/404.html', 'Page not found — API Example Linter']
];

for (const [page, title] of pages) {
  test(`${page} has the accessibility and title baseline`, () => {
    const html = readFileSync(page, 'utf8');
    assert.match(html, /<html lang="en">/);
    assert.ok(html.includes('<title>' + title + '</title>'));
    assert.equal((html.match(/<main\b/g) ?? []).length, 1);
    assert.equal((html.match(/<h1\b/g) ?? []).length, 1);
    assert.match(html, /class="skip-link"/);
    assert.match(html, /href="\/privacy\/"[^>]*>Privacy/);
    assert.match(html, /href="\/terms\/"[^>]*>Terms/);
    assert.match(html, /Built by Param Factory · v0\.1\.0/);
  });
}

for (const page of pages.slice(0, 4).map(([file]) => file)) {
  test(`${page} has share and canonical metadata`, () => {
    const html = readFileSync(page, 'utf8');
    assert.match(html, /rel="canonical"/);
    assert.match(html, /property="og:title"/);
    assert.match(html, /property="og:description"/);
    assert.match(html, /contract-loom-social\.png/);
    assert.match(html, /name="twitter:card" content="summary_large_image"/);
    assert.match(html, /rel="apple-touch-icon"/);
  });
}

test('first screen states the job, audience, demo action, and next result', () => {
  const html = readFileSync('site/index.html', 'utf8');
  assert.match(html, /<h1 id="hero-title">Lint API examples against OpenAPI\.<\/h1>/);
  assert.match(html, /For API maintainers whose copied JSON or curl examples drift from their OpenAPI contract\./);
  assert.match(html, /href="\/\?demo=1#sample-result">Try it with sample data<\/a>/);
  assert.match(html, /Runs the included failing API example in a temporary folder\./);
  assert.doesNotMatch(html, /Catch the example|Five-minute|Contract-aware|Examples enter|formats between the cracks/i);
});

test('hero and social artwork meet their image contracts', () => {
  assert.ok(statSync('site/public/assets/contract-loom.webp').size <= 300 * 1024);
  const html = readFileSync('site/index.html', 'utf8');
  assert.match(html, /contract-loom\.webp/);
  assert.match(html, /alt="[^"]+"/);
  assert.match(html, /width="1536" height="1024"/);
  const social = statSync('site/public/assets/contract-loom-social.png');
  assert.ok(social.size > 0);
  assert.ok(statSync('site/public/apple-touch-icon.png').size > 0);
});

test('runtime uses no third-party analytics or font hosts', () => {
  for (const file of ['site/index.html', 'site/demo/index.html', 'site/src/main.ts', 'site/src/styles.css']) {
    const source = readFileSync(file, 'utf8');
    assert.doesNotMatch(source, /fonts\.googleapis|fonts\.gstatic|googletag|segment\.io/i);
  }
});

test('visible install command is the exact executable command copied by the control', () => {
  const html = readFileSync('site/index.html', 'utf8');
  const command = 'cargo install --git https://github.com/B-Divyesh/sf-api-example-linter.git';
  assert.match(html, new RegExp(`<code>${command}</code>`));
  assert.match(html, new RegExp(`data-copy="${command}"`));
  assert.doesNotMatch(html, /github\.com\/…/);
});

test('Azure deployment config provides real 404 handling and response policy', () => {
  const config = JSON.parse(readFileSync('site/public/staticwebapp.config.json', 'utf8'));
  assert.equal(config.navigationFallback, undefined);
  assert.deepEqual(config.responseOverrides['404'], { rewrite: '/404.html', statusCode: 404 });
  assert.equal(config.globalHeaders['X-Content-Type-Options'], 'nosniff');
  assert.equal(config.globalHeaders['Referrer-Policy'], 'no-referrer');
  assert.match(config.globalHeaders['Permissions-Policy'], /camera=\(\)/);
  assert.match(config.globalHeaders['Content-Security-Policy'], /frame-ancestors 'none'/);
  assert.deepEqual(config.routes, [{
    route: '/assets/*',
    headers: { 'Cache-Control': 'public, max-age=31536000, immutable' }
  }]);
});

test('sitemap and service worker include every public route', () => {
  const sitemap = readFileSync('site/public/sitemap.xml', 'utf8');
  const worker = readFileSync('site/public/sw.js', 'utf8');
  for (const route of ['/demo/', '/privacy/', '/terms/']) {
    assert.ok(sitemap.includes(route));
    assert.ok(worker.includes(route));
  }
});
