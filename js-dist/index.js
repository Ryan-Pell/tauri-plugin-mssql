var e=Object.defineProperty;function n(e,n=!1){let t=window.crypto.getRandomValues(new Uint32Array(1))[0],r=`_${t}`;return Object.defineProperty(window,r,{value:t=>(n&&Reflect.deleteProperty(window,r),null==e?void 0:e(t)),writable:!1,configurable:!0}),t}async function t(e,t={}){return new Promise(((r,o)=>{let c=n((e=>{r(e),Reflect.deleteProperty(window,`_${i}`)}),!0),i=n((e=>{o(e),Reflect.deleteProperty(window,`_${c}`)}),!0);window.__TAURI_IPC__({cmd:e,callback:c,error:i,...t})}))}function r(e,n="asset"){let t=encodeURIComponent(e);return navigator.userAgent.includes("Windows")?`https://${n}.localhost/${t}`:`${n}://localhost/${t}`}function o(e){return new Promise(((n,r)=>{let o={};e&&(o={...o,db:e}),t("plugin:mssql|connect",o).then((()=>n(!0))).catch((e=>r(JSON.parse(e))))}))}function c(){return new Promise(((e,n)=>{t("plugin:mssql|disconnect").then((()=>e(!0))).catch((e=>n(JSON.parse(e))))}))}function i(e,n){return new Promise(((r,o)=>{let c={tsql:e};n&&(c={...c,connection:n}),t("plugin:mssql|query",c).then((e=>{r({raw:e,json:()=>JSON.parse(e.toString())})})).catch((n=>o({error:n,query:e})))}))}function l(){return new Promise(((e,n)=>{t("plugin:mssql|default_config").then((n=>e(JSON.parse(n)))).catch((e=>n(e)))}))}function s(){return new Promise((e=>{t("plugin:mssql|connection_status").then((n=>e("true"===String(n).toLowerCase())))}))}((n,t)=>{for(var r in t)e(n,r,{get:t[r],enumerable:!0})})({},{convertFileSrc:()=>r,invoke:()=>t,transformCallback:()=>n});export{o as connect,s as connectionStatus,l as defaultConnectionString,c as disconnect,i as query};
//# sourceMappingURL=index.js.map
