const copyButtons = document.querySelectorAll<HTMLButtonElement>('[data-copy]');

for (const button of copyButtons) {
  button.addEventListener('click', async () => {
    const label = button.querySelector('span');
    try {
      await navigator.clipboard.writeText(button.dataset.copy ?? '');
      if (label) label.textContent = 'Copied';
      button.classList.add('copied');
      window.setTimeout(() => {
        if (label) label.textContent = 'Copy';
        button.classList.remove('copied');
      }, 1800);
    } catch {
      if (label) label.textContent = 'Select text to copy';
    }
  });
}

const output = document.querySelector<HTMLElement>('#terminal-output code');
const empty = document.querySelector<HTMLElement>('#demo-empty');
const status = document.querySelector<HTMLElement>('#demo-status');
const play = document.querySelector<HTMLButtonElement>('#play-demo');
const restart = document.querySelector<HTMLButtonElement>('#restart-demo');
const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
const frames = [
  '<span class="prompt">$</span> api-example-linter check docs --spec openapi.yaml --operation createPet',
  '<span class="scan">SCAN</span>  docs/create-pet.md:18  request example',
  '<span class="pass-text">PASS</span>  docs/create-pet.md:18  matches createPet request',
  '<span class="scan">SCAN</span>  docs/create-pet.md:42  curl request body',
  '<span class="fail-text">FAIL</span>  docs/create-pet.md:42:1  SCHEMA_MISMATCH',
  '      property <span class="highlight">\'retired_field\'</span> is not allowed  <span class="pointer">($/retired_field)</span>',
  '',
  '<span class="summary">2 examples checked · 1 passed · 1 failed</span>',
  '<span class="exit">exit 1</span>  CI stopped before the stale example shipped'
];
let frame = 0;
let timer: number | undefined;
let playing = false;

function renderFrame() {
  if (!output || !empty || !status || !play) return;
  empty.hidden = true;
  output.parentElement!.hidden = false;
  output.innerHTML = frames.slice(0, frame).join('\n');
  status.textContent = frame >= frames.length ? 'Recording complete. One stale field was caught.' : `Frame ${frame} of ${frames.length}.`;
  if (frame >= frames.length) {
    playing = false;
    play.textContent = 'Play again';
    window.clearInterval(timer);
  }
}

function start() {
  if (!play) return;
  if (frame >= frames.length) frame = 0;
  if (reduceMotion) { frame = frames.length; renderFrame(); return; }
  playing = !playing;
  play.textContent = playing ? 'Pause recording' : 'Resume recording';
  window.clearInterval(timer);
  if (playing) {
    frame += 1;
    renderFrame();
    timer = window.setInterval(() => { frame += 1; renderFrame(); }, 620);
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
  if (status) status.textContent = 'Recording stopped at the first frame.';
});

const offline = document.querySelector<HTMLElement>('#offline');
function updateConnection() { if (offline) offline.hidden = navigator.onLine; }
window.addEventListener('online', updateConnection);
window.addEventListener('offline', updateConnection);
updateConnection();

if ('serviceWorker' in navigator && location.protocol !== 'file:') {
  window.addEventListener('load', () => navigator.serviceWorker.register('/sw.js').catch(() => undefined));
}
