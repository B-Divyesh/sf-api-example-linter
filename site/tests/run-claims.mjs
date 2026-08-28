import { spawnSync } from 'node:child_process';

const result = spawnSync(process.execPath, [
  '--test',
  '--test-concurrency=1',
  ...process.argv.slice(2),
  'site/tests/claims.test.mjs'
], { stdio: 'inherit' });

process.exit(result.status ?? 1);
