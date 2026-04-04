![TCAS](web/frontend/src/lib/assets/logo.svg)

# A computer algebra system written in Rust

This project emerged from my PDE solver where I experimented with WebGL and WebAssembly compiled from Rust. I took the basic implementation of the parser I implemented and made it a bit more robust. Furthermore, I added a (pretty naive) `BigInteger` and `Rational` implementation. For the AST, I implementated a first draft of a cannonicalization procedure, with constant folding and gathering terms with constant coefficients. From the AST, Latex output can be generated.

## Demo

The project comes with a library that provides WASM bindings, which allows one to embed the CAS in a website. To try it out, visit [the demo website](https://kercle.github.io/cassida/).

Example screenshot of the web app demonstrating plotting and differentiation:
![Demo Screenshot](assets/screenshot.png)

## Design goals

Since this is just a hobby project of mine, I want to keep the core modules -- that is `parser`, `symbolics` and `numbers` -- independent of external crates. These are the crates that do the actual heavy lifting.

Frontend-wise, I want the project to be platform agnostic. In particular, I would like the CAS to be compilable to WASM in order to embed it into static websites. The demo page is such an example, but in the future I imagine Cassida to be comparable to projects like [Algebrite](https://github.com/davidedc/Algebrite).

### Expressions and patterns

Everything in Cassida is an expression, represented internally as a tree where nodes either has a head and children (arguments) -- both of which are again sub-expressions -- or atoms, which are objects like numbers, string literals or symbols.

Expressions can be matched again patterns (which are expressions themselves) and rewritten based on given rules. In that sense, Cassida borrows many design-paradigms from systems like Mathematica.

Internally, the heard of this is the pattern matching engine. Each pattern is compiled into a byte-code program that can be run against expressions to be matched (subjects). This allows for efficient reuse of patterns for multiple subjects, in particular for builtins like pattern-based integration.

For efficiency, multiple patterns can also be merged into a single program, effectively building a trie for more efficient matching of large sets of rules.

The pattern matching engine currently supports:

- Matching literals (two trees match exactly).
- Matching wildcards (blanks) which can match any sub-tree.
- Matching variadics for sequences or multisets of arguments of nodes.
- Binding wildcards or variadics to variables.
- Limited set of predicates for wildcards and variadics.
- Conditions.
- Optionals, similar to Mathematica where e.g. `c_.*x` can match both `3 x` and `x` even thought the latter does not contain a multiplication (Optionals make heavy use of the `alternative` feature)
- Alternatives for matching different pattern in one execution. This is e.g. used in `optional` or when merging multiple rules into a single pattern program (e.g. rules for rules-based integration).

## Native Cassida language

The syntax of the expressions is Mathematica inspired, but deliberately steers away from certain concepts. The most drastic change is not using `{..}` for lists. Instead Cassida has `Tuple`. In its native syntax, a tuple is just multiple expressions seperated by a comma, unless the comma appears in a function call.

### Examples for tuples

- `f[x, y, z]` is not interpreted as a tuple, but rather `f` taking the three arguments `x`, `y` and `z`.
- `x, y` or equivalently `(x, y)` is represented as `Tuple[x, y]`.
- `f[(x, y), z]` is represented as `f[Tuple[x, y], z]`.

Tuples are also used as vector objects.

### Syntax

```
<identifier_or_function_call> ::= <identifier>
   | <identifier> "[" "]"
   | <identifier> "[" <parse_expression> { "," <expression_or_enclosed_block> }* "]"
<atom> ::= <number>
   | "(" <block> ")"
   | <identifier_or_function_call>
<pattern_test> ::= <atom> [ ("?") <atom> ]
<factorial> ::= <pattern_test> [ "!" ]
<power> ::= <factorial> { "^" <power> }
<signed_power> ::= { "+" | "-" }* power
<product> ::= <signed_power> { ("*"|"/") <signed_power> }*
   | <signed_power> { <signed_power> }*
<sum> ::= <product> { ("+"|"-") <product> }*
<cmp> ::= <sum> { ("<" | "<=" | "==" | ">=" | ">") <sum> }*
<and> ::= <cmp> { "&&" <cmp> }
<or> ::= <and> { "||" <and> }
<cond> ::= <or> { "/;" <or> }
<rule_delayed> ::= <cond> { ":>" <cond> }
<expression_or_enclosed_block> ::= <cond> | "{" <block> "}"
<expression_or_tuple> ::= <cond> { "," <cmd> }*
<block> ::= <expression_or_tuple> { ";" <expression_or_tuple> }*
   | "{" <block> "}"
```

### Language support for VS-Code

Cassida language support for VS-Code is provided by an extension in the `tools` directory.

## Builtin functionality

Builtins are modeled as a trait and registered in the kernel. The trait carries functions
for generating documentation of the builtins. To view the documentation, one can for example
open the demo page and run `Help[]`. More specific documentation can be displayed by
executing `Help[<HeadSymbol>]`, e.g. `Head[Integrate]`.

Note that Cassida also differs from Mathematica in the name of its builtins, e.g. instead of `Plus`,
it used `Add` and instead of `D` it uses `Diff`.

### Integration

Eventually I would like to implement parts of the Risch algorithm. Until I am ready to ready to
dive into this rabbit hole, the plan is to implement most of the rules included in the RUBI
dataset [1].

Some rules are already implemented, however since Cassida's syntax is not identical to Mathematica
and not all predicates are supported for the helpers RUBI builds upon, the rules need to be
adapted accordingly.

## Goals

- [x] Decouple `AstNode` from internal `Expr`
- [x] Basic Pattern matching
- [x] More advanced Pattern matching (commutative expressions with at most one sequence)
- [x] Make Expr into Merkle tree for quicker pattern matching
- [x] Remove annotation from Expr, as the intended use case was solved differently
- [ ] Allow more sequences in commutative patterns
- [ ] Support for more flexible predicate in pattern matching
- [ ] Add compile errors for patterns (e.g. unsupported predicates etc.)
- [x] Rework normalization to make it both more efficient and maintainable
- [x] Basic Rewrite engine
- [x] Basic univariate polynomial engine
- [ ] Basic multivariate polynomial engine
- [ ] Solve polynomial equations
- [x] Naive Differentiation
- [x] Naive Integration with simple rules
- [ ] More advanced integration
- [ ] Matrix/Vector operations
- [ ] Numerical evaluation and dynamical variables for dashboards
- [ ] Numerical integration and ODE/PDE solving
- [ ] Numerical solver for equations
- [x] Web frontend built with Svelte
- [x] Compile to Web-Assembly and bundle with frontend

## Logo

The logo features a [tortoise beetle](https://en.wikipedia.org/wiki/Cassidinae). The name is taken from the genus Cassida.

# References

1. Rich, A., Scheibe, P., Abbasi, N. (2018). *Rule-based integration: An extensive system of symbolic integration rules*. Journal of Open Source Software, 3(32), 1073. https://doi.org/10.21105/joss.01073
