import test, { after } from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { spawn, spawnSync } from 'node:child_process';
import { createServer } from 'node:http';
import { chromium } from 'playwright';
import { startSite } from './helpers.mjs';

const binary = resolve('target/debug/api-example-linter');
const site = await startSite();
const browser = await chromium.launch({ headless: true });
after(async () => {
  await browser.close();
  await site.close();
});

function run(args, options = {}) {
  return spawnSync(binary, args, { encoding: 'utf8', ...options });
}

function runAsync(args, options = {}) {
  return new Promise(resolveRun => {
    const child = spawn(binary, args, options);
    let stdout = '';
    let stderr = '';
    child.stdout.on('data', chunk => { stdout += chunk; });
    child.stderr.on('data', chunk => { stderr += chunk; });
    child.on('close', status => resolveRun({ status, stdout, stderr }));
  });
}

test('@claim:demo-temp-isolation bundled demo uses and removes a temporary workspace', () => {
  const cwd = mkdtempSync(join(tmpdir(), 'api-linter-real-'));
  const sentinel = join(cwd, 'real.txt');
  writeFileSync(sentinel, 'unchanged');
  writeFileSync(join(cwd, '.api-example-linter.json'), '{"inputs":["missing-user-path"]}');
  const result = run(['demo'], { cwd });
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /2 example\(s\) checked · 1 passed · 1 failed/);
  assert.match(result.stdout, /SCHEMA_MISMATCH/);
  const workspace = result.stdout.match(/^Temporary folder: (.+)$/m)?.[1];
  assert.ok(workspace?.startsWith(tmpdir()));
  assert.equal(existsSync(workspace), false);
  assert.equal(readFileSync(sentinel, 'utf8'), 'unchanged');
});

test('@claim:shell-non-execution curl examples are parsed without running shell text or making network requests', async () => {
  const cwd = mkdtempSync(join(tmpdir(), 'api-linter-shell-'));
  const marker = join(cwd, 'must-not-exist');
  let requests = 0;
  const probe = createServer((_request, response) => {
    requests += 1;
    response.writeHead(302, { location: '/elsewhere' });
    response.end();
  });
  await new Promise(done => probe.listen(0, '127.0.0.1', done));
  const address = probe.address();
  const markdown = join(cwd, 'curl.md');
  const fence = String.fromCharCode(96).repeat(3);
  writeFileSync(markdown, fence + 'curl operation=createPet direction=request\n' +
    'curl --data=\'{"name":"Ada","tag":"rescue"}\' http://127.0.0.1:' + address.port + '/redirect; touch ' + marker + '\n' +
    fence + '\n');
  const result = run(['check', markdown, '--spec', resolve('fixtures/openapi.yaml'), '--operation', 'createPet']);
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /1 example\(s\) checked · 1 passed/);
  assert.equal(existsSync(marker), false);
  assert.equal(requests, 0);
  await new Promise(done => probe.close(done));
});

