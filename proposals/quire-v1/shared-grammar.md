# Shared native grammar fragment

Proposed L1 contribution for a new syntax edition; **not the completed composed
grammar or an accepted profile**. Historical `ix:native` / `0-draft` sources keep
their exact rules. The candidate uses `ix:native` / `1-draft` to make the change
explicit without renaming historical sources. Profile definition artifacts and
their exact revisions/digests must be accepted before this candidate executes.

This fragment defines common lexical rules, profile/model declarations, shared
expressions, state clauses and reusable predicates. Temporal/protocol productions
will join this same grammar with E/B's semantics; their bodies are not arbitrary
balanced text and no adapter may parse them with a second expression grammar.
L1 stays open until those productions and family examples are integrated.

## Lexical contract

Source is UTF-8 with original bytes retained. Reject an initial byte-order mark,
raw NUL, invalid UTF-8 and bare CR. Space, tab, LF and CRLF separate tokens;
`//` comments run to the line ending or EOF. There are no block comments,
indentation rules or automatic semicolons. Comments and whitespace do not alter
meaning but remain part of source identity. A comment marker inside a quoted
string is text. Skipped comments still consume source-byte budget.

Identifiers are case-sensitive ASCII `[A-Za-z_][A-Za-z0-9_]*`. No Unicode
normalization, case folding or name repair occurs. Quoted strings use JSON
escapes and valid Unicode scalar values; unpaired surrogates and unescaped
controls refuse. The original escaped spelling and decoded value are distinct.
Source/token/node spans use half-open byte offsets into the original document,
including CRLF and multibyte text. A decoded string offset is not a source span.

Use maximal token matching: `::`, `!=`, `<=` and `>=` are indivisible; a keyword
prefix inside a longer identifier is not a keyword. Keywords are selected by
edition. Adding a keyword in `1-draft` cannot steal an identifier admitted by
`0-draft`. One recognizer can retain raw word spelling and let the selected
edition classify it; no second handwritten token scanner is required.

An unsigned integer spelling is `0` or `[1-9][0-9]*`. A minus is a separate unary
token. Leading zeros and malformed exponent/string forms are syntax errors.
Decimal/exponent number forms may be recognized for an unsupported numeric-form
diagnostic; recognition does not admit floating point. Numeric value/range
checking follows syntax and the selected scalar domain.

Common reserved words for this candidate fragment are `language`, `edition`,
`profile`, `version`, `digest`, `model`, `using`, `predicate`, `Boolean`,
`invariant`, `pre`, `post`, `on`, `at`, `current`, `let`, `in`, `if`, `then`,
`else`, `implies`, `or`, `and`, `not`, `self`, `result`, `true`, `false`,
`present`, `value`, `deref`, `size`, `forall`, `exists`, `reaches`, `rational`,
`div`, `rem`, `mod`, `temporal` and `protocol`. The complete edition inventory
will add the reviewed family vocabulary before acceptance; this fragment is not
an extensible runtime keyword registry.

## Declarations and expressions

EBNF uses quoted terminals, comma concatenation, `|` alternatives, `{…}`
repetition and `[…]` optionality. `ident`, `string` and `uint` follow the lexical
contract. `EOF` requires the entire unit to be consumed.

```ebnf
common-unit  = header, profile, { profile }, { model },
               declaration, { declaration }, EOF ;
header       = 'language', string, 'edition', string, ';' ;
profile      = 'profile', ident, '=', string, 'version', string,
               'digest', string, ';' ;
model        = 'model', ident, '=', string, 'version', string,
               'digest', string, ';' ;
type-name    = ident, '::', ident ;
parameter-type = 'Boolean' | type-name ;
declaration  = predicate | state-clause ;
predicate    = 'predicate', ident, 'using', ident,
               '(', [ parameter, { ',', parameter } ], ')',
               ':', 'Boolean', block ;
parameter    = ident, ':', parameter-type ;
state-clause = 'invariant', ident, 'using', ident, 'on', type-name,
               'at', 'current', block
             | ('pre' | 'post'), ident, 'using', ident, 'on',
               type-name, '::', ident, block ;
block        = '{', expr, '}' ;
expr         = 'let', ident, '=', expr, 'in', expr
             | 'if', expr, 'then', expr, 'else', expr
             | implication ;
implication  = disjunction, [ 'implies', implication ] ;
disjunction  = conjunction, { 'or', conjunction } ;
conjunction  = comparison, { 'and', comparison } ;
comparison   = sum, [ ('=' | '!=' | '<' | '<=' | '>' | '>='), sum ] ;
sum          = product, { ('+' | '-'), product } ;
product      = unary, { ('*' | 'div' | 'rem' | 'mod' | '/'), unary } ;
unary        = ('not' | '-'), unary | postfix ;
postfix      = primary, { '.', ident } ;
primary      = 'true' | 'false' | 'self' | 'result' | uint | string
             | ident, [ '(', [ expr, { ',', expr } ], ')' ]
             | type-name, '::', ident
             | '(', expr, ')'
             | ('present' | 'value' | 'deref' | 'size' | 'pre'),
               '(', expr, ')'
             | ('forall' | 'exists'), '(', ident, 'in', expr, ':', expr, ')'
             | 'reaches', '(', expr, ',', expr, ',', ident, ')'
             | 'rational', '(', signed-int, ',', signed-int, ')' ;
signed-int   = [ '-' ], uint ;
```

