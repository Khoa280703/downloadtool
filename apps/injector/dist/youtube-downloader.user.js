// ==UserScript==
// @name         YouTube Downloader
// @namespace    https://yourdomain.com
// @version      1.0.0
// @description  Download YouTube videos in the best quality
// @downloadURL  https://yourdomain.com/userscript
// @updateURL    https://yourdomain.com/userscript
// @match        https://www.youtube.com/watch*
// @match        https://youtube.com/watch*
// @connect      yourdomain.com
// @grant        GM_xmlhttpRequest
// @run-at       document-idle
// ==/UserScript==

(function () {
  'use strict';

  const BUTTON_HOST_ID = "downloadtool-btn-host";
  const ANCHOR_SELECTORS = [
    "#above-the-fold #top-level-buttons-computed",
    "#actions-inner #top-level-buttons-computed",
    "ytd-watch-metadata #actions",
    "#menu-container #top-level-buttons-computed"
  ];
  function findActionsContainer() {
    for (const sel of ANCHOR_SELECTORS) {
      const el = document.querySelector(sel);
      if (el) return el;
    }
    return null;
  }
  function injectButton(onClick) {
    var _a;
    (_a = document.getElementById(BUTTON_HOST_ID)) == null ? void 0 : _a.remove();
    const container = findActionsContainer();
    if (!container) {
      console.warn("[DownloadTool] Could not find YouTube actions container");
      return;
    }
    const host = document.createElement("div");
    host.id = BUTTON_HOST_ID;
    host.style.display = "inline-flex";
    host.style.alignItems = "center";
    host.style.marginLeft = "8px";
    const shadow = host.attachShadow({ mode: "closed" });
    const style = document.createElement("style");
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
    const btn = document.createElement("button");
    btn.textContent = "⬇ Download";
    btn.addEventListener("click", () => {
      btn.classList.add("loading");
      btn.textContent = "⏳ Loading...";
      onClick();
    });
    shadow.appendChild(style);
    shadow.appendChild(btn);
    container.appendChild(host);
  }
  function resetButton() {
    const host = document.getElementById(BUTTON_HOST_ID);
    if (!(host == null ? void 0 : host.shadowRoot)) return;
    const btn = host.shadowRoot.querySelector("button");
    if (!btn) return;
    btn.classList.remove("loading");
    btn.textContent = "⬇ Download";
  }
  const MODAL_HOST_ID = "downloadtool-modal-host";
  function showQualityPicker(pairs, onSelect, onClose) {
    var _a, _b;
    (_a = document.getElementById(MODAL_HOST_ID)) == null ? void 0 : _a.remove();
    const host = document.createElement("div");
    host.id = MODAL_HOST_ID;
    document.body.appendChild(host);
    const shadow = host.attachShadow({ mode: "closed" });
    const style = document.createElement("style");
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
    const overlay = document.createElement("div");
    overlay.className = "overlay";
    overlay.addEventListener("click", (e) => {
      if (e.target === overlay) closeModal();
    });
    const modal = document.createElement("div");
    modal.className = "modal";
    const closeBtn = document.createElement("button");
    closeBtn.className = "close-btn";
    closeBtn.textContent = "✕";
    closeBtn.addEventListener("click", closeModal);
    const title = document.createElement("h2");
    title.textContent = "Select quality";
    title.appendChild(closeBtn);
    const list = document.createElement("ul");
    for (const pair of pairs) {
      const li = document.createElement("li");
      const btn = document.createElement("button");
      const label = document.createElement("span");
      label.textContent = pair.label;
      const badge = document.createElement("span");
      badge.className = "badge";
      badge.textContent = ((_b = pair.videoCodec) == null ? void 0 : _b.toUpperCase()) ?? "MP4";
      btn.appendChild(label);
      btn.appendChild(badge);
      btn.addEventListener("click", () => {
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
  function filterDownloadableFormats(formats) {
    return formats.filter((f) => {
      if (f.is_audio_only) return true;
      if (!f.has_audio && f.ext === "webm") return false;
      return true;
    });
  }
  function buildMuxedPairs(formats) {
    const videoStreams = formats.filter((f) => !f.is_audio_only && !f.has_audio);
    const audioStreams = formats.filter((f) => f.is_audio_only);
    const bestAudio = audioStreams.sort((a, b) => (b.bitrate ?? 0) - (a.bitrate ?? 0))[0];
    if (!bestAudio) return [];
    return videoStreams.map((v) => {
      var _a, _b, _c, _d;
      return {
        label: v.quality,
        videoUrl: v.url,
        audioUrl: bestAudio.url,
        videoCodec: ((_a = v.codec_label) == null ? void 0 : _a.toLowerCase().includes("264")) ? "h264" : ((_b = v.codec_label) == null ? void 0 : _b.toLowerCase().includes("265")) ? "h265" : void 0,
        audioCodec: ((_c = bestAudio.codec_label) == null ? void 0 : _c.toLowerCase().includes("aac")) ? "aac" : ((_d = bestAudio.codec_label) == null ? void 0 : _d.toLowerCase().includes("opus")) ? "opus" : void 0
      };
    });
  }
  function buildMuxedUrl(apiBase, pair, title) {
    const params = new URLSearchParams({
      video_url: pair.videoUrl,
      audio_url: pair.audioUrl,
      title
    });
    if (pair.videoCodec) params.set("video_codec", pair.videoCodec);
    if (pair.audioCodec) params.set("audio_codec", pair.audioCodec);
    return `${apiBase}/api/stream/muxed?${params.toString()}`;
  }
  const API_BASE = "https://yourdomain.com";
  function gmFetch(url, options) {
    return new Promise((resolve, reject) => {
      GM_xmlhttpRequest({
        method: options.method,
        url,
        headers: { "Content-Type": "application/json" },
        data: options.body,
        onload: (res) => {
          resolve({
            ok: res.status >= 200 && res.status < 300,
            json: () => Promise.resolve(JSON.parse(res.responseText))
          });
        },
        onerror: reject
      });
    });
  }
  async function handleDownload() {
    var _a, _b;
    const videoUrl = location.href;
    try {
      const res = await gmFetch(`${API_BASE}/api/extract`, {
        method: "POST",
        body: JSON.stringify({ url: videoUrl })
      });
      if (!res.ok) {
        const err = await res.json().catch(() => ({ error: "Unknown error" }));
        alert(`[DownloadTool] Extraction failed: ${err.error}`);
        resetButton();
        return;
      }
      const data = await res.json();
      const title = ((_a = data.metadata) == null ? void 0 : _a.title) ?? "video";
      const allFormats = ((_b = data.metadata) == null ? void 0 : _b.formats) ?? [];
      const downloadable = filterDownloadableFormats(allFormats);
      const pairs = buildMuxedPairs(downloadable);
      if (pairs.length === 0) {
        alert("[DownloadTool] No downloadable formats found.");
        resetButton();
        return;
      }
      showQualityPicker(
        pairs,
        (pair) => {
          window.location.href = buildMuxedUrl(API_BASE, pair, title);
        },
        resetButton
      );
    } catch (e) {
      console.error("[DownloadTool]", e);
      alert(`[DownloadTool] Error: ${e instanceof Error ? e.message : String(e)}`);
      resetButton();
    }
  }
  function observeNavigation() {
    let lastTitle = document.title;
    let debounce;
    new MutationObserver(() => {
      if (document.title !== lastTitle) {
        lastTitle = document.title;
        clearTimeout(debounce);
        debounce = setTimeout(() => {
          if (location.href.includes("youtube.com/watch")) {
            injectButton(handleDownload);
          }
        }, 500);
      }
    }).observe(document.querySelector("title") ?? document.head, {
      subtree: true,
      characterData: true,
      childList: true
    });
  }
  if (location.href.includes("youtube.com/watch")) {
    injectButton(handleDownload);
  }
  observeNavigation();

})();