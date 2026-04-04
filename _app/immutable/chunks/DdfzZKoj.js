import { h as S, aR as x, aS as E, N as M, aT as L, aU as U, aV as W, aW as C, aX as I } from "./D3xIz7z5.js";
let ie, se, ae, p;
let __tla = (async () => {
  const N = /* @__PURE__ */ Symbol("is custom element"), R = /* @__PURE__ */ Symbol("is html"), O = E ? "link" : "LINK";
  ae = function(e) {
    if (S) {
      var t = false, n = () => {
        if (!t) {
          if (t = true, e.hasAttribute("value")) {
            var o = e.value;
            p(e, "value", null), e.value = o;
          }
          if (e.hasAttribute("checked")) {
            var r = e.checked;
            p(e, "checked", null), e.checked = r;
          }
        }
      };
      e.__on_r = n, M(n), L();
    }
  };
  p = function(e, t, n, o) {
    var r = B(e);
    S && (r[t] = e.getAttribute(t), t === "src" || t === "srcset" || t === "href" && e.nodeName === O) || r[t] !== (r[t] = n) && (t === "loading" && (e[U] = n), n == null ? e.removeAttribute(t) : typeof n != "string" && D(e).includes(t) ? e[t] = n : e.setAttribute(t, n));
  };
  function B(e) {
    return e.__attributes ?? (e.__attributes = {
      [N]: e.nodeName.includes("-"),
      [R]: e.namespaceURI === x
    });
  }
  var m = /* @__PURE__ */ new Map();
  function D(e) {
    var t = e.getAttribute("is") || e.nodeName, n = m.get(t);
    if (n) return n;
    m.set(t, n = []);
    for (var o, r = e, _ = Element.prototype; _ !== r; ) {
      o = C(r);
      for (var s in o) o[s].set && n.push(s);
      r = W(r);
    }
    return n;
  }
  const F = "" + new URL("../assets/kernel_bg.7idnPtHn.wasm", import.meta.url).href, z = async (e = {}, t) => {
    let n;
    if (t.startsWith("data:")) {
      const o = t.replace(/^data:.*?base64,/, "");
      let r;
      if (typeof Buffer == "function" && typeof Buffer.from == "function") r = Buffer.from(o, "base64");
      else if (typeof atob == "function") {
        const _ = atob(o);
        r = new Uint8Array(_.length);
        for (let s = 0; s < _.length; s++) r[s] = _.charCodeAt(s);
      } else throw new Error("Cannot decode base64-encoded data URL");
      n = await WebAssembly.instantiate(r, e);
    } else {
      const o = await fetch(t), r = o.headers.get("Content-Type") || "";
      if ("instantiateStreaming" in WebAssembly && r.startsWith("application/wasm")) n = await WebAssembly.instantiateStreaming(o, e);
      else {
        const _ = await o.arrayBuffer();
        n = await WebAssembly.instantiate(_, e);
      }
    }
    return n.instance.exports;
  };
  class h {
    __destroy_into_raw() {
      const t = this.__wbg_ptr;
      return this.__wbg_ptr = 0, A.unregister(this), t;
    }
    free() {
      const t = this.__destroy_into_raw();
      c.__wbg_cassidakernel_free(t, 0);
    }
    execute(t) {
      let n, o;
      try {
        const r = j(t, c.__wbindgen_malloc, c.__wbindgen_realloc), _ = y, s = c.cassidakernel_execute(this.__wbg_ptr, r, _);
        return n = s[0], o = s[1], k(s[0], s[1]);
      } finally {
        c.__wbindgen_free(n, o, 1);
      }
    }
    constructor() {
      const t = c.cassidakernel_new();
      return this.__wbg_ptr = t >>> 0, A.register(this, this.__wbg_ptr, this), this;
    }
  }
  Symbol.dispose && (h.prototype[Symbol.dispose] = h.prototype.free);
  function H(e, t) {
    throw new Error(k(e, t));
  }
  function K() {
    const e = c.__wbindgen_externrefs, t = e.grow(4);
    e.set(0, void 0), e.set(t + 0, void 0), e.set(t + 1, null), e.set(t + 2, true), e.set(t + 3, false);
  }
  const A = typeof FinalizationRegistry > "u" ? {
    register: () => {
    },
    unregister: () => {
    }
  } : new FinalizationRegistry((e) => c.__wbg_cassidakernel_free(e >>> 0, 1));
  function k(e, t) {
    return e = e >>> 0, P(e, t);
  }
  let l = null;
  function u() {
    return (l === null || l.byteLength === 0) && (l = new Uint8Array(c.memory.buffer)), l;
  }
  function j(e, t, n) {
    if (n === void 0) {
      const a = f.encode(e), i = t(a.length, 1) >>> 0;
      return u().subarray(i, i + a.length).set(a), y = a.length, i;
    }
    let o = e.length, r = t(o, 1) >>> 0;
    const _ = u();
    let s = 0;
    for (; s < o; s++) {
      const a = e.charCodeAt(s);
      if (a > 127) break;
      _[r + s] = a;
    }
    if (s !== o) {
      s !== 0 && (e = e.slice(s)), r = n(r, o, o = s + e.length * 3, 1) >>> 0;
      const a = u().subarray(r + s, r + o), i = f.encodeInto(e, a);
      s += i.written, r = n(r, o, s, 1) >>> 0;
    }
    return y = s, r;
  }
  let b = new TextDecoder("utf-8", {
    ignoreBOM: true,
    fatal: true
  });
  b.decode();
  const G = 2146435072;
  let g = 0;
  function P(e, t) {
    return g += t, g >= G && (b = new TextDecoder("utf-8", {
      ignoreBOM: true,
      fatal: true
    }), b.decode(), g = t), b.decode(u().subarray(e, e + t));
  }
  const f = new TextEncoder();
  "encodeInto" in f || (f.encodeInto = function(e, t) {
    const n = f.encode(e);
    return t.set(n), {
      read: e.length,
      written: n.length
    };
  });
  let y = 0, c;
  function X(e) {
    c = e;
  }
  URL = globalThis.URL;
  const d = await z({
    "./kernel_bg.js": {
      __wbg___wbindgen_throw_df03e93053e0f4bc: H,
      __wbindgen_init_externref_table: K
    }
  }, F), V = d.memory, Y = d.__wbg_cassidakernel_free, q = d.cassidakernel_execute, J = d.cassidakernel_new, $ = d.eval_input, Q = d.__wbindgen_externrefs, Z = d.__wbindgen_malloc, ee = d.__wbindgen_realloc, te = d.__wbindgen_free, v = d.__wbindgen_start, ne = Object.freeze(Object.defineProperty({
    __proto__: null,
    __wbg_cassidakernel_free: Y,
    __wbindgen_externrefs: Q,
    __wbindgen_free: te,
    __wbindgen_malloc: Z,
    __wbindgen_realloc: ee,
    __wbindgen_start: v,
    cassidakernel_execute: q,
    cassidakernel_new: J,
    eval_input: $,
    memory: V
  }, Symbol.toStringTag, {
    value: "Module"
  }));
  X(ne);
  v();
  function re() {
    const { subscribe: e, set: t, update: n } = I({
      data: {
        input_history: [],
        ouput_history: [],
        last_error: null
      },
      connected: false
    });
    let o;
    function r(a, i) {
      i.type == "err" ? a.last_error = i.content : i.type == "ok" && (a.last_error = null, a.ouput_history.push(i.content));
    }
    async function _() {
      return o = new h(), n((a) => ({
        ...a,
        connected: true
      })), {
        send: async (a) => {
          const i = await (o == null ? void 0 : o.execute(a)), T = typeof i == "string" ? JSON.parse(i) : i;
          n((w) => (w.data.input_history.push(a), r(w.data, T), {
            ...w,
            connected: true
          }));
        }
      };
    }
    let s = {
      send: () => {
      }
    };
    return typeof window < "u" && _().then((a) => s = a), {
      subscribe: e,
      send: (a) => s.send(a)
    };
  }
  se = re();
  ie = function(e) {
    se.send(e);
  };
})();
export {
  __tla,
  ie as a,
  se as b,
  ae as r,
  p as s
};
