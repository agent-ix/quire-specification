# Shared native grammar draft

Proposed L1 contribution for a new syntax edition; **not the completed composed
grammar or an accepted profile**. Historical `ix:native` / `0-draft` sources keep
their exact rules. The candidate uses `ix:native` / `1-draft` to make the change
explicit without renaming historical sources. Profile definition artifacts and
their exact revisions/digests must be accepted before this candidate executes.

This draft defines common lexical rules, profile/model declarations, shared
expressions, state clauses, reusable predicates and native temporal clauses.
Temporal meaning comes from FR-090–095, contributed at E's `1f107dd` revision.
Protocol productions will join this same grammar with B's contract; their bodies
are not arbitrary balanced text and no adapter may parse them with a second
expression grammar. L1 stays open until those productions and the family
bindings/examples are integrated.

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
`present`, `value`, `deref`, `size`, `contains`, `forall`, `exists`, `filter`,
`map`, `count`, `sum`, `reaches`, `rational`,
`div`, `rem`, `mod`, `temporal`, `protocol`, `over`, `clock`, `origin`, `each`,
`when`, `capture`, `holds`, `eventually`, `always`, `until`, `release`, `once`,
`historically`, `since` and `triggered`. The complete edition inventory
will add the reviewed family vocabulary before acceptance; this fragment is not
an extensible runtime keyword registry.

## Declarations and expressions

EBNF uses quoted terminals, comma concatenation, `|` alternatives, `{…}`
repetition and `[…]` optionality. `ident`, `string` and `uint` follow the lexical
contract. `EOF` requires the entire unit to be consumed.

```ebnf
state-temporal-unit = header, profile, { profile }, { model },
               declaration, { declaration }, EOF ;
header       = 'language', string, 'edition', string, ';' ;
profile      = 'profile', ident, '=', string, 'version', string,
               'digest', string, ';' ;
model        = 'model', ident, '=', string, 'version', string,
               'digest', string, ';' ;
type-name    = ident, '::', ident ;
parameter-type = 'Boolean' | type-name ;
declaration  = predicate | state-clause | temporal-clause ;
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
             | ('present' | 'value' | 'deref' | 'pre'),
               '(', expr, ')'
             | 'size', [ '<', type-name, '>' ], '(', expr, ')'
             | 'contains', '(', expr, ',', expr, ')'
             | ('forall' | 'exists' | 'filter' | 'map'),
               '(', ident, 'in', expr, ':', expr, ')'
             | ('count' | 'sum'), '<', type-name, '>',
               '(', ident, 'in', expr, ':', expr, ')'
             | 'reaches', '(', expr, ',', expr, ',', ident, ')'
             | 'rational', '(', signed-int, ',', signed-int, ')' ;
signed-int   = [ '-' ], uint ;

temporal-clause = 'temporal', ident, 'using', ident,
                 'over', '(', parameter, ')', 'clock', string,
                 activation, '{', { capture }, temporal-expr, '}' ;
activation   = 'on', 'origin'
             | 'on', 'each', '(', parameter, ')',
               [ 'when', '(', expr, ')' ] ;
capture      = 'capture', ident, ':', parameter-type, '=', expr, ';' ;
interval     = '[', uint, ',', uint, ']' ;
temporal-expr = temporal-implication ;
temporal-implication = temporal-disjunction,
                       [ 'implies', temporal-implication ] ;
temporal-disjunction = temporal-conjunction,
                       { 'or', temporal-conjunction } ;
temporal-conjunction = temporal-relation,
                       { 'and', temporal-relation } ;
temporal-relation = temporal-unary,
                    [ ('until' | 'release' | 'since' | 'triggered'),
                      interval, temporal-unary ] ;
temporal-unary = 'not', temporal-unary
              | ('eventually' | 'always' | 'once' | 'historically'),
                interval, temporal-unary
              | temporal-primary ;
temporal-primary = 'true' | 'false' | '(', temporal-expr, ')'
                 | 'holds', '(', expr, ')' ;
```

