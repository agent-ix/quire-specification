# Proposed native finite-state profile

Status: proposed; profile identifier `state-finite/0-draft`, language tag
`ix:native`, edition `0-draft`. These are private experimental identifiers.
The public language name is deferred until the first release review; a rename
requires an explicit source/profile migration, never reinterpretation of old
bytes. Spec artifacts declare model fields and operations, and the domain
package carries them. The fixtures pin this file's bytes.

## Admission and implementation status

“Admit” below is a proposed semantic rule, **not implemented support**. A compiler
must report separate parsed, linked, reference-evaluable and backend-supported
states per obligation. An unsupported projection cannot erase the obligation.

| Construct | Proposed state profile | Existing executable projection / extension gate |
|---|---|---|
| Boolean, closed enum, explicitly bounded integer, bounded text | Admit | Qualify each backend; IR representation alone proves no execution |
| Acyclic structural records | Admit, equality only when every field type admits it | Existing record representation; no object/record equality substitution |
| Optional absence | Admit `present` and guarded `value` | Preserve the exact path and observation in guard facts |
| Identity-bearing reference and optional reference | Admit with explicit closed population | Requires reviewed typed-model adapter and a new qualified projection; no recursive-record encoding |
| Current, pre, post observations | Admit with explicit context/operation anchor | One invocation; pre/post visibility and frames require validation |
| `not`, `and`, `or`, `implies` | Admit left-to-right short-circuit semantics | Preserve source implication nodes and activation correspondence |
| Value `if` and `let` | Admit selected branch and evaluate-once binding | No textual substitution or unproved lowering |
| Ordered, duplicate-preserving bounded sequences | Admit `size`, `forall`, `exists` | Empty universal true; empty existential false |
| Identity equality, exact finite parent reachability | Admit | Visited identities; fuel exhaustion stays incomplete |
| `+`, `-`, `*`, unary `-`, `div`, `rem` on integers | Admit only with static definedness | Explicit bounded operands/intermediates; no native wrapping |
| Rational division, Decimal, floats, implicit unit conversion | Recognize, refuse | Separate arithmetic profile and backend qualification |
| Set, Bag, OrderedSet, `collect`, flatten | Recognize, refuse | Specify output kind, duplicates and bounds before admission |
| User helper calls, recursion, subtype casts, tuples, map values | Recognize, refuse | Helper purity/totality/cost and subtyping rules need their own extension |
| Unbounded quantification, temporal/message/protocol expressions | Recognize, refuse | Separate named subject/profile; no global gate on finite state work |

Exact units are type annotations: like-unit comparison and addition/subtraction
are permitted. Multiplication, division and remainder initially require
dimensionless operands. No implicit scaling or conversion is admitted.

## Source grammar proposal

The source unit is UTF-8 with optional LF or CRLF line endings, no BOM, no NUL.
Identifiers are ASCII `[A-Za-z_][A-Za-z0-9_]*`; keywords cannot be identifiers.
Strings follow JSON string escape rules and contain only valid Unicode scalar
values. Decimal integer tokens have no leading zeros except `0`; a leading
minus is a unary operator. Whitespace and `//` comments separate tokens.
Token spans refer to original bytes, before formatting or newline conversion.

```ebnf
unit         = 'language', string, 'edition', string, ';',
               'profile', string, ';', import, { import }, clause, { clause } ;
import       = 'model', ident, '=', string, 'version', string,
               'digest', string, ';' ;
type-name    = ident, '::', ident ;
clause       = 'invariant', ident, 'on', type-name, 'at', 'current', block
             | ('pre' | 'post'), ident, 'on', type-name, '::', ident, block ;
block        = '{', expr, '}' ;
expr         = 'let', ident, '=', expr, 'in', expr
             | 'if', expr, 'then', expr, 'else', expr
             | implication ;
implication  = disjunction, [ 'implies', implication ] ;
disjunction  = conjunction, { 'or', conjunction } ;
conjunction  = comparison, { 'and', comparison } ;
comparison   = sum, [ ('=' | '!=' | '<' | '<=' | '>' | '>='), sum ] ;
sum          = product, { ('+' | '-'), product } ;
product      = unary, { ('*' | 'div' | 'rem'), unary } ;
unary        = ('not' | '-'), unary | postfix ;
postfix      = primary, { '.', ident } ;
primary      = 'true' | 'false' | 'self' | 'result' | integer | string | ident
             | type-name, '::', ident
             | '(', expr, ')'
             | ('present' | 'value' | 'deref' | 'size' | 'pre'), '(', expr, ')'
             | ('forall' | 'exists'), '(', ident, 'in', expr, ':', expr, ')'
             | 'reaches', '(', expr, ',', expr, ',', ident, ')' ;
```

The token `pre` is context-sensitive only between clause introducer and builtin
call; both are explicit above. `self`, `result` and declared operation parameter
names resolve from the anchor environment. Enum values use the resolved qualified
name `Model::Enum::Variant`. Display names in a domain package import
must resolve to exactly one stable declaration identity; duplicates are errors.
The grammar declares no model fields or operations: spec artifacts declare them,
and the imported domain package carries those declarations. A clause identifier maps to an authored requirement and
criterion in the source manifest; it is not a newly minted requirement identity.

