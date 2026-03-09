(function(){"use strict";const b="downloadtool-btn-host",_=["#above-the-fold #top-level-buttons-computed","#actions-inner #top-level-buttons-computed","ytd-watch-metadata #actions","#menu-container #top-level-buttons-computed"];function k(){for(const n of _){const o=document.querySelector(n);if(o)return o}return null}function S(n){var d;(d=document.getElementById(b))==null||d.remove();const o=k();if(!o){console.warn("[DownloadTool] Could not find YouTube actions container");return}const r=document.createElement("div");r.id=b,r.style.display="inline-flex",r.style.alignItems="center",r.style.marginLeft="8px";const e=r.attachShadow({mode:"closed"}),t=document.createElement("style");t.textContent=`
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
  `;const a=document.createElement("button");a.textContent="⬇ Download",a.addEventListener("click",()=>{a.classList.add("loading"),a.textContent="⏳ Loading...",n()}),e.appendChild(t),e.appendChild(a),o.appendChild(r)}function u(){const n=document.getElementById(b);if(!(n!=null&&n.shadowRoot))return;const o=n.shadowRoot.querySelector("button");o&&(o.classList.remove("loading"),o.textContent="⬇ Download")}const g="downloadtool-modal-host";function L(n,o,r){var y,C;(y=document.getElementById(g))==null||y.remove();const e=document.createElement("div");e.id=g,document.body.appendChild(e);const t=e.attachShadow({mode:"closed"}),a=document.createElement("style");a.textContent=`
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
  `;const d=document.createElement("div");d.className="overlay",d.addEventListener("click",s=>{s.target===d&&h()});const i=document.createElement("div");i.className="modal";const l=document.createElement("button");l.className="close-btn",l.textContent="✕",l.addEventListener("click",h);const c=document.createElement("h2");c.textContent="Select quality",c.appendChild(l);const f=document.createElement("ul");for(const s of n){const v=document.createElement("li"),p=document.createElement("button"),E=document.createElement("span");E.textContent=s.label;const x=document.createElement("span");x.className="badge",x.textContent=((C=s.videoCodec)==null?void 0:C.toUpperCase())??"MP4",p.appendChild(E),p.appendChild(x),p.addEventListener("click",()=>{h(),o(s)}),v.appendChild(p),f.appendChild(v)}i.appendChild(c),i.appendChild(f),d.appendChild(i),t.appendChild(a),t.appendChild(d);function h(){e.remove(),r()}}function T(n){return n.filter(o=>o.is_audio_only?!0:!(!o.has_audio&&o.ext==="webm"))}function D(n){const o=n.filter(t=>!t.is_audio_only&&!t.has_audio),e=[...n.filter(t=>t.is_audio_only)].sort((t,a)=>{const d=t.ext==="m4a"||t.ext==="mp4"?0:1,i=a.ext==="m4a"||a.ext==="mp4"?0:1;return d!==i?d-i:(a.bitrate??0)-(t.bitrate??0)})[0];return e?o.map(t=>{var a,d,i,l;return{label:t.quality,videoUrl:t.url,audioUrl:e.url,videoFormatId:t.format_id,audioFormatId:e.format_id,videoCodec:(a=t.codec_label)!=null&&a.toLowerCase().includes("264")?"h264":(d=t.codec_label)!=null&&d.toLowerCase().includes("265")?"h265":void 0,audioCodec:(i=e.codec_label)!=null&&i.toLowerCase().includes("aac")?"aac":(l=e.codec_label)!=null&&l.toLowerCase().includes("opus")?"opus":void 0}}):[]}function I(n,o,r,e){const t=new URLSearchParams({video_url:o.videoUrl,audio_url:o.audioUrl,title:r});return e&&t.set("source_url",e),o.videoFormatId&&t.set("video_format_id",o.videoFormatId),o.audioFormatId&&t.set("audio_format_id",o.audioFormatId),`${n}/download/mux-job?${t.toString()}`}const m=document.currentScript,w=m!=null&&m.src?new URL(m.src).origin:window.location.origin;location.href.includes("youtube.com/watch")?S(R):console.info("[DownloadTool] Not a YouTube watch page, skipping.");async function R(){var o,r;const n=location.href;try{const e=await fetch(`${w}/api/extract`,{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({url:n})});if(!e.ok){const c=await e.json().catch(()=>({error:"Unknown error"}));alert(`[DownloadTool] Extraction failed: ${c.error??e.statusText}`),u();return}const t=await e.json(),a=((o=t.metadata)==null?void 0:o.title)??"video",d=((r=t.metadata)==null?void 0:r.formats)??[],i=T(d),l=D(i);if(l.length===0){alert("[DownloadTool] No downloadable formats found."),u();return}L(l,c=>{const f=I(w,c,a,n);window.location.href=f},u)}catch(e){console.error("[DownloadTool] Error:",e),alert(`[DownloadTool] Error: ${e instanceof Error?e.message:String(e)}`),u()}}})();
