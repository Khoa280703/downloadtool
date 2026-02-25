// Content script — injected into YouTube watch pages
// Injects a Download button via Shadow DOM; communicates with background for API calls

const BUTTON_HOST_ID = 'downloadtool-ext-host';

const ANCHOR_SELECTORS = [
  '#above-the-fold #top-level-buttons-computed',
  '#actions-inner #top-level-buttons-computed',
  'ytd-watch-metadata #actions',
  '#menu-container #top-level-buttons-computed',
];

function findActionsContainer(): Element | null {
  for (const sel of ANCHOR_SELECTORS) {
    const el = document.querySelector(sel);
    if (el) return el;
  }
  return null;
}

function injectButton(): void {
  document.getElementById(BUTTON_HOST_ID)?.remove();
  if (!location.href.includes('youtube.com/watch')) return;

  const container = findActionsContainer();
  if (!container) return;

  const host = document.createElement('div');
  host.id = BUTTON_HOST_ID;
  host.style.cssText = 'display:inline-flex;align-items:center;margin-left:8px';

  const shadow = host.attachShadow({ mode: 'open' });

  const style = document.createElement('style');
  style.textContent = `
    button {
      display:inline-flex;align-items:center;gap:6px;
      padding:0 16px;height:36px;border:none;border-radius:18px;
      background:#ff0000;color:#fff;font-size:14px;font-weight:500;
      font-family:Roboto,Arial,sans-serif;cursor:pointer;white-space:nowrap;
      transition:background .15s;
    }
    button:hover{background:#cc0000}
    button.loading{background:#888;cursor:wait}
  `;

  const btn = document.createElement('button');
  btn.textContent = '⬇ Download';

  btn.addEventListener('click', async () => {
    btn.classList.add('loading');
    btn.textContent = '⏳ Loading...';

    // Send URL to background for extraction (avoids CORS — background has full network access)
    chrome.runtime.sendMessage(
      { type: 'EXTRACT', url: location.href },
      (response: { error?: string; streams?: unknown[] } | null) => {
        btn.classList.remove('loading');
        btn.textContent = '⬇ Download';
        if (chrome.runtime.lastError || response?.error) {
          alert(`[DownloadTool] ${response?.error ?? chrome.runtime.lastError?.message}`);
        }
        // Popup handles stream selection; content script just triggers extraction
      },
    );
  });

  shadow.appendChild(style);
  shadow.appendChild(btn);
  container.appendChild(host);
}

// Initial inject
injectButton();

// SPA navigation: YouTube changes document.title on navigation
let lastTitle = document.title;
let debounceTimer: ReturnType<typeof setTimeout>;

new MutationObserver(() => {
  if (document.title !== lastTitle) {
    lastTitle = document.title;
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(injectButton, 500);
  }
}).observe(document.querySelector('title') ?? document.head, {
  subtree: true,
  characterData: true,
  childList: true,
});