`state-temporal-unit` is the explicitly limited entry point for this draft.
The earlier common-only fragment is subsumed; neither entry point has an adopted
profile identity. This is not the final composed-unit production. Adding protocol
declarations requires explicit grammar productions, not a catch-all body.
Common value-expression productions carry no temporal or protocol-control
semantics; the temporal productions have the interpretation specified below.
The [state contract](state-contract.md) specifies ordered queries, exact
aggregation, numeric domains and operation-anchor correspondence. Type arguments
after the reserved query names are type positions, not comparisons; the normal
comparison grammar applies inside their argument expressions.
Named imported model types are the authority for parameter domains. This draft
does not introduce inline record/field schemas or infer scalar bounds from data.
Any additional parameter type constructors require the reviewed shared type
contract; model identity and presence work remain with D.

`using` selects a declared profile alias for every native declaration;
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

Temporal declarations additionally follow the environment scopes below. Their
current-input and activation binders are distinct bindings even when their
model types or runtime values agree. The same no-shadowing rule applies to
binders and captures; a separate declaration may reuse the names.

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

## Native temporal composition

The temporal operators use [FR-090](../../spec/functional/FR-090-select-temporal-profile-and-clock.md)
through [FR-095](../../spec/functional/FR-095-preserve-native-tl-correspondence.md).
The syntax does not select a different clock or closed-boundary convention to
fit a backend. Each interval has inclusive integer bounds `0 <= a <= b`; its
unit is one tick of the explicitly selected clock interpretation. Negative,
unbounded or fractional interval spellings are not interval syntax. Reversed
bounds parse but refuse semantic admission. Large valid bounds still require
resource admission. A rational sample period belongs to the clock binding,
not to an implicit decimal interval conversion.

`holds(expr)` is one typed native Boolean leaf at the temporal evaluation
instant. Its argument uses the **same** `expr` production, name resolver, type
checker and declared-domain definedness rules as a state/predicate expression.
The leaf retains its expression, declaration references, source spans and
environment; it is not an authored TL proposition identifier or serialized
formula string. Ordinary value expressions cannot contain a temporal operator,
and a temporal formula cannot enter a predicate's Boolean argument position.

The outer formula has implication (right associative), `or`, `and`, one binary
temporal relation, unary operators, then primaries in increasing precedence.
Binary temporal relations do not chain without parentheses. For example,
`eventually[0,3] holds(p) and holds(q)` has outer `and`; authors group
`eventually[0,3] (holds(p) and holds(q))` to put both leaves in its operand.
`holds(p) until[0,3] holds(q) until[0,4] holds(r)` is invalid syntax.

An outer `true` or `false` is a temporal constant. `holds(true)` remains an
atomic native valuation, true at an admitted instant and subject to atomic
false extension outside the authoritative execution in the selected profile.
Thus on a complete one-position event trace `always[0,1] true` is true and
`always[0,1] holds(true)` is false under the false-extension profile. A compiler
must not erase that distinction by folding the leaf into a temporal constant.
Under a complete finite-window profile both formulas quantify only its eligible
instants. Missing observations never become false extension.

### Input and activation scopes

`over (sample: M::Sample)` declares the one typed valuation-input slot at each
admitted instant. M::Sample is an existing model export, not a native schema.
Its record may contain declared references, optional observations and finite
population views. The F-owned binding supplies values and the exact subject,
instant, snapshot/window membership and completeness they depend on; the
compiler does not query an ambient model or construct missing records. The
same source slot can receive different values at different instants, but its
type and required binding contract are static package dependencies.

`clock "refund-clock"` names a nonempty, case-sensitive clock role in the
clause's binding requirements. It is not a system clock lookup. A request must
resolve it to exactly one F-owned clock binding compatible with the selected
temporal profile, preserving its exact identity, authority, unit and applicable
epoch/period. There is no default clock or coercion from event count to time.
That concrete clock binding remains execution input; it does not mutate the
linked source package. The authored role and required interpretation are static.

`on each (failed: M::FailedFulfillment)` selects each bound semantic trigger of
the declared model type for the assessed workflow/scope. An optional `when`
uses a total Boolean expression in the trigger environment to select activation.
A false guard does not activate an obligation. An unavailable guard cannot be
treated as false. F supplies trigger identity/correlation; equal payloads,
timestamps or transport receipts are insufficient. Duplicate receipts for the
same semantic trigger retain one activation, as specified by FR-093.