`common-unit` is the explicitly limited test entry point for this fragment.
It is not the final composed-unit production. Adding temporal and protocol
declarations requires explicit grammar productions, not a catch-all body.
The common productions carry no clock, observation or protocol-control semantics.
Named imported model types are the authority for parameter domains. This draft
does not introduce inline record/field schemas or infer scalar bounds from data.
Any additional parameter type constructors require the reviewed shared type
contract; model identity and presence work remain with D.

`using` selects a declared profile alias for every predicate/state declaration;
there is no backend-derived default. Profile tuples include the definition name,
revision and byte digest, not just a convenient version label. Model aliases
resolve exact declared exports. A grammar parse does not prove those identities.

## Precedence and bindings

From lowest to highest: `let`/`if`; `implies` (right associative); `or`; `and`;
one comparison; addition/subtraction; product; unary `not`/negation; field access,
calls and parenthesized primaries. Addition/product chains associate left.
Comparisons do not chain: `a < b < c` is invalid syntax. Parentheses select an
explicit tree, which must still type-check. `not a = b` means `(not a) = b`;
authors write `not (a = b)` for negating a comparison.

Model and profile aliases are unique within their source unit; they do not
pollute another source unit's alias scope. Native declaration names are unique
within the linked package; ambiguous cross-unit declarations refuse. A unit
containing only model-independent Boolean predicates needs no dummy model import.
Any referenced model declaration still requires an exact explicit import. Predicate
parameters and lexical binders are unique in their enclosing declaration and
cannot shadow aliases, native declarations or another visible binding. Separate
predicate declarations may reuse a parameter name. A `let` binding is visible
only in its body, not its initializer. A quantifier binder is visible only in
its predicate, not its domain. Forward predicate references can resolve after
declaration collection; the complete call graph must be acyclic. No overload
resolution or dynamic dispatch is introduced.

A named predicate reads its explicit parameters and pure lexical values.
`self`, `result` and `pre(...)` are caller-side anchor operations, unavailable
as implicit ambient state inside a reusable predicate. Callers bind values at
the declared instant, then pass those values explicitly. This avoids giving
one reusable body different hidden current/pre/post meanings in each family.

Examples, under appropriately typed/admitted declarations:

| Source fragment | Expected syntax / binding |
| --- | --- |
| `predicate Positive using S (x: M::Amount): Boolean { x > 0 }` | One typed Boolean predicate declaration; M::Amount supplies bounds/units. |
| `Positive(self.amount)` | A call with a caller-anchored argument; historical no-call profiles still refuse admission. |
| `a implies b implies c` | `a implies (b implies c)` |
| `a + b * c` | `a + (b * c)` |
| `let x = a in x + x` | Initializer once; both reads refer to one immutable binding. |
| `forall(x in self.items: Positive(x.amount))` | Binder scoped to the predicate; sequence order/duplicates come from the selected semantics. |
| `let x = x in true` without an outer x | Unresolved initializer reference; not a recursive let. |
| `predicate P using S (x: M::Amount, x: M::Amount): Boolean { true }` | Duplicate parameter refusal. |
| `M::Order` as a value | Bare type/context name refusal; not an implicit population. |

## Exact rational literal proposal

`rational(n,d)` is an exact literal form with signed integer constants, not a
general runtime conversion or user function. Reject zero denominator. Normalize
sign to a positive denominator and divide numerator/denominator by their greatest
common divisor before the authored rational-domain checks; zero becomes `0/1`.
Do not use floating point, silently widen a domain, or infer an integer-to-rational
conversion. The unique expected imported rational type supplies normalized
numerator/denominator bounds; absence or ambiguity of that type refuses.

Examples: `rational(2,4)` and `rational(-2,-4)` normalize to `1/2`;
`rational(0,-7)` becomes `0/1`; `rational(1,0)` refuses. Distinct source spellings
keep their original source identities even where normalized values agree. A
raw component exceeding a final bound can normalize inside that bound; reject
only after normalization, subject to explicit parsing/arithmetic resource limits.
Resource exhaustion is incomplete execution/admission, not a false predicate.

`1/2`, `1 div 2`, `1 rem 2` and `1 mod 2` remain recognized expressions refused
by the carried-forward v1 arithmetic admission. They do not spell rational
literals. This edition's ability to parse an operator does not enable it in the
first state profile or alter old `0-draft` division/remainder semantics.

## Outstanding integration

Before accepting L1: integrate temporal/protocol productions and examples,
reconcile the complete keyword/type/builtin/query catalog, agree family scopes
and anchors, supply exact profile definition artifacts, and run the selected
baseline review. This document neither closes those requirements nor qualifies
the current compiler against the new syntax.
