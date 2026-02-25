// Inject a Download button into the YouTube watch page using Shadow DOM

const BUTTON_HOST_ID = 'downloadtool-btn-host';

// Multiple fallback selectors for YouTube's action bar (A/B tested frequently)
const ANCHOR_SELECTORS = [
  '#above-the-fold #top-level-buttons-computed',
  '#actions-inner #top-level-buttons-computed',
  'ytd-watch-metadata #actions',
  '#menu-container #top-level-buttons-computed',
];

/** Find YouTube's action container with fallback selectors */
function findActionsContainer(): Element | null {
  for (const sel of ANCHOR_SELECTORS) {
    const el = document.querySelector(sel);
    if (el) return el;
  }
  return null;
}

/**
 * Inject the Download button into the YouTube page.
 * Uses Shadow DOM to isolate styles from YouTube's CSS.
 * Idempotent: safe to call multiple times.
 *
 * @param onClick - Callback when the button is clicked
 */
export function injectButton(onClick: () => void): void {
  // Idempotency guard: remove existing host if present
  document.getElementById(BUTTON_HOST_ID)?.remove();

  const container = findActionsContainer();
  if (!container) {
    console.warn('[DownloadTool] Could not find YouTube actions container');
    return;
  }

  // Shadow DOM host
  const host = document.createElement('div');
  host.id = BUTTON_HOST_ID;
  host.style.display = 'inline-flex';
  host.style.alignItems = 'center';
  host.style.marginLeft = '8px';

  const shadow = host.attachShadow({ mode: 'closed' });

  // Scoped styles inside shadow root — not affected by YouTube CSS
  const style = document.createElement('style');
  style.textContent = `
    button {
      display: inline-flex;
      align-items: center;
      gap: 6px;
      padding: 0 16px;
      height: 36px;
      border: none;
      border-radius: 18px;
      background: #ff0000;
      color: #fff;
      font-size: 14px;
      font-weight: 500;
      font-family: Roboto, Arial, sans-serif;
      cursor: pointer;
      white-space: nowrap;
      transition: background 0.15s;
    }
    button:hover { background: #cc0000; }
    button:active { background: #aa0000; }
    button.loading { background: #888; cursor: wait; }
  `;

  const btn = document.createElement('button');
  btn.textContent = '⬇ Download';
  btn.addEventListener('click', () => {
    btn.classList.add('loading');
    btn.textContent = '⏳ Loading...';
    onClick();
  });

  shadow.appendChild(style);
  shadow.appendChild(btn);
  container.appendChild(host);
}

/** Reset the Download button to its default state (after picker closes) */
export function resetButton(): void {
  const host = document.getElementById(BUTTON_HOST_ID);
  if (!host?.shadowRoot) return;
  const btn = host.shadowRoot.querySelector('button');
  if (!btn) return;
  btn.classList.remove('loading');
  btn.textContent = '⬇ Download';
}