test('@claim:supported-inputs JSON, curl, and embedded OpenAPI 3.0 and 3.1 examples are checked', () => {
  const markdown = run(['check', 'fixtures/valid.md', '--spec', 'fixtures/openapi.yaml', '--operation', 'createPet']);
  assert.equal(markdown.status, 0, markdown.stderr);
  assert.match(markdown.stdout, /2 example\(s\) checked · 2 passed/);
  const openapi31 = run(['check', 'fixtures/openapi.yaml', '--operation', 'createPet', '--direction', 'request']);
  assert.equal(openapi31.status, 0, openapi31.stderr);
  assert.match(openapi31.stdout, /1 example\(s\) checked · 1 passed/);
  const cwd = mkdtempSync(join(tmpdir(), 'api-linter-openapi30-'));
  const spec30 = join(cwd, 'openapi.yaml');
  writeFileSync(spec30, readFileSync('fixtures/openapi.yaml', 'utf8').replace('3.1.0', '3.0.3'));
  const openapi30 = run(['check', spec30, '--operation', 'createPet', '--direction', 'request']);
  assert.equal(openapi30.status, 0, openapi30.stderr);
  assert.match(openapi30.stdout, /1 example\(s\) checked · 1 passed/);
  const fence = String.fromCharCode(96).repeat(3);
  const forms = join(cwd, 'curl-forms.md');
  const commands = [
    "curl --data '{\"name\":\"Ada\"}' https://example.invalid/pets",
    "curl --data-raw='{\"name\":\"Ada\"}' https://example.invalid/pets",
    "curl --data-binary='{\"name\":\"Ada\"}' https://example.invalid/pets",
    "curl -d'{\"name\":\"Ada\"}' https://example.invalid/pets",
    "curl -d='{\"name\":\"Ada\"}' https://example.invalid/pets"
  ];
  writeFileSync(forms, commands.map(command => `${fence}curl operation=createPet direction=request\n${command}\n${fence}`).join('\n'));
  const allForms = run(['check', forms, '--spec', 'fixtures/openapi.yaml', '--operation', 'createPet']);
  assert.equal(allForms.status, 0, allForms.stderr);
  assert.match(allForms.stdout, /5 example\(s\) checked · 5 passed/);
});

test('@claim:schema-mapping named schemas and operation request mappings both validate', () => {
  const schema = run(['check', 'fixtures/valid.md', '--spec', 'fixtures/openapi.yaml', '--schema', 'Pet']);
  assert.equal(schema.status, 0, schema.stderr);
  const operation = run(['check', 'fixtures/valid.md', '--spec', 'fixtures/openapi.yaml', '--operation', 'createPet', '--direction', 'request']);
  assert.equal(operation.status, 0, operation.stderr);
});

test('@claim:diagnostic-output text, JSON, and GitHub output identify the failed example', () => {
  for (const format of ['text', 'json', 'github']) {
    const result = run(['check', 'fixtures/invalid.md', '--spec', 'fixtures/openapi.yaml', '--schema', 'Pet', '--format', format]);
    assert.equal(result.status, 1);
    assert.match(result.stdout, /fixtures\/invalid\.md/);
    assert.match(result.stdout, /SCHEMA_MISMATCH/);
  }
  const github = run(['check', 'fixtures/invalid.md', '--spec', 'fixtures/openapi.yaml', '--schema', 'Pet', '--format', 'github']);
  assert.match(github.stdout, /::error file=fixtures\/invalid\.md,line=4,col=1,title=SCHEMA_MISMATCH::/);
  assert.equal(run(['check', 'fixtures/valid.md', '--spec', 'fixtures/openapi.yaml', '--schema', 'Pet']).status, 0);
  assert.equal(run(['check', 'missing.md', '--format', 'json']).status, 2);
});

test('@claim:mapping-metadata fence metadata supports schema, operation, request, response, and global defaults', () => {
  const cwd = mkdtempSync(join(tmpdir(), 'api-linter-metadata-'));
  const docs = join(cwd, 'examples.md');
  const fence = String.fromCharCode(96).repeat(3);
  writeFileSync(docs, [
    `${fence}json schema=Pet\n{"name":"Ada"}\n${fence}`,
    `${fence}json operation=createPet direction=request\n{"name":"Ada"}\n${fence}`,
    `${fence}json operation=createPet direction=response\n{"name":"Ada"}\n${fence}`,
    `${fence}json\n{"name":"Ada"}\n${fence}`
  ].join('\n'));
  const result = run(['check', docs, '--spec', 'fixtures/openapi.yaml', '--operation', 'createPet', '--direction', 'request']);
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /4 example\(s\) checked · 4 passed/);
});

test('@claim:config-precedence CLI flags override conflicting configuration values', () => {
  const cwd = mkdtempSync(join(tmpdir(), 'api-linter-config-precedence-'));
  const config = join(cwd, 'config.json');
  writeFileSync(config, JSON.stringify({
    spec: resolve('fixtures/openapi.yaml'), inputs: [resolve('fixtures/invalid.md')], schema: 'Missing', format: 'json'
  }));
  const result = run(['check', 'fixtures/valid.md', '--config', config, '--spec', 'fixtures/openapi.yaml', '--schema', 'Pet', '--format', 'text']);
  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /2 example\(s\) checked · 2 passed/);
  assert.doesNotMatch(result.stdout, /"summary"/);
});