`on origin` creates the whole-execution obligation at its authoritative origin.
It does not treat the first supplied row as that origin. This form has no trigger
binder. Any origin capture requiring a current input needs a bound input at that
origin; temporal constant-only formulas do not manufacture a sample to read.

Capture initializers run once in the activation environment, in source order.
They may read the current input at the activation anchor, the trigger where
declared, and earlier captures. They cannot read themselves, later captures or
another obligation's environment. The `when` guard precedes capture evaluation
and cannot read captures. Its same-value presence facts may justify guarded
capture reads at that activation; they do not justify future observations.
Captures have explicit declared types, retain value/provenance immutably and
do not retag reference snapshots. Failed input binding or definedness cannot
create a healthy activation.

Within the temporal formula, only the current-input binder and declared captures
are value bindings. The trigger binder is intentionally unavailable there;
authors capture any trigger values needed later. `self`, `result` and `pre(...)`
are unavailable ambient state operations in this environment. Explicit qualified
model values/references or immutable captured values can be passed to shared
predicates; invocation observations still require their exact state bindings.
This preserves FR-034 without giving a predicate hidden temporal state.

### Source examples and distinguishing cases

These are declaration fragments, not whole executable fixture files. Their
enclosing unit must import exact model/profile definitions before admission.
For this synthetic example M exports an explicitly bounded Amount 1..20,
PaymentId, FailedFulfillment (charged Boolean, paymentId, amount) and RefundSample
with an authored optional refund whose paymentId and amount use those same types.
The optional channel has one declared absence meaning. S admits the shared
predicate rules; T selects timestamped-event finite-window meaning. The binding
declares a seconds-based exact clock role `refund-clock` and keeps each order,
payment effect and refund relationship explicit.

```quire
predicate RefundMatches using S
    (sample: M::RefundSample, payment: M::PaymentId, amount: M::Amount): Boolean {
  present(sample.refund) and
  value(sample.refund).paymentId = payment and
  value(sample.refund).amount = amount
}

temporal RefundDue using T
    over (sample: M::RefundSample) clock "refund-clock"
    on each (failed: M::FailedFulfillment) when (failed.charged) {
  capture payment: M::PaymentId = failed.paymentId;
  capture amount: M::Amount = failed.amount;
  eventually[0,30] holds(RefundMatches(sample, payment, amount))
}
```

Two distinct charged triggers with equal amounts create separate obligations.
A refund at 30 seconds with the correct effect/relationship participates; a
refund for another order does not satisfy the first obligation. Complete
progress through 30 with no matching refund settles the bounded miss; missing
external-provider observations remain incomplete. A later mutable amount never
changes the capture. These expected outcomes depend on E/F's premises, not just
successful parsing of the fragment.

| Change to the fragment | Required result |
| --- | --- |
| Use `failed.amount` directly in the temporal leaf | Unresolved activation-only binder; capture explicitly. |
| Capture amount from the current input, then change a later input | Preserve the activation value and its provenance. |
| Reference a later capture in an earlier initializer | Located lexical-scope refusal. |
| Call `RefundMatches` with `eventually[0,30] holds(true)` as an argument | Reject temporal syntax in a value-expression position; no status-to-Boolean coercion. |
| Select a clock binding from another order/scope or of event-position kind | Refuse the binding/profile mismatch even if interval numerals agree. |
| Lower this timestamped formula through the current index-based TL profiles | Typed unsupported correspondence; retain the valid native subject. |
| Put a sequence query in `holds(...)` whose root is not Boolean | Type refusal; queries still use the common expression grammar. |
| Bind a population view with unknown required membership | Incomplete input; neither a missing sequence nor missing members become an empty aggregate. |

Selected finite snapshot/window aggregate rules use the same common `filter`,
`count` and `sum` expressions over explicitly bound model fields. The source
requires the declared view; D/F supply its membership, window and completeness
contract. No `allInstances()` or implicit history query is introduced.

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

Before accepting L1: integrate protocol productions and examples,
reconcile the complete keyword/type catalog, agree model/observation binding slots
and anchors, supply exact profile definition artifacts, and run the selected
baseline review. This document neither closes those requirements nor qualifies
the current compiler against the new syntax.
