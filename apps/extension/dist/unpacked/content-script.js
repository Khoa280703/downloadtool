(function(){"use strict";const i="downloadtool-ext-host",m=["#above-the-fold #top-level-buttons-computed","#actions-inner #top-level-buttons-computed","ytd-watch-metadata #actions","#menu-container #top-level-buttons-computed"];function f(){for(const n of m){const t=document.querySelector(n);if(t)return t}return null}function r(){var u;if((u=document.getElementById(i))==null||u.remove(),!location.href.includes("youtube.com/watch"))return;const n=f();if(!n)return;const t=document.createElement("div");t.id=i,t.style.cssText="display:inline-flex;align-items:center;margin-left:8px";const l=t.attachShadow({mode:"open"}),d=document.createElement("style");d.textContent=`
    button {
      display:inline-flex;align-items:center;gap:6px;
      padding:0 16px;height:36px;border:none;border-radius:18px;
      background:#ff0000;color:#fff;font-size:14px;font-weight:500;
      font-family:Roboto,Arial,sans-serif;cursor:pointer;white-space:nowrap;
      transition:background .15s;
    }
    button:hover{background:#cc0000}
    button.loading{background:#888;cursor:wait}
  `;const e=document.createElement("button");e.textContent="⬇ Download",e.addEventListener("click",async()=>{e.classList.add("loading"),e.textContent="⏳ Loading...",chrome.runtime.sendMessage({type:"EXTRACT",url:location.href},o=>{var s;e.classList.remove("loading"),e.textContent="⬇ Download",(chrome.runtime.lastError||o!=null&&o.error)&&alert(`[DownloadTool] ${(o==null?void 0:o.error)??((s=chrome.runtime.lastError)==null?void 0:s.message)}`)})}),l.appendChild(d),l.appendChild(e),n.appendChild(t)}r();let c=document.title,a;new MutationObserver(()=>{document.title!==c&&(c=document.title,clearTimeout(a),a=setTimeout(r,500))}).observe(document.querySelector("title")??document.head,{subtree:!0,characterData:!0,childList:!0})})();