test('@claim:config-init starter configuration is created without overwriting an existing file', () => {
  const cwd = mkdtempSync(join(tmpdir(), 'api-linter-init-'));
  const config = join(cwd, 'config.json');
  const first = run(['init', config]);
  assert.equal(first.status, 0, first.stderr);
  const original = readFileSync(config, 'utf8');
  const second = run(['init', config]);
  assert.equal(second.status, 2);
  assert.equal(readFileSync(config, 'utf8'), original);
});

test('@claim:core-schema-checks required, unknown, scalar type, and local reference rules are enforced', () => {
  const cwd = mkdtempSync(join(tmpdir(), 'api-linter-type-'));
  const fence = String.fromCharCode(96).repeat(3);
  const missingDocs = join(cwd, 'missing.md');
  writeFileSync(missingDocs, fence + 'json\n{"retired_field":true}\n' + fence + '\n');
  const missing = run(['check', missingDocs, '--spec', resolve('fixtures/openapi.yaml'), '--schema', 'Pet', '--format', 'json']);
  assert.equal(missing.status, 1);
  assert.match(missing.stdout, /missing required property 'name'/);
  assert.match(missing.stdout, /property 'retired_field' is not allowed/);
  const docs = join(cwd, 'type.md');
  writeFileSync(docs, fence + 'json\n{"name":42}\n' + fence + '\n');
  const type = run(['check', docs, '--spec', resolve('fixtures/openapi.yaml'), '--schema', 'Pet']);
  assert.equal(type.status, 1);
  assert.match(type.stdout, /expected string, found number/);
});

test('@claim:local-by-default default checks work offline and never fetch remote references', async () => {
  let requests = 0;
  const probe = createServer((_request, response) => {
    requests += 1;
    response.end('{}');
  });
  await new Promise(done => probe.listen(0, '127.0.0.1', done));
  const address = probe.address();
  const cwd = mkdtempSync(join(tmpdir(), 'api-linter-remote-ref-'));
  const spec = join(cwd, 'openapi.yaml');
  const docs = join(cwd, 'example.md');
  writeFileSync(spec, 'openapi: 3.1.0\ninfo: {title: Probe, version: 1.0.0}\npaths: {}\ncomponents:\n  schemas:\n    Pet:\n      $ref: \'http://127.0.0.1:' + address.port + '/schema.json\'\n');
  writeFileSync(docs, String.fromCharCode(96).repeat(3) + 'json\n{"name":"Ada"}\n' + String.fromCharCode(96).repeat(3) + '\n');
  const result = run(['check', docs, '--spec', spec, '--schema', 'Pet']);
  assert.equal(result.status, 1);
  assert.match(result.stdout, /remote \$ref .* is not supported/);
  assert.equal(requests, 0);
  const local = run(['check', 'fixtures/valid.md', '--spec', 'fixtures/openapi.yaml', '--operation', 'createPet']);
  assert.equal(local.status, 0, local.stderr);
  await new Promise(done => probe.close(done));
});

