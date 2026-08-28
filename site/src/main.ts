export {};

const main = document.querySelector<HTMLElement>('#main');
main?.setAttribute('tabindex', '-1');
document.querySelector<HTMLAnchorElement>('.skip-link')?.addEventListener('click', () => {
  window.setTimeout(() => main?.focus(), 0);
});

const copyButtons = document.querySelectorAll<HTMLButtonElement>('[data-copy]');

for (const button of copyButtons) {
  const originalLabel = button.querySelector('span')?.textContent ?? 'Copy';
  button.addEventListener('click', async () => {
    const label = button.querySelector('span');
    try {
      await navigator.clipboard.writeText(button.dataset.copy ?? '');
      if (label) label.textContent = 'Copied';
      button.classList.add('copied');
      window.setTimeout(() => {
        if (label) label.textContent = originalLabel;
        button.classList.remove('copied');
      }, 1800);
    } catch {
      if (label) label.textContent = 'Copy failed';
    }
  });
}

const output = document.querySelector<HTMLElement>('#terminal-output code');
const empty = document.querySelector<HTMLElement>('#demo-empty');
const status = document.querySelector<HTMLElement>('#demo-status');
const play = document.querySelector<HTMLButtonElement>('#play-demo');
const restart = document.querySelector<HTMLButtonElement>('#restart-demo');
const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
const params = new URLSearchParams(location.search);
const isDemo = params.get('demo') === '1' || location.pathname.replace(/\/+$/, '') === '/demo';
const frames = [
  '<span class="prompt">$</span> api-example-linter demo',
  'Demo — bundled sample data in a temporary folder',
  'Temporary folder: /tmp/api-example-linter-demo-7f3c',
  '',
  '<span class="scan">CHECK</span>  docs/create-pet.md:6  JSON request',
  '<span class="pass-text">PASS</span>   name and status match createPet',
  '<span class="scan">CHECK</span>  docs/create-pet.md:12  curl request body',
  '<span class="fail-text">FAIL</span>   docs/create-pet.md:12:1  SCHEMA_MISMATCH',
  '       property <span class="highlight">\'retired_field\'</span> is not allowed  <span class="pointer">($/retired_field)</span>',
  '',
  '<span class="summary">FAIL  2 example(s) checked · 1 passed · 1 failed</span>',
  'Demo complete. The temporary folder is removed now.'
];
let frame = 0;
let timer: number | undefined;
let playing = false;

function renderFrame() {
  if (!output || !empty || !status || !play) return;
  empty.hidden = true;
  if (output.parentElement) output.parentElement.hidden = false;
  output.innerHTML = frames.slice(0, frame).join('\n');
  status.textContent = frame >= frames.length ? 'Sample result complete. One stale field was found.' : `Step ${frame} of ${frames.length}.`;
  if (isDemo) sessionStorage.setItem('demo:api-example-linter:frame', String(frame));
  if (frame >= frames.length) {
    playing = false;
    play.textContent = 'Play again';
    window.clearInterval(timer);
  }
}

function showSeededResult(message = 'Sample result loaded. One stale field was found.') {
  frame = frames.length;
  renderFrame();
  if (status) status.textContent = message;
}

function start() {
  if (!play) return;
  if (frame >= frames.length) frame = 0;
  if (reduceMotion) {
    showSeededResult('Sample result shown without animation.');
    return;
  }
  playing = !playing;
  play.textContent = playing ? 'Pause recording' : 'Resume recording';
  window.clearInterval(timer);
  if (playing) {
    frame += 1;
    renderFrame();
    timer = window.setInterval(() => {
      frame += 1;
      renderFrame();
    }, 620);
  }
}

play?.addEventListener('click', start);
restart?.addEventListener('click', () => {
  window.clearInterval(timer);
  playing = false;
  frame = 0;
  if (output) output.innerHTML = '';
  if (output?.parentElement) output.parentElement.hidden = true;
  if (empty) empty.hidden = false;
  if (play) play.textContent = 'Play recording';
  if (status) status.textContent = 'Recording reset. Press Play recording to run it.';
  if (isDemo) sessionStorage.setItem('demo:api-example-linter:frame', '0');
});

function clearDemoStorage() {
  for (let index = sessionStorage.length - 1; index >= 0; index -= 1) {
    const key = sessionStorage.key(index);
    if (key?.startsWith('demo:api-example-linter:')) sessionStorage.removeItem(key);
  }
}

if (isDemo) {
  document.querySelector<HTMLElement>('[data-demo-banner]')?.removeAttribute('hidden');
  if (params.get('demo') === '1') document.title = 'Demo — API Example Linter';
  showSeededResult();
  document.querySelector<HTMLElement>('.route-announcer')!.textContent = 'Demo loaded with bundled sample data.';
}

document.querySelector<HTMLButtonElement>('[data-reset-demo]')?.addEventListener('click', () => {
  clearDemoStorage();
  sessionStorage.setItem('demo:api-example-linter:frame', String(frames.length));
  showSeededResult('Demo reset to the bundled sample result.');
});

document.querySelector<HTMLElement>('[data-leave-demo]')?.addEventListener('click', () => clearDemoStorage());

const offline = document.querySelector<HTMLElement>('#offline');
function updateConnection() {
  if (offline) offline.hidden = navigator.onLine;
}
window.addEventListener('online', updateConnection);
window.addEventListener('offline', updateConnection);
updateConnection();

if ('serviceWorker' in navigator && location.protocol !== 'file:') {
  window.addEventListener('load', () => navigator.serviceWorker.register('/sw.js').catch(() => undefined));
}
