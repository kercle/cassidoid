import { h as v, aR as x, aS as E, N as M, aT as L, aU as C, aV as N, aW as U, aX as W } from "./BfX1JZpI.js";
let ie, se, ae, m;
let __tla = (async () => {
  const I = /* @__PURE__ */ Symbol("is custom element"), O = /* @__PURE__ */ Symbol("is html"), R = E ? "link" : "LINK";
  ae = function(e) {
    if (v) {
      var t = false, n = () => {
        if (!t) {
          if (t = true, e.hasAttribute("value")) {
            var o = e.value;
            m(e, "value", null), e.value = o;
          }
          if (e.hasAttribute("checked")) {
            var r = e.checked;
            m(e, "checked", null), e.checked = r;
          }
        }
      };
      e.__on_r = n, M(n), L();
    }
  };
  m = function(e, t, n, o) {
    var r = B(e);
    v && (r[t] = e.getAttribute(t), t === "src" || t === "srcset" || t === "href" && e.nodeName === R) || r[t] !== (r[t] = n) && (t === "loading" && (e[C] = n), n == null ? e.removeAttribute(t) : typeof n != "string" && D(e).includes(t) ? e[t] = n : e.setAttribute(t, n));
  };
  function B(e) {
    return e.__attributes ?? (e.__attributes = {
      [I]: e.nodeName.includes("-"),
      [O]: e.namespaceURI === x
    });
  }
  var A = /* @__PURE__ */ new Map();
  function D(e) {
    var t = e.getAttribute("is") || e.nodeName, n = A.get(t);
    if (n) return n;
    A.set(t, n = []);
    for (var o, r = e, i = Element.prototype; i !== r; ) {
      o = U(r);
      for (var s in o) o[s].set && n.push(s);
      r = N(r);
    }
    return n;
  }
  const F = "" + new URL("../assets/kernel_bg.BAO4Cg9N.wasm", import.meta.url).href, z = async (e = {}, t) => {
    let n;
    if (t.startsWith("data:")) {
      const o = t.replace(/^data:.*?base64,/, "");
      let r;
      if (typeof Buffer == "function" && typeof Buffer.from == "function") r = Buffer.from(o, "base64");
      else if (typeof atob == "function") {
        const i = atob(o);
        r = new Uint8Array(i.length);
        for (let s = 0; s < i.length; s++) r[s] = i.charCodeAt(s);
      } else throw new Error("Cannot decode base64-encoded data URL");
      n = await WebAssembly.instantiate(r, e);
    } else {
      const o = await fetch(t), r = o.headers.get("Content-Type") || "";
      if ("instantiateStreaming" in WebAssembly && r.startsWith("application/wasm")) n = await WebAssembly.instantiateStreaming(o, e);
      else {
        const i = await o.arrayBuffer();
        n = await WebAssembly.instantiate(i, e);
      }
    }
    return n.instance.exports;
  };
  class h {
    __destroy_into_raw() {
      const t = this.__wbg_ptr;
      return this.__wbg_ptr = 0, S.unregister(this), t;
    }
    free() {
      const t = this.__destroy_into_raw();
      c.__wbg_cassidakernel_free(t, 0);
    }
    execute(t) {
      let n, o;
      try {
        const r = G(t, c.__wbindgen_malloc, c.__wbindgen_realloc), i = y, s = c.cassidakernel_execute(this.__wbg_ptr, r, i);
        return n = s[0], o = s[1], T(s[0], s[1]);
      } finally {
        c.__wbindgen_free(n, o, 1);
      }
    }
    constructor() {
      const t = c.cassidakernel_new();
      return this.__wbg_ptr = t >>> 0, S.register(this, this.__wbg_ptr, this), this;
    }
  }
  Symbol.dispose && (h.prototype[Symbol.dispose] = h.prototype.free);
  function K(e, t) {
    throw new Error(T(e, t));
  }
  function j() {
    const e = c.__wbindgen_externrefs, t = e.grow(4);
    e.set(0, void 0), e.set(t + 0, void 0), e.set(t + 1, null), e.set(t + 2, true), e.set(t + 3, false);
  }
  const S = typeof FinalizationRegistry > "u" ? {
    register: () => {
    },
    unregister: () => {
    }
  } : new FinalizationRegistry((e) => c.__wbg_cassidakernel_free(e >>> 0, 1));
  function T(e, t) {
    return e = e >>> 0, X(e, t);
  }
  let u = null;
  function b() {
    return (u === null || u.byteLength === 0) && (u = new Uint8Array(c.memory.buffer)), u;
  }
  function G(e, t, n) {
    if (n === void 0) {
      const a = l.encode(e), _ = t(a.length, 1) >>> 0;
      return b().subarray(_, _ + a.length).set(a), y = a.length, _;
    }
    let o = e.length, r = t(o, 1) >>> 0;
    const i = b();
    let s = 0;
    for (; s < o; s++) {
      const a = e.charCodeAt(s);
      if (a > 127) break;
      i[r + s] = a;
    }
    if (s !== o) {
      s !== 0 && (e = e.slice(s)), r = n(r, o, o = s + e.length * 3, 1) >>> 0;
      const a = b().subarray(r + s, r + o), _ = l.encodeInto(e, a);
      s += _.written, r = n(r, o, s, 1) >>> 0;
    }
    return y = s, r;
  }
  let w = new TextDecoder("utf-8", {
    ignoreBOM: true,
    fatal: true
  });
  w.decode();
  const H = 2146435072;
  let g = 0;
  function X(e, t) {
    return g += t, g >= H && (w = new TextDecoder("utf-8", {
      ignoreBOM: true,
      fatal: true
    }), w.decode(), g = t), w.decode(b().subarray(e, e + t));
  }
  const l = new TextEncoder();
  "encodeInto" in l || (l.encodeInto = function(e, t) {
    const n = l.encode(e);
    return t.set(n), {
      read: e.length,
      written: n.length
    };
  });
  let y = 0, c;
  function P(e) {
    c = e;
  }
  URL = globalThis.URL;
  const d = await z({
    "./kernel_bg.js": {
      __wbg___wbindgen_throw_df03e93053e0f4bc: K,
      __wbindgen_init_externref_table: j
    }
  }, F), V = d.memory, Y = d.__wbg_cassidakernel_free, q = d.cassidakernel_execute, J = d.cassidakernel_new, $ = d.eval_input, Q = d.__wbindgen_externrefs, Z = d.__wbindgen_malloc, ee = d.__wbindgen_realloc, te = d.__wbindgen_free, k = d.__wbindgen_start, ne = Object.freeze(Object.defineProperty({
    __proto__: null,
    __wbg_cassidakernel_free: Y,
    __wbindgen_externrefs: Q,
    __wbindgen_free: te,
    __wbindgen_malloc: Z,
    __wbindgen_realloc: ee,
    __wbindgen_start: k,
    cassidakernel_execute: q,
    cassidakernel_new: J,
    eval_input: $,
    memory: V
  }, Symbol.toStringTag, {
    value: "Module"
  }));
  P(ne);
  k();
  function re() {
    const { subscribe: e, set: t, update: n } = W({
      data: {
        history: []
      },
      connected: false
    });
    let o;
    function r(a, _) {
      let f = a.history.pop();
      f !== void 0 && f.type != "err" && a.history.push(f), a.history.push(_);
    }
    async function i() {
      return o = new h(), n((a) => ({
        ...a,
        connected: true
      })), {
        send: async (a) => {
          const _ = await (o == null ? void 0 : o.execute(a)), f = typeof _ == "string" ? JSON.parse(_) : _;
          n((p) => (r(p.data, f), {
            ...p,
            connected: true
          }));
        }
      };
    }
    let s = {
      send: () => {
      }
    };
    return typeof window < "u" && i().then((a) => s = a), {
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
  m as s
};
