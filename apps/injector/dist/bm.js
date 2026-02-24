(function(){"use strict";const b="downloadtool-btn-host",_=["#above-the-fold #top-level-buttons-computed","#actions-inner #top-level-buttons-computed","ytd-watch-metadata #actions","#menu-container #top-level-buttons-computed"];function k(){for(const n of _){const e=document.querySelector(n);if(e)return e}return null}function S(n){var i;(i=document.getElementById(b))==null||i.remove();const e=k();if(!e){console.warn("[DownloadTool] Could not find YouTube actions container");return}const d=document.createElement("div");d.id=b,d.style.display="inline-flex",d.style.alignItems="center",d.style.marginLeft="8px";const o=d.attachShadow({mode:"closed"}),t=document.createElement("style");t.textContent=`
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
  `;const a=document.createElement("button");a.textContent="⬇ Download",a.addEventListener("click",()=>{a.classList.add("loading"),a.textContent="⏳ Loading...",n()}),o.appendChild(t),o.appendChild(a),e.appendChild(d)}function u(){const n=document.getElementById(b);if(!(n!=null&&n.shadowRoot))return;const e=n.shadowRoot.querySelector("button");e&&(e.classList.remove("loading"),e.textContent="⬇ Download")}const w="downloadtool-modal-host";function L(n,e,d){var C,y;(C=document.getElementById(w))==null||C.remove();const o=document.createElement("div");o.id=w,document.body.appendChild(o);const t=o.attachShadow({mode:"closed"}),a=document.createElement("style");a.textContent=`
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
  `;const i=document.createElement("div");i.className="overlay",i.addEventListener("click",s=>{s.target===i&&h()});const c=document.createElement("div");c.className="modal";const r=document.createElement("button");r.className="close-btn",r.textContent="✕",r.addEventListener("click",h);const l=document.createElement("h2");l.textContent="Select quality",l.appendChild(r);const m=document.createElement("ul");for(const s of n){const v=document.createElement("li"),f=document.createElement("button"),E=document.createElement("span");E.textContent=s.label;const g=document.createElement("span");g.className="badge",g.textContent=((y=s.videoCodec)==null?void 0:y.toUpperCase())??"MP4",f.appendChild(E),f.appendChild(g),f.addEventListener("click",()=>{h(),e(s)}),v.appendChild(f),m.appendChild(v)}c.appendChild(l),c.appendChild(m),i.appendChild(c),t.appendChild(a),t.appendChild(i);function h(){o.remove(),d()}}function T(n){return n.filter(e=>e.is_audio_only?!0:!(!e.has_audio&&e.ext==="webm"))}function D(n){const e=n.filter(t=>!t.is_audio_only&&!t.has_audio),o=n.filter(t=>t.is_audio_only).sort((t,a)=>(a.bitrate??0)-(t.bitrate??0))[0];return o?e.map(t=>{var a,i,c,r;return{label:t.quality,videoUrl:t.url,audioUrl:o.url,videoCodec:(a=t.codec_label)!=null&&a.toLowerCase().includes("264")?"h264":(i=t.codec_label)!=null&&i.toLowerCase().includes("265")?"h265":void 0,audioCodec:(c=o.codec_label)!=null&&c.toLowerCase().includes("aac")?"aac":(r=o.codec_label)!=null&&r.toLowerCase().includes("opus")?"opus":void 0}}):[]}function U(n,e,d){const o=new URLSearchParams({video_url:e.videoUrl,audio_url:e.audioUrl,title:d});return e.videoCodec&&o.set("video_codec",e.videoCodec),e.audioCodec&&o.set("audio_codec",e.audioCodec),`${n}/api/stream/muxed?${o.toString()}`}const p=document.currentScript,x=p!=null&&p.src?new URL(p.src).origin:window.location.origin;location.href.includes("youtube.com/watch")?S(N):console.info("[DownloadTool] Not a YouTube watch page, skipping.");async function N(){var e,d;const n=location.href;try{const o=await fetch(`${x}/api/extract`,{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({url:n})});if(!o.ok){const l=await o.json().catch(()=>({error:"Unknown error"}));alert(`[DownloadTool] Extraction failed: ${l.error??o.statusText}`),u();return}const t=await o.json(),a=((e=t.metadata)==null?void 0:e.title)??"video",i=((d=t.metadata)==null?void 0:d.formats)??[],c=T(i),r=D(c);if(r.length===0){alert("[DownloadTool] No downloadable formats found."),u();return}L(r,l=>{const m=U(x,l,a);window.location.href=m},u)}catch(o){console.error("[DownloadTool] Error:",o),alert(`[DownloadTool] Error: ${o instanceof Error?o.message:String(o)}`),u()}}})();
