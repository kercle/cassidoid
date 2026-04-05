import { c as u, a as o, f as c } from "../chunks/DY3I6B5O.js";
import { C as p, E, a as _, aQ as v, ay as y, h as i, aD as h, aE as l, K as f, J as m, e as g, a8 as T, w as C, x as F } from "../chunks/D3xIz7z5.js";
import { B as w } from "../chunks/BDkrSsJV.js";
import { s as R, __tla as __tla_0 } from "../chunks/CYfllkgq.js";
let O;
let __tla = Promise.all([
  (() => {
    try {
      return __tla_0;
    } catch {
    }
  })()
]).then(async () => {
  function b(s, n, ...t) {
    var r = new w(s);
    p(() => {
      const a = n() ?? null;
      r.ensure(a, a && ((e) => a(e, ...t)));
    }, E);
  }
  function x(s, n) {
    let t = null, r = i;
    var a;
    if (i) {
      t = g;
      for (var e = T(document.head); e !== null && (e.nodeType !== h || e.data !== s); ) e = l(e);
      if (e === null) f(false);
      else {
        var d = l(e);
        e.remove(), m(d);
      }
    }
    i || (a = document.head.appendChild(_()));
    try {
      p(() => n(a), v | y);
    } finally {
      r && (f(true), m(t));
    }
  }
  const D = "" + new URL("../assets/favicon.dfyh6Zu5.svg", import.meta.url).href;
  var N = c('<link rel="icon"/>');
  O = function(s, n) {
    var t = u();
    x("12qhfyh", (a) => {
      var e = N();
      F(() => R(e, "href", D)), o(a, e);
    });
    var r = C(t);
    b(r, () => n.children), o(s, t);
  };
});
export {
  __tla,
  O as component
};
