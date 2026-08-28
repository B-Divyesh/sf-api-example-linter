import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, statSync } from 'node:fs';

const pages = ['site/index.html', 'site/privacy/index.html', 'site/terms/index.html'];

for (const page of pages) {
  test(`${page} has the accessibility baseline`, () => {
    const html = readFileSync(page, 'utf8');
    assert.match(html, /<html lang="en">/);
    assert.match(html, /<title>[^<]+<\/title>/);
    assert.equal((html.match(/<main\b/g) ?? []).length, 1);
    assert.equal((html.match(/<h1\b/g) ?? []).length, 1);
    assert.match(html, /class="skip-link"/);
  });
}

test('hero is WebP and under 300 KB', () => {
  assert.ok(statSync('site/public/assets/contract-loom.webp').size <= 300 * 1024);
  const html = readFileSync('site/index.html', 'utf8');
  assert.match(html, /contract-loom\.webp/);
  assert.match(html, /alt="[^\"]+"/);
  assert.match(html, /width="1536" height="1024"/);
});

test('runtime uses no third-party analytics or font hosts', () => {
  for (const file of ['site/index.html', 'site/src/main.ts', 'site/src/styles.css']) {
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

test('Azure deployment config enforces the documented response policy and asset caching', () => {
  const config = JSON.parse(readFileSync('site/public/staticwebapp.config.json', 'utf8'));
  assert.equal(config.globalHeaders['X-Content-Type-Options'], 'nosniff');
  assert.equal(config.globalHeaders['Referrer-Policy'], 'no-referrer');
  assert.match(config.globalHeaders['Permissions-Policy'], /camera=\(\)/);
  assert.match(config.globalHeaders['Content-Security-Policy'], /default-src 'self'/);
  assert.deepEqual(config.routes, [{
    route: '/assets/*',
    headers: { 'Cache-Control': 'public, max-age=31536000, immutable' }
  }]);
});
