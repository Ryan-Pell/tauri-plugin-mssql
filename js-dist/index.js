var e=Object.defineProperty;function n(e,n=!1){let t=window.crypto.getRandomValues(new Uint32Array(1))[0],r=`_${t}`;return Object.defineProperty(window,r,{value:t=>(n&&Reflect.deleteProperty(window,r),null==e?void 0:e(t)),writable:!1,configurable:!0}),t}async function t(e,t={}){return new Promise(((r,o)=>{let l=n((e=>{r(e),Reflect.deleteProperty(window,`_${c}`)}),!0),c=n((e=>{o(e),Reflect.deleteProperty(window,`_${l}`)}),!0);window.__TAURI_IPC__({cmd:e,callback:l,error:c,...t})}))}function r(e,n="asset"){let t=encodeURIComponent(e);return navigator.userAgent.includes("Windows")?`https://${n}.localhost/${t}`:`${n}://localhost/${t}`}async function o(e,n){let r={tsql:e};n&&(r={...r,connection:n});let o=await t("plugin:mssql|query",r);return{raw:o,json:()=>JSON.parse(o)}}async function l(){}((n,t)=>{for(var r in t)e(n,r,{get:t[r],enumerable:!0})})({},{convertFileSrc:()=>r,invoke:()=>t,transformCallback:()=>n});export{l as defaultConnectionString,o as query};
//# sourceMappingURL=index.js.map