Balanced reserved forms `helper`, `rec`, `set`, `bag`, `collect`, `cast`, `tuple`,
`always`, `eventually`, `send`, `receive` produce `unsupported_construct` at their
introducing token. They do not count as parsed supported expressions. Malformed
delimiters/tokens produce `invalid_syntax`; both cases preserve the entire source
clause. No silent recovery creates a successful partial unit.

## Static and evaluation rules

Write `G; F |- e : T` for expression typing under declarations G and exact
observation-qualified guard facts F. Successful linking requires a Boolean clause
root and definedness for every potentially evaluated subexpression.

- `present(o)` requires `Option<T>`. In its true branch add the fact that the
  exact option at its snapshot is present; `value(o)` requires that fact and
  yields T. `deref(r)` requires `Ref<T>` and resolves in r's snapshot. Facts
  about `pre(self.parent)` cannot justify `value(self.parent)` at post.
- `a and b` types b under facts from a being true; `a or b` under a being false;
  `a implies b` under a being true. Evaluate a once. Skip b for false/true/false
  respectively. The result is false/true/true in those skipped cases. Otherwise
  evaluate b. Record whether an implication's antecedent was evaluated and true;
  absence of a backend probe is unavailable coverage, not an inactive source.
- `if c then a else b` requires a Boolean c and equal branch types; type each
  branch under its condition facts and evaluate only the selected branch.
  `let x = a in b` evaluates a once and binds an immutable lexical value. Reject
  duplicate local names in the same scope; nested shadowing is initially refused.
- Comparisons require exactly equal operand types and units. A literal receives
  the unique expected declared scalar type if it fits; ambiguous literals refuse.
  Text compares exact Unicode scalar sequences, without normalization. Identity
  equality compares type identity plus object identity within a declared universe;
  equal fields do not imply equal objects. References across pre/post may compare
  identity only when both snapshots name that same universe.
- `self` and `deref(r)` denote identity-bearing objects of the context/target
  type; their `.field` access reads that object's immutable snapshot. They are
  not anonymous structural records. `reaches` accepts two such objects or typed
  references to the same object type/universe and follows the specified field.
- Integers are mathematical signed values within explicit model min/max bounds
  contained in signed 64-bit representation. Every intermediate result must fit
  its declared type. Prove bounds statically using declared ranges and guards;
  inability to establish definedness refuses compilation, even if samples fit.
  `div` truncates toward zero; `a rem b = a - (a div b) * b`, so `-7 rem 3 = -1`.
  Zero divisor and minimum-signed-value divided by `-1` require refusal unless
  excluded by bounds/guards. No host overflow, widening or floating arithmetic.
- A sequence's order and repeated values are significant. Its length obeys an
  authored maximum. Iterate `forall`/`exists` in order, short-circuit on the first
  false/true respectively. Missing sequence data is not an empty sequence.
- `reaches(start, target, parent)` follows the named optional reference field
  for one or more edges. Test each reached identity for target before suppressing
  repeated expansion with a visited set. A self-loop or cycle back to start
  therefore satisfies `reaches(start,start,parent)`. Termination follows from
  the validated finite universe, without imposing an implicit path-depth bound.
  A separate authored bounded-reachability operator would have different meaning.

Optional absence, explicit nullable value, unavailable observation and malformed
input remain distinct. This profile admits absence but refuses nullable model
fields, rather than collapsing null into absence. Snapshot validation precedes
predicate execution, including fields skipped by a short-circuit branch.

## State and resource boundary

Every runtime input supplies an immutable, typed finite population and explicit
completeness for each required universe. Duplicate identities, wrong types and
dangling targets in a claimed complete universe are invalid input. An incomplete
population or unavailable required observation yields incomplete evaluation,
with no Boolean value. A producer's completeness declaration is an assumption
about the supplied population; it does not prove deployment-wide completeness.

Invariant `self` refers to current. Preconditions use pre; postconditions use
post, with `pre(expr)` selecting the same invocation's pre snapshot. `result`
is post-only. Parameters are immutable invocation values. Creation/deletion are
explicit sets of identities, disjoint and equal to the population difference;
the model's frame/effect binding lists permitted changed fields and created or
deleted types. A missing frame or mismatched invocation refuses execution.
Dereferencing a deleted object in post is invalid; `pre` can still access it.

Parsing, linking and evaluation have separate caller-supplied resource ceilings.
Charge before work; count each evaluated syntax node and each expanded graph
identity at least once. Exact accounting/version must be fixed before executable
conformance vectors are frozen. Oversized input, exhausted fuel and cancellation
never produce false, empty collections or success. This packet does not yet
claim exact fuel-count parity across implementations.

A successful concrete evaluation is not enumeration of all inputs or a proof
of validity. Finite search, decidable validity and practical completion are
separate claims with separate method capabilities and budgets.

## Qualification obligations

The normative-candidate vector set must include healthy and violating parent
order, cycles, equal-valued distinct objects, incomplete universes, missing and
foreign references, stale imports, wrong snapshots, deleted objects, skipped
undefined branches, duplicate collection values, integer overflow and negative
remainder. Conditional/helper/collection extensions require independent expected
cases; reference/generated agreement alone cannot qualify their meaning.

A timed refund is refused by this state profile. Its charge identity, matching
amount, deadline, silent interval and untriggered case belong to an explicit
temporal/observation profile. A parser portfolio refers to identified source,
tool and package artifacts; its method, plan and result contracts are separate
from this expression grammar.
