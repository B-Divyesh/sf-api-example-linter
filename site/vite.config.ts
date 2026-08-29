import { defineConfig } from 'vite';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';

const here = fileURLToPath(new URL('.', import.meta.url));
const outDir = resolve(here, '../dist/site');
// The terminal recording is produced by the CLI during each site build.  The
// temporary directory is intentionally normalized because the operating
// system assigns a different suffix on every demo run.
const demoTranscript = execFileSync('cargo', ['run', '--quiet', '--', 'demo'], {
  cwd: resolve(here, '..'),
  encoding: 'utf8'
}).replace(/\/tmp\/api-example-linter-demo-[^/\s]+/g, '/tmp/api-example-linter-demo-<temporary>');

export default defineConfig({
  root: resolve(here),
  publicDir: resolve(here, 'public'),
  define: {
    __DEMO_TRANSCRIPT__: JSON.stringify(demoTranscript)
  },
  build: {
    outDir,
    emptyOutDir: true,
    target: 'es2022',
    cssCodeSplit: false,
    rollupOptions: {
      input: {
        index: resolve(here, 'index.html'),
        demo: resolve(here, 'demo/index.html'),
        privacy: resolve(here, 'privacy/index.html'),
        terms: resolve(here, 'terms/index.html'),
        notFound: resolve(here, '404.html')
      }
    }
  },
  plugins: [{
    name: 'precache-built-assets',
    closeBundle() {
      const assets = readdirSync(resolve(outDir, 'assets'))
        .filter(name => name.endsWith('.js') || name.endsWith('.css'))
        .map(name => JSON.stringify(`/assets/${name}`));
      const path = resolve(outDir, 'sw.js');
      const source = readFileSync(path, 'utf8').replace('/* BUILD_ASSETS */', assets.join(', '));
      writeFileSync(path, source);
    }
  }]
});
