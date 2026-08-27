import { defineConfig } from 'vite';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { readFileSync, readdirSync, writeFileSync } from 'node:fs';

const here = fileURLToPath(new URL('.', import.meta.url));
const outDir = resolve(here, '../dist/site');

export default defineConfig({
  root: resolve(here),
  publicDir: resolve(here, 'public'),
  build: {
    outDir,
    emptyOutDir: true,
    target: 'es2022',
    cssCodeSplit: false,
    rollupOptions: {
      input: {
        index: resolve(here, 'index.html'),
        privacy: resolve(here, 'privacy/index.html'),
        terms: resolve(here, 'terms/index.html')
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
