var ln=(S,h)=>()=>(h||S((h={exports:{}}).exports,h),h.exports);var fn=ln((gn,O)=>{(async()=>{(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const _ of document.querySelectorAll('link[rel="modulepreload"]'))t(_);new MutationObserver(_=>{for(const r of _)if(r.type==="childList")for(const a of r.addedNodes)a.tagName==="LINK"&&a.rel==="modulepreload"&&t(a)}).observe(document,{childList:!0,subtree:!0});function n(_){const r={};return _.integrity&&(r.integrity=_.integrity),_.referrerPolicy&&(r.referrerPolicy=_.referrerPolicy),_.crossOrigin==="use-credentials"?r.credentials="include":_.crossOrigin==="anonymous"?r.credentials="omit":r.credentials="same-origin",r}function t(_){if(_.ep)return;_.ep=!0;const r=n(_);fetch(_.href,r)}})();const S="/assets/wasm_bg--_rAyLjU.wasm",h=async(e={},n)=>{let t;if(n.startsWith("data:")){const _=n.replace(/^data:.*?base64,/,"");let r;if(typeof Buffer=="function"&&typeof Buffer.from=="function")r=Buffer.from(_,"base64");else if(typeof atob=="function"){const a=atob(_);r=new Uint8Array(a.length);for(let s=0;s<a.length;s++)r[s]=a.charCodeAt(s)}else throw new Error("Cannot decode base64-encoded data URL");t=await WebAssembly.instantiate(r,e)}else{const _=await fetch(n),r=_.headers.get("Content-Type")||"";if("instantiateStreaming"in WebAssembly&&r.startsWith("application/wasm"))t=await WebAssembly.instantiateStreaming(_,e);else{const a=await _.arrayBuffer();t=await WebAssembly.instantiate(a,e)}}return t.instance.exports};let o;function D(e){o=e}const y=new Array(128).fill(void 0);y.push(void 0,null,!0,!1);function c(e){return y[e]}let k=y.length;function R(e){e<132||(y[e]=k,k=e)}function W(e){const n=c(e);return R(e),n}const P=typeof TextDecoder>"u"?(0,O.require)("util").TextDecoder:TextDecoder;let B=new P("utf-8",{ignoreBOM:!0,fatal:!0});B.decode();let T=null;function E(){return(T===null||T.byteLength===0)&&(T=new Uint8Array(o.memory.buffer)),T}function f(e,n){return e=e>>>0,B.decode(E().subarray(e,e+n))}function d(e){k===y.length&&y.push(y.length+1);const n=k;return k=y[n],y[n]=e,n}let b=0;const q=typeof TextEncoder>"u"?(0,O.require)("util").TextEncoder:TextEncoder;let L=new q("utf-8");const I=typeof L.encodeInto=="function"?function(e,n){return L.encodeInto(e,n)}:function(e,n){const t=L.encode(e);return n.set(t),{read:e.length,written:t.length}};function m(e,n,t){if(t===void 0){const i=L.encode(e),w=n(i.length,1)>>>0;return E().subarray(w,w+i.length).set(i),b=i.length,w}let _=e.length,r=n(_,1)>>>0;const a=E();let s=0;for(;s<_;s++){const i=e.charCodeAt(s);if(i>127)break;a[r+s]=i}if(s!==_){s!==0&&(e=e.slice(s)),r=t(r,_,_=s+e.length*3,1)>>>0;const i=E().subarray(r+s,r+_),w=I(e,i);s+=w.written}return b=s,r}function g(e){return e==null}let A=null;function v(){return(A===null||A.byteLength===0)&&(A=new Int32Array(o.memory.buffer)),A}function j(e){const n=typeof e;if(n=="number"||n=="boolean"||e==null)return`${e}`;if(n=="string")return`"${e}"`;if(n=="symbol"){const r=e.description;return r==null?"Symbol":`Symbol(${r})`}if(n=="function"){const r=e.name;return typeof r=="string"&&r.length>0?`Function(${r})`:"Function"}if(Array.isArray(e)){const r=e.length;let a="[";r>0&&(a+=j(e[0]));for(let s=1;s<r;s++)a+=", "+j(e[s]);return a+="]",a}const t=/\[object ([^\]]+)\]/.exec(toString.call(e));let _;if(t.length>1)_=t[1];else return toString.call(e);if(_=="Object")try{return"Object("+JSON.stringify(e)+")"}catch{return"Object"}return e instanceof Error?`${e.name}: ${e.message}
${e.stack}`:_}function l(e,n){try{return e.apply(this,n)}catch(t){o.__wbindgen_exn_store(d(t))}}class ${__destroy_into_raw(){const n=this.__wbg_ptr;return this.__wbg_ptr=0,n}free(){const n=this.__destroy_into_raw();o.__wbg_canvas_free(n)}constructor(n,t){const _=m(n,o.__wbindgen_malloc,o.__wbindgen_realloc),r=b;var a=g(t)?0:m(t,o.__wbindgen_malloc,o.__wbindgen_realloc),s=b;const i=o.canvas_new(_,r,a,s);return this.__wbg_ptr=i>>>0,this}render(){o.canvas_render(this.__wbg_ptr)}get_js_canvas(){const n=o.canvas_get_js_canvas(this.__wbg_ptr);return W(n)}add_line(n,t,_,r,a){const s=o.canvas_add_line(this.__wbg_ptr,n,t,_,r,d(a));return p.__wrap(s)}add_rect(n,t,_,r,a){const s=o.canvas_add_rect(this.__wbg_ptr,n,t,_,r,d(a));return p.__wrap(s)}add_circle(n,t,_,r){const a=o.canvas_add_circle(this.__wbg_ptr,n,t,_,d(r));return p.__wrap(a)}add_text(n,t,_,r){const a=m(n,o.__wbindgen_malloc,o.__wbindgen_realloc),s=b,i=o.canvas_add_text(this.__wbg_ptr,a,s,t,_,d(r));return p.__wrap(i)}on_mouse_down(n,t,_){o.canvas_on_mouse_down(this.__wbg_ptr,n,t,_)}on_mouse_move(n,t,_){o.canvas_on_mouse_move(this.__wbg_ptr,n,t,_)}on_mouse_up(n,t,_){o.canvas_on_mouse_up(this.__wbg_ptr,n,t,_)}on_key_down(n,t){const _=m(n,o.__wbindgen_malloc,o.__wbindgen_realloc),r=b;o.canvas_on_key_down(this.__wbg_ptr,_,r,t)}on_key_up(n,t){const _=m(n,o.__wbindgen_malloc,o.__wbindgen_realloc),r=b;o.canvas_on_key_up(this.__wbg_ptr,_,r,t)}on_key_press(n,t){const _=m(n,o.__wbindgen_malloc,o.__wbindgen_realloc),r=b;o.canvas_on_key_press(this.__wbg_ptr,_,r,t)}}class p{static __wrap(n){n=n>>>0;const t=Object.create(p.prototype);return t.__wbg_ptr=n,t}__destroy_into_raw(){const n=this.__wbg_ptr;return this.__wbg_ptr=0,n}free(){const n=this.__destroy_into_raw();o.__wbg_noderef_free(n)}translate(n,t){o.noderef_translate(this.__wbg_ptr,n,t)}set_style(n){o.noderef_set_style(this.__wbg_ptr,d(n))}rotation(n,t,_,r){var a=g(t)?0:m(t,o.__wbindgen_malloc,o.__wbindgen_realloc),s=b;o.noderef_rotation(this.__wbg_ptr,n,a,s,!g(_),g(_)?0:_,!g(r),g(r)?0:r)}remove(){const n=this.__destroy_into_raw();o.noderef_remove(n)}is_hovered(n,t){return o.noderef_is_hovered(this.__wbg_ptr,n,t)!==0}}function F(e){W(e)}function N(e,n){const t=f(e,n);return d(t)}function U(e){return c(e)===void 0}function H(){const e=new Error;return d(e)}function M(e,n){const t=c(n).stack,_=m(t,o.__wbindgen_malloc,o.__wbindgen_realloc),r=b;v()[e/4+1]=r,v()[e/4+0]=_}function J(e,n){let t,_;try{t=e,_=n,console.error(f(e,n))}finally{o.__wbindgen_free(t,_,1)}}function X(e){let n;try{n=c(e)instanceof Window}catch{n=!1}return n}function Y(e){const n=c(e).document;return g(n)?0:d(n)}function z(e,n,t){const _=c(e).getElementById(f(n,t));return g(_)?0:d(_)}function K(e){let n;try{n=c(e)instanceof CanvasRenderingContext2D}catch{n=!1}return n}function G(e){const n=c(e).canvas;return g(n)?0:d(n)}function Q(e,n){c(e).strokeStyle=c(n)}function V(e,n){c(e).fillStyle=c(n)}function Z(e,n){c(e).lineWidth=n}function ee(e,n,t){c(e).font=f(n,t)}function ne(e,n,t){c(e).textBaseline=f(n,t)}function te(e){c(e).beginPath()}function _e(e){c(e).fill()}function re(e){c(e).stroke()}function oe(){return l(function(e,n,t,_,r,a){c(e).arc(n,t,_,r,a)},arguments)}function ce(e){c(e).closePath()}function ae(e,n,t){c(e).lineTo(n,t)}function se(e,n,t){c(e).moveTo(n,t)}function ie(e,n,t,_,r){c(e).rect(n,t,_,r)}function ue(e,n,t,_,r){c(e).clearRect(n,t,_,r)}function de(e,n,t,_,r){c(e).fillRect(n,t,_,r)}function le(e){c(e).restore()}function fe(e){c(e).save()}function be(){return l(function(e,n,t,_,r){c(e).fillText(f(n,t),_,r)},arguments)}function ge(){return l(function(e,n,t){const _=c(e).measureText(f(n,t));return d(_)},arguments)}function we(){return l(function(e,n,t,_,r){c(e).strokeText(f(n,t),_,r)},arguments)}function me(){return l(function(e,n){c(e).rotate(n)},arguments)}function ye(){return l(function(e,n,t){c(e).translate(n,t)},arguments)}function he(e){let n;try{n=c(e)instanceof HTMLCanvasElement}catch{n=!1}return n}function ve(e){return c(e).width}function pe(e){return c(e).height}function ke(){return l(function(e,n,t,_){const r=c(e).getContext(f(n,t),c(_));return g(r)?0:d(r)},arguments)}function xe(e){return c(e).width}function Te(e,n){const t=new Function(f(e,n));return d(t)}function Ee(){return l(function(e,n){const t=c(e).call(c(n));return d(t)},arguments)}function Le(){return l(function(){const e=self.self;return d(e)},arguments)}function Ae(){return l(function(){const e=window.window;return d(e)},arguments)}function Se(){return l(function(){const e=globalThis.globalThis;return d(e)},arguments)}function je(){return l(function(){const e=global.global;return d(e)},arguments)}function Ce(e){const n=c(e);return d(n)}function Oe(e,n){const t=c(n),_=typeof t=="string"?t:void 0;var r=g(_)?0:m(_,o.__wbindgen_malloc,o.__wbindgen_realloc),a=b;v()[e/4+1]=a,v()[e/4+0]=r}function We(){return l(function(e,n){const t=JSON.parse(f(e,n));return d(t)},arguments)}function Be(){return l(function(e){const n=JSON.stringify(c(e));return d(n)},arguments)}function De(e,n){const t=j(c(n)),_=m(t,o.__wbindgen_malloc,o.__wbindgen_realloc),r=b;v()[e/4+1]=r,v()[e/4+0]=_}function Re(e,n){throw new Error(f(e,n))}URL=globalThis.URL;const u=await h({"./wasm_bg.js":{__wbindgen_object_drop_ref:F,__wbindgen_string_new:N,__wbindgen_is_undefined:U,__wbg_new_abda76e883ba8a5f:H,__wbg_stack_658279fe44541cf6:M,__wbg_error_f851667af71bcfc6:J,__wbg_instanceof_Window_cde2416cf5126a72:X,__wbg_document_183cf1eecfdbffee:Y,__wbg_getElementById_328f8c4a5bb51ba8:z,__wbg_instanceof_CanvasRenderingContext2d_e264df6db9ec5a3d:K,__wbg_canvas_840f8824dfdda709:G,__wbg_setstrokeStyle_1bf67b48c7e92f7c:Q,__wbg_setfillStyle_343558d6a1a50509:V,__wbg_setlineWidth_52861f70ee5fc11d:Z,__wbg_setfont_aa132dcc79171764:ee,__wbg_settextBaseline_995123d99acaaab3:ne,__wbg_beginPath_dcaf84daa6be35fe:te,__wbg_fill_530550bd1c480bcf:_e,__wbg_stroke_abe71960396d06f0:re,__wbg_arc_267b3955b82dae7d:oe,__wbg_closePath_45d0b0af592ad33e:ce,__wbg_lineTo_577441645a6c05ee:ae,__wbg_moveTo_96f4d56b6d86d473:se,__wbg_rect_72792bb8280bef83:ie,__wbg_clearRect_bba8d57f3a4d13b9:ue,__wbg_fillRect_beae00c04b0cfb93:de,__wbg_restore_74a0c86b727a531b:le,__wbg_save_61ede9e2b8a62e6f:fe,__wbg_fillText_cebdfe9ab584bd2c:be,__wbg_measureText_d5f74b84861f26fa:ge,__wbg_strokeText_70522e9939cd77c0:we,__wbg_rotate_29ee26841d3e9d62:me,__wbg_translate_5368ecd407f90026:ye,__wbg_instanceof_HtmlCanvasElement_838d8b92f3c55028:he,__wbg_width_b813b325b323728a:ve,__wbg_height_646e862bac72cff1:pe,__wbg_getContext_897a215471051682:ke,__wbg_width_f07bb821f6c98e0b:xe,__wbg_newnoargs_ccdcae30fd002262:Te,__wbg_call_669127b9d730c650:Ee,__wbg_self_3fad056edded10bd:Le,__wbg_window_a4f46c98a61d4089:Ae,__wbg_globalThis_17eff828815f7d84:Se,__wbg_global_46f939f6541643c5:je,__wbindgen_object_clone_ref:Ce,__wbindgen_string_get:Oe,__wbg_parse_3f0cb48976ca4123:We,__wbg_stringify_4039297315a25b00:Be,__wbindgen_debug_string:De,__wbindgen_throw:Re}},S),Pe=u.memory,qe=u.__wbg_context_free,Ie=u.__wbg_canvas_free,$e=u.canvas_new,Fe=u.canvas_render,Ne=u.canvas_get_js_canvas,Ue=u.canvas_add_line,He=u.canvas_add_rect,Me=u.canvas_add_circle,Je=u.canvas_add_text,Xe=u.canvas_on_mouse_down,Ye=u.canvas_on_mouse_move,ze=u.canvas_on_mouse_up,Ke=u.canvas_on_key_down,Ge=u.canvas_on_key_up,Qe=u.canvas_on_key_press,Ve=u.__wbg_noderef_free,Ze=u.noderef_translate,en=u.noderef_set_style,nn=u.noderef_rotation,tn=u.noderef_remove,_n=u.noderef_is_hovered,rn=u.__wbindgen_malloc,on=u.__wbindgen_realloc,cn=u.__wbindgen_free,an=u.__wbindgen_exn_store,sn=Object.freeze(Object.defineProperty({__proto__:null,__wbg_canvas_free:Ie,__wbg_context_free:qe,__wbg_noderef_free:Ve,__wbindgen_exn_store:an,__wbindgen_free:cn,__wbindgen_malloc:rn,__wbindgen_realloc:on,canvas_add_circle:Me,canvas_add_line:Ue,canvas_add_rect:He,canvas_add_text:Je,canvas_get_js_canvas:Ne,canvas_new:$e,canvas_on_key_down:Ke,canvas_on_key_press:Qe,canvas_on_key_up:Ge,canvas_on_mouse_down:Xe,canvas_on_mouse_move:Ye,canvas_on_mouse_up:ze,canvas_render:Fe,memory:Pe,noderef_is_hovered:_n,noderef_remove:tn,noderef_rotation:nn,noderef_set_style:en,noderef_translate:Ze},Symbol.toStringTag,{value:"Module"}));D(sn);function C(e,n,t){const _=e.getBoundingClientRect();return _==null?{x:n,y:t}:{x:n-_.left,y:t-_.top}}function un(e){const n=new $(e,"black"),t=n.get_js_canvas();if(t==null)return;document.addEventListener("mousemove",i=>{const{x:w,y:x}=C(t,i.clientX,i.clientY);n.on_mouse_move(w,x,Date.now())}),document.addEventListener("mousedown",i=>{const{x:w,y:x}=C(t,i.clientX,i.clientY);n.on_mouse_down(w,x,Date.now())}),document.addEventListener("mouseup",i=>{const{x:w,y:x}=C(t,i.clientX,i.clientY);n.on_mouse_up(w,x,Date.now())}),document.addEventListener("keydown",i=>{n.on_key_down(i.key,Date.now())}),document.addEventListener("keyup",i=>{n.on_key_up(i.key,Date.now())}),document.addEventListener("keypress",i=>{n.on_key_press(i.key,Date.now())}),n.add_circle(500,100,50,{fill_color:"red",stroke_width:5,stroke_color:"blue"}),n.add_rect(100,100,15,100,{fill_color:"white"}),n.add_rect(400,100,15,100,{fill_color:"white"});const _=n.add_text("Hello World",431,503,{font_size:20,fill_color:"white"});let r=0,a=0;const s=async()=>{_.translate(r+100,a+50),_.rotation(25,"tl"),n.render(),requestAnimationFrame(s)};requestAnimationFrame(s)}document.querySelector("#app").innerHTML=`
  <canvas id="canvas" width="500" height="500"></canvas>
`;function dn(){const e=document.getElementById("canvas");if(e==null)return;const n=window.innerWidth,t=window.innerHeight;e.width=n-20,e.height=t-20,un("canvas")}requestAnimationFrame(dn)})()});export default fn();