test('@claim:mock-host-gating mock requests require a loopback or explicitly allowed host', async () => {
  const denied = run(['check', 'fixtures/valid.md', '--spec', 'fixtures/openapi.yaml', '--operation', 'createPet', '--direction', 'request', '--mock-base-url', 'http://example.invalid']);
  assert.equal(denied.status, 1);
  assert.match(denied.stdout, /not allowed/);
  let requests = 0;
  const mock = createServer((_request, response) => {
    requests += 1;
    response.writeHead(204);
    response.end();
  });
  await new Promise(done => mock.listen(0, done));
  const address = mock.address();
  const allowed = await runAsync(['check', 'fixtures/valid.md', '--spec', 'fixtures/openapi.yaml', '--operation', 'createPet', '--direction', 'request', '--mock-base-url', 'http://127.0.0.1:' + address.port]);
  assert.equal(allowed.status, 0, allowed.stderr || allowed.stdout);
  assert.equal(requests, 2);
  const explicitlyAllowed = await runAsync(['check', 'fixtures/valid.md', '--spec', 'fixtures/openapi.yaml', '--operation', 'createPet', '--direction', 'request', '--mock-base-url', 'http://localhost.:' + address.port, '--allow-host', 'localhost.']);
  assert.equal(explicitlyAllowed.status, 0, explicitlyAllowed.stderr || explicitlyAllowed.stdout);
  assert.equal(requests, 4);
  await new Promise(done => mock.close(done));
});

test('@claim:demo-transcript-parity web recording is the normalized CLI demo output', async () => {
  const cli = run(['demo']);
  assert.equal(cli.status, 0, cli.stderr);
  const normalize = value => value.replace(/\/tmp\/api-example-linter-demo-[^/\s]+/g, '/tmp/api-example-linter-demo-<temporary>').trim();
  const context = await browser.newContext();
  const page = await context.newPage();
  await page.goto(site.origin + '/demo/?demo=1');
  const transcript = await page.locator('#terminal-output code').textContent();
  assert.equal(normalize(transcript ?? ''), normalize(cli.stdout));
  await context.close();
});

test('@claim:browser-privacy demo flow makes only same-origin requests and sets no cookies or file controls', async () => {
  const context = await browser.newContext();
  const page = await context.newPage();
  const requests = [];
  page.on('request', request => requests.push(request.url()));
  await page.goto(site.origin + '/demo/?demo=1');
  await page.getByRole('button', { name: 'Reset demo' }).click();
  assert.ok(requests.length > 0);
  assert.ok(requests.every(url => new URL(url).origin === site.origin), requests.join('\n'));
  assert.deepEqual(await context.cookies(), []);
  assert.equal(await page.evaluate(() => localStorage.length), 0);
  assert.equal(await page.locator('form').count(), 0);
  assert.equal(await page.locator('input[type="file"]').count(), 0);
  await context.close();
});

test('@claim:demo-web-isolation reset removes only demo session data', async () => {
  const context = await browser.newContext();
  const page = await context.newPage();
  await page.goto(site.origin + '/demo/?demo=1');
  await page.evaluate(() => {
    sessionStorage.setItem('real:user-setting', 'keep');
    sessionStorage.setItem('demo:api-example-linter:old', 'remove');
  });
  await page.getByRole('button', { name: 'Reset demo' }).click();
  const storage = await page.evaluate(() => Object.fromEntries(Object.entries(sessionStorage)));
  assert.equal(storage['real:user-setting'], 'keep');
  assert.equal(storage['demo:api-example-linter:old'], undefined);
  assert.equal(storage['demo:api-example-linter:frame'], '7');
  await context.close();
});

test('@claim:offline-site visited demo reloads without a network connection', async () => {
  const context = await browser.newContext({ serviceWorkers: 'allow' });
  const page = await context.newPage();
  await page.goto(site.origin + '/demo/');
  await page.evaluate(() => navigator.serviceWorker.ready);
  await page.reload();
  await page.waitForFunction(() => Boolean(navigator.serviceWorker.controller));
  await context.setOffline(true);
  await page.reload({ waitUntil: 'domcontentloaded' });
  await page.getByRole('heading', { level: 1, name: 'Run the bundled linter sample.' }).waitFor();
  await page.goto(site.origin + '/', { waitUntil: 'domcontentloaded' });
  await page.getByRole('heading', { level: 1, name: 'Lint API examples against OpenAPI.' }).waitFor();
  await context.close();
});

test('@claim:mit-license the repository ships the MIT license', () => {
  const license = readFileSync('LICENSE', 'utf8');
  assert.match(license, /Permission is hereby granted/);
  assert.match(readFileSync('Cargo.toml', 'utf8'), /license = "MIT"/);
});
