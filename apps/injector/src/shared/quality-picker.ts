// Quality picker modal rendered inside a Shadow DOM root

import type { MuxedPair } from './stream-utils.js';

const MODAL_HOST_ID = 'downloadtool-modal-host';

/** Show a quality picker modal for available muxed pairs */
export function showQualityPicker(
  pairs: MuxedPair[],
  onSelect: (pair: MuxedPair) => void,
  onClose: () => void,
): void {
  // Remove existing modal
  document.getElementById(MODAL_HOST_ID)?.remove();

  const host = document.createElement('div');
  host.id = MODAL_HOST_ID;
  document.body.appendChild(host);

  const shadow = host.attachShadow({ mode: 'closed' });

  const style = document.createElement('style');
  style.textContent = `
    .overlay {
      position: fixed; inset: 0;
      background: rgba(0,0,0,0.6);
      z-index: 999999;
      display: flex; align-items: center; justify-content: center;
    }
    .modal {
      background: #212121;
      border-radius: 12px;
      padding: 24px;
      min-width: 300px;
      max-width: 420px;
      width: 90vw;
      color: #fff;
      font-family: Roboto, Arial, sans-serif;
      box-shadow: 0 8px 32px rgba(0,0,0,0.5);
    }
    h2 {
      margin: 0 0 16px;
      font-size: 16px;
      font-weight: 500;
      color: #fff;
    }
    .close-btn {
      float: right;
      background: none;
      border: none;
      color: #aaa;
      font-size: 20px;
      cursor: pointer;
      margin-top: -4px;
    }
    .close-btn:hover { color: #fff; }
    ul { list-style: none; padding: 0; margin: 0; }
    li button {
      width: 100%;
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 10px 14px;
      margin-bottom: 8px;
      background: #333;
      border: 1px solid #444;
      border-radius: 8px;
      color: #fff;
      font-size: 14px;
      cursor: pointer;
      transition: background 0.1s;
    }
    li button:hover { background: #444; }
    .badge {
      font-size: 11px;
      background: #ff0000;
      color: #fff;
      padding: 2px 6px;
      border-radius: 4px;
    }
  `;

  const overlay = document.createElement('div');
  overlay.className = 'overlay';
  overlay.addEventListener('click', (e) => {
    if (e.target === overlay) closeModal();
  });

  const modal = document.createElement('div');
  modal.className = 'modal';

  const closeBtn = document.createElement('button');
  closeBtn.className = 'close-btn';
  closeBtn.textContent = '✕';
  closeBtn.addEventListener('click', closeModal);

  const title = document.createElement('h2');
  title.textContent = 'Select quality';
  title.appendChild(closeBtn);

  const list = document.createElement('ul');
  for (const pair of pairs) {
    const li = document.createElement('li');
    const btn = document.createElement('button');

    const label = document.createElement('span');
    label.textContent = pair.label;

    const badge = document.createElement('span');
    badge.className = 'badge';
    badge.textContent = pair.videoCodec?.toUpperCase() ?? 'MP4';

    btn.appendChild(label);
    btn.appendChild(badge);
    btn.addEventListener('click', () => {
      closeModal();
      onSelect(pair);
    });
    li.appendChild(btn);
    list.appendChild(li);
  }

  modal.appendChild(title);
  modal.appendChild(list);
  overlay.appendChild(modal);
  shadow.appendChild(style);
  shadow.appendChild(overlay);

  function closeModal() {
    host.remove();
    onClose();
  }
}
