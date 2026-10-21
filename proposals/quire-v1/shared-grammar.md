# Shared native grammar

This document defines the composed `ix:native` / `1-draft` edition grammar and
the complete-V1 facet additions. Historical `ix:native` / `0-draft` sources keep
their exact rules and bytes. Selecting the complete facet bundle is explicit;
it never relabels a historical package or makes an unselected construct valid.

This grammar defines one composed entry point for common expressions, state,
temporal and protocol/choreography declarations. Temporal meaning comes from
FR-090–095; protocol meaning comes from FR-050–061 and the
[protocol contract](protocol-contract.md). No family body is arbitrary balanced
text and no adapter may parse expressions with a second grammar. The behavioral
and object requirements own meaning; this document owns source recognition,
precedence and binding shape.

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

`ident` excludes that edition's reserved words in native declaration/binder
positions. A `member-name` accepts the same ASCII word spelling including
reserved words in a syntactically qualified model/member position. Thus
`view.exhausted`, `M::result` and `M::Payment::retry` preserve model-owned names;
they do not declare native binders named `exhausted`, `result` or `retry`.
The parser uses token spelling in those positions without rescanning source or
changing the token's original span. No punctuation, Unicode normalization or
implicit model-name repair is admitted by this contextual rule.

An unsigned integer spelling is `0` or `[1-9][0-9]*`. A minus is a separate unary
token. `..` is one two-character punctuation token, scanned by longest match, and
only the refinement `seq` form uses it (FR-376); `.` stays member selection, so
`0 .. 1`, `0..1` and `a.b` scan as `uint` `..` `uint` and `ident` `.` `member-name`. Leading zeros and malformed exponent/string forms are syntax errors.
Decimal/exponent number forms may be recognized for an unsupported numeric-form
diagnostic; recognition does not admit floating point. Numeric value/range
checking follows syntax and the selected scalar domain.

Common reserved words for this candidate fragment are `language`, `edition`,
`profile`, `digest`, `model`, `using`, `predicate`, `Boolean`,
`invariant`, `pre`, `post`, `on`, `at`, `current`, `let`, `in`, `if`, `then`,
`else`, `implies`, `or`, `and`, `not`, `self`, `result`, `true`, `false`,
`present`, `value`, `deref`, `size`, `contains`, `forall`, `exists`, `filter`,
`map`, `count`, `sum`, `reaches`, `rational`,
`div`, `rem`, `mod`, `temporal`, `protocol`, `over`, `clock`, `origin`, `each`,
`when`, `capture`, `holds`, `eventually`, `always`, `until`, `release`, `once`,
`historically`, `since`, `triggered`, `role`, `relationship`, `channel`, `from`,
`to`, `carries`, `ordering`, `unordered`, `fifo`, `by`, `delivery`, `requires`,
`compensate`, `for`, `as`, `activate`, `first`, `within`, `attempts`, `of`,
`retry`, `commit`, `never`, `recover`, `run`, `sequence`, `choice`, `visible`,
`case`, `parallel`, `branch`, `join`, `all`, `repeat`, `max`, `while`,
`exhausted`, `await`, `after`, `match`, `timeout`, `send`, `via`, `receive`,
`attempt`, `contracts`, `effect`, `event`, `related`, `check` and `finish`.
This candidate inventory is edition-owned, not an extensible runtime registry.

The complete-V1 facets additionally reserve `import`, `dimension`, `unit`,
`ordered`, `enum`, `record`, `tuple`, `union`, `type`, `function`, `pure`, `decreases`,
`Integer`, `Int`, `Rational`, `Decimal`, `Float32`, `Float64`, `Text`, `Option`,
`Sequence`, `Set`, `Bag`, `OrderedSet`, `Reference`, `Quantity`, `Population`, `null`, `none`, `decimal`,
`float32`, `float64`, `bits`, `convert`, `allInstances`, `collect`, `flatMap`,
`flatten`, `fold`, `reduce`, `identity`, `toward-zero`, `toward-positive`,
`toward-negative`, `nearest-even`, `nearest-away`, `unicode-scalars`, `nfc`,
`nfd`, `nfkc`, `nfkd`, `binary-utf8`, `lifetime`, `workflow`, `scope`,
`capacity`, `symbolic`, `overflow`, `reject`, `block`, `loss`, `at-most-once`,
`at-least-once`, `exactly-once-premise`, `any`, `quorum`, `outstanding`,
`continue`, `cancel`, `invariant`, `variant`, `relation`, `hyper`, `trace`,
`bounded`, `hybrid`, `mode`, `flow`, `transition`, `reset`, `synthesis`,
`grammar`, `satisfies`, `verify`, `claim`, `method`, `domain`, `depends`,
`bound`, `lookup`, `absent`, `body`, `abstraction`, `terminal`, `scheduling`,
`adversarial`, `memory`, `fence`, `fork`, `node` and `stutter`. In a
`protocol-step-source`, `join` and `activate` keep their common reservation
and name a step kind. The words `sc`, `tso`, `ra`, `relaxed`, `acquire`,
`acq_rel`, `seq_cst`, `cend`, `duplicate`, `lose`, `spawn` and `retire` are
matched as keyword spellings only in the `memory-clause`, `memory-order`,
`fence-order` and `protocol-step-source` positions and remain identifiers
elsewhere; `release` keeps its common reservation, and `ordering` is reused
from channel declarations. The words `undefined`,
`empty` and `refused` are matched as keyword spellings only in the lookup
absence position and remain identifiers elsewhere. The words `population`,
`operation` and `receiver` are matched as keyword spellings only in their
`abstraction-decl` positions and remain identifiers elsewhere. Hyphenated tokens are matched as single keyword spellings only in
their grammar positions; they are never identifier normalization aliases.

## Declarations and expressions

EBNF uses quoted terminals, comma concatenation, `|` alternatives, `{…}`
repetition and `[…]` optionality. `ident`, `member-name`, `string` and `uint`
follow the lexical contract. `EOF` requires the entire unit to be consumed.

```ebnf
composed-unit = header, profile, { profile }, { model },
               declaration, { declaration }, EOF ;
header       = 'language', string, 'edition', string, ';' ;
profile      = 'profile', ident, '=', string, ';' ;
model        = 'model', ident, '=', string, 'digest', string, ';' ;
model-name   = ident, '::', member-name ;
type-name    = model-name ;
operation-name = type-name, '::', member-name ;
parameter-type = 'Boolean' | type-name ;
declaration  = predicate | state-clause | temporal-clause | protocol-clause ;
predicate    = 'predicate', ident, 'using', ident,
               '(', [ parameter, { ',', parameter } ], ')',
               ':', 'Boolean', block ;
parameter    = ident, ':', parameter-type ;
state-clause = 'invariant', ident, 'using', ident, 'on', type-name,
               'at', 'current', block
             | ('pre' | 'post'), ident, 'using', ident, 'on',
               operation-name, block ;
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
postfix      = primary, { '.', member-name } ;
primary      = 'true' | 'false' | 'self' | 'result' | uint | string
             | ident, [ '(', [ expr, { ',', expr } ], ')' ]
             | type-name, '::', member-name
             | '(', expr, ')'
             | ('present' | 'value' | 'deref' | 'pre'),
               '(', expr, ')'
             | 'size', [ '<', type-name, '>' ], '(', expr, ')'
             | 'contains', '(', expr, ',', expr, ')'
             | ('forall' | 'exists' | 'filter' | 'map'),
               '(', ident, 'in', expr, ':', expr, ')'
             | ('count' | 'sum'), '<', type-name, '>',
               '(', ident, 'in', expr, ':', expr, ')'
             | 'reaches', '(', expr, ',', expr, ',', member-name, ')'
             | 'rational', '(', signed-int, ',', signed-int, ')' ;
signed-int   = [ '-' ], uint ;

temporal-clause = 'temporal', ident, 'using', ident,
                 'over', '(', parameter, ')', [ 'clock', string ],
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

protocol-clause = 'protocol', ident, 'using', ident,
                  'over', '(', parameter, ')', activation, '{',
                  { capture }, role, { role }, { relationship }, { channel },
                  { protocol-requirement | compensation },
                  'run', control, finish, '}' ;
role         = 'role', ident, 'on', model-name, ';' ;
relationship = 'relationship', ident, '=', model-name, ';' ;
channel      = 'channel', ident, 'from', ident, 'to', ident,
               'carries', type-name, 'ordering', ordering,
               'delivery', delivery-policy,
               'capacity', capacity,
               'overflow', overflow-policy, ';' ;
ordering     = 'unordered'
             | 'fifo', 'by', '(', parameter, ')', block ;
protocol-requirement = 'requires', 'temporal', ident, ';' ;
node-ref     = ident, { '::', ident } ;
compensation = 'compensate', ident, 'for', node-ref,
               'as', '(', parameter, ')', 'by', ident,
               'on', operation-name, 'using', ident, 'clock', string, '{',
               { capture }, 'activate', 'first', '(', parameter, ')',
               'when', block, '{', { capture }, '}',
               'within', interval, ';', 'attempts', uint, 'of', type-name, ';',
               'retry', '(', parameter, ',', parameter, ')', block, ';',
               'commit', (node-ref | 'never'), ';',
               'recover', '(', parameter, ')', block, ';', '}' ;
control      = sequence | choice | parallel | repetition | await-control
             | event-node | check | commit ;
sequence     = 'sequence', ident, '{', { control }, '}' ;
visibility   = 'visible', '(', [ expr, { ',', expr } ], ')' ;
choice       = 'choice', ident, 'by', ident, visibility, '{',
               case, case, { case }, '}' ;
case         = 'case', ident, 'when', block, control ;
parallel     = 'parallel', ident, '{', branch, branch, { branch }, '}',
               'join', 'all', '[', ident, { ',', ident }, ']', ';' ;
branch       = 'branch', ident, control ;
repetition   = 'repeat', ident, 'by', ident, visibility,
               ( 'max', uint, 'while', block, control, 'exhausted', control
               | 'while', block, control ) ;
await-control = 'await', ident, 'after', node-ref,
                'using', ident, 'clock', string, 'within', interval,
                'match', event-node, 'then', control, 'timeout', control ;
related      = 'related', 'by', ident, '(', expr, ',', expr, ')' ;
event-node   = 'send', ident, 'via', ident,
               'as', '(', parameter, ')', { related }, block, ';'
             | 'receive', ident, 'via', ident, 'of', node-ref,
               'as', '(', parameter, ')', { related }, block, ';'
             | 'attempt', ident, 'by', ident, 'on', operation-name,
               'contracts', '[', [ ident, { ',', ident } ], ']',
               'as', '(', parameter, ')', { related }, block, ';'
             | 'effect', ident, 'of', node-ref,
               'as', '(', parameter, ')', { related }, block, ';'
             | 'event', ident, 'by', ident, [ 'for', node-ref ],
               'as', '(', parameter, ')', { related }, block, ';' ;
check        = 'check', ident, 'using', ident, block, ';' ;
commit       = 'commit', ident, 'by', ident,
               'as', '(', parameter, ')', block, ';' ;
finish       = 'finish', ident, 'as', '(', parameter, ')', block, ';' ;
```

`composed-unit` subsumes the earlier common-only and state/temporal drafting
entry points. None has an adopted profile identity. All three families are now
syntactically represented; grammar completeness does not establish agreement of
their model, observation or result contracts, or an executable implementation.
Common value-expression productions carry no temporal or protocol-control
semantics; the temporal productions have the interpretation specified below.
The [choreography surface contract](choreography-surface.md) supplies control,
data-scope and compensation correspondence for the protocol productions.
The [state contract](state-contract.md) specifies ordered queries, exact
aggregation, numeric domains and operation-anchor correspondence. Type arguments
after the reserved query names are type positions, not comparisons; the normal
comparison grammar applies inside their argument expressions.
Domain types named from an imported domain package give model-owned parameter
domains. The complete-V1 additions below also admit native `record`, `enum` and
`tuple` declarations for value types that are not domain objects. A native
declaration cannot declare a domain object.

`using` selects a declared profile alias for every native declaration;
there is no backend-derived default. A profile's string names its definition
identity, which the explicit compiler inventory resolves. A grammar parse does
not prove that identity. A `profile` declaration may name any
[header-selectable layer](definitions/README.md#header-selectable-layers), and
the declarations that use its alias are admitted under that layer's
admitted-form set (FR-001).

A `model` declaration imports one domain package. Its string names the domain
package identity, and `digest` is the
`sha256-jcs` digest of the domain package: SHA-256 over the RFC 8785 canonical
bytes of the semantic IR document lifted from the imported spec's artifacts
([AD-006](../../spec/assurance/AD-006-model-graph-state-contracts.md)). The
import is part of the package identity. Clauses name domain types by qualified
name through the model alias, for example `Orders::Order` after
`model Orders = "acme/orders" digest "…";`. A qualified name
resolves to one effective declaration key (domain package identity, IR node
identity, `sha256-jcs`). A name absent from the domain package refuses. The
three-segment name `Orders::Tier::Gold` names the case `Gold` of the variant
type `Tier` (the grammar is unchanged; resolution is in
[model-complete](definitions/model-complete.md)).
The [package contract](package-contract.md) defines the closed source inventory,
domain package import, exact definition dependencies and typed binding roles.
The explicit compiler inventory supplies the edition definition matching the
header; no installed-backend default completes an otherwise missing selection.

## Complete-V1 facet grammar

When the package selects `quire.value.complete/v1`,
`quire.model.complete/v1`, `quire.temporal.complete/v1`,
`quire.protocol.complete/v1` and the applicable runtime/analysis facets, the
following productions extend or replace the same-named bounded productions
above. These are edition-owned terminals, not backend-discovered syntax. A
recognized construct whose required facet is absent receives a located
`unsupported_construct`; a selected frontend that has not implemented an
admitted construct receives `unimplemented_capability` and cannot reinterpret
it.

```ebnf
complete-unit = header, profile, { profile }, { import-decl }, { model },
                declaration, { declaration }, EOF ;
import-decl   = 'import', string, [ 'as', ident ], ';' ;

declaration   = dimension-decl | unit-decl | enum-decl | record-decl
              | tuple-decl | union-decl | alias-decl | function | predicate
              | state-clause | temporal-clause | protocol-clause
              | relation-clause | hyper-clause | state-graph-clause
              | hybrid-decl | synthesis-decl | verification-plan
              | operation-body | abstraction-decl | terminal-decl ;

type-ref      = 'Boolean' | 'Integer' | qualified-name
              | 'Int', '[', signed-int, ',', signed-int, ']'
              | 'Rational', '[', signed-int, ',', signed-int, ';',
                            uint, ',', uint, ']'
              | 'Decimal', '[', signed-int, ',', signed-int, ';',
                           uint, ',', uint, [ ';', rounding-mode ], ']'
              | ('Float32' | 'Float64'), [ '[', rounding-mode, ']' ]
              | 'Text', '[', uint, ',', uint, ';', text-profile, ']'
              | 'Option', '<', type-ref, '>'
              | ('Sequence' | 'Set' | 'Bag' | 'OrderedSet'),
                '<', type-ref, '>', [ '[', uint, ',', uint, ']' ]
              | 'Reference', '<', qualified-name, '>'
              | 'Quantity', '<', qualified-name, '>'
              | 'Population', '<', qualified-name, '>', [ '[', uint, ']' ] ;
qualified-name = ident, { '::', member-name } ;
rounding-mode = 'exact' | 'toward-zero' | 'toward-positive'
              | 'toward-negative' | 'nearest-even' | 'nearest-away' ;
text-profile  = 'unicode-scalars' | 'nfc' | 'nfd' | 'nfkc' | 'nfkd'
              | 'binary-utf8' ;

dimension-decl = 'dimension', ident,
                 [ '=', dimension-term, { ('*' | '/'), dimension-term } ], ';' ;
dimension-term = qualified-name, [ '^', signed-int ] ;
unit-decl     = 'unit', ident, ':', qualified-name, '=', exact-number,
                [ '*', qualified-name ], [ '+', exact-number ], ';' ;
enum-decl     = [ 'ordered' ], 'enum', ident, '{', enum-member,
                { ',', enum-member }, [ ',' ], '}' ;
enum-member   = ident, [ '=', string ] ;
record-decl   = 'record', ident, '{', field, { field }, '}' ;
field         = ident, ':', type-ref, [ '?' ], ';' ;
tuple-decl    = 'tuple', ident, '(', type-ref, { ',', type-ref }, ')', ';' ;
union-decl    = 'union', ident, '{', union-member, { ',', union-member },
                [ ',' ], '}' ;
union-member  = ident, [ '(', type-ref, { ',', type-ref }, ')' ] ;
alias-decl    = 'type', ident, '=', type-ref, ';' ;

function      = 'function', ident, 'using', ident,
                '(', [ parameter, { ',', parameter } ], ')', ':', type-ref,
                'pure', [ 'decreases', '(', expr, ')' ], block ;
parameter-type = type-ref ;
parameter     = ident, ':', type-ref ;

expr          = 'let', ident, '=', expr, 'in', expr
              | 'if', expr, 'then', expr, 'else', expr
              | 'case', expr, '{', case-arm, { case-arm }, '}'
              | implication ;
case-arm      = qualified-name, [ '(', ident, { ',', ident }, ')' ],
                ':', expr, ';' ;

primary       = 'true' | 'false' | 'null' | 'none' | 'self' | 'result'
              | uint | string | exact-number | collection-value
              | record-value | float-value
              | qualified-name, [ '(', [ expr, { ',', expr } ], ')' ]
              | '(', expr, ')'
              | ('present' | 'value' | 'deref' | 'pre'), '(', expr, ')'
              | 'convert', '<', type-ref, '>', '(', expr, ')'
              | 'allInstances', '<', type-ref, '>', '(', expr, ')'
              | collection-call
              | protocol-reference
              | 'reaches', '(', expr, ',', expr, ',', qualified-name, ')'
              | 'lookup', '<', type-ref, '>', '(', expr, ',', expr, ')',
                'absent', absence-mode ;
protocol-reference = 'protocol', qualified-name ;
absence-mode  = 'undefined' | 'empty' | 'refused' ;
operation-body = 'body', ident, 'using', ident, 'on', qualified-name,
                 '(', [ ident, { ',', ident } ], ')', block ;
exact-number  = 'rational', '(', signed-int, ',', signed-int, ')'
              | 'decimal', '(', signed-int, ',', uint, ')' ;
float-value   = 'float32', '(', 'bits', ':', hex32, ')'
              | 'float64', '(', 'bits', ':', hex64, ')' ;
hex32         = '0x', hex-digit, hex-digit, hex-digit, hex-digit,
                     hex-digit, hex-digit, hex-digit, hex-digit ;
hex64         = '0x', hex-digit, hex-digit, hex-digit, hex-digit,
                     hex-digit, hex-digit, hex-digit, hex-digit,
                     hex-digit, hex-digit, hex-digit, hex-digit,
                     hex-digit, hex-digit, hex-digit, hex-digit ;
hex-digit     = '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7'
              | '8' | '9' | 'a' | 'b' | 'c' | 'd' | 'e' | 'f' ;
collection-value = 'sequence', '[', [ expr, { ',', expr } ], ']'
                 | 'set', '[', [ expr, { ',', expr } ], ']'
                 | 'bag', '[', [ expr, { ',', expr } ], ']'
                 | 'orderedSet', '[', [ expr, { ',', expr } ], ']' ;
record-value  = qualified-name, '{', field-value, { ',', field-value }, '}' ;
field-value   = ident, ':', expr ;

postfix       = primary, { '.', member-name,
                           [ '(', [ expr, { ',', expr } ], ')' ]
                         | '[', expr, ']' } ;
product       = unary, { ('*' | '/' | 'div' | 'rem' | 'mod'), unary } ;
collection-call = ('map' | 'collect' | 'filter' | 'flatMap'),
                  '(', ident, 'in', expr, ':', expr, ')'
                | 'flatten', '(', expr, ')'
                | ('fold' | 'reduce'), '<', qualified-name, '>',
                  '(', ident, ',', ident, 'in', expr, ':', expr,
                  [ ',', 'identity', ':', expr ], ')' ;
```

`null`, `none`, collection constructors, record constructors and bit-level
float constructors are disjoint forms; no empty spelling aliases
another kind. Decimal source is exact coefficient/scale syntax. In
`Decimal[lo, hi; smin, smax; mode]` the signed pair is the inclusive membership
coefficient bound and the unsigned pair is the inclusive membership scale bound;
FR-140 owns their well-formedness and value-only membership. IEEE source is
bit-level so signed zero and NaN payload identity survive parsing. Human decimal
float conversion is an explicit library function under a selected rounding
profile, not a literal shortcut.

The one qualified reference/call production is syntactically canonical. Static
resolution classifies a bare qualified reference as an enum member, a
domain-package population (inside a state clause, as the operand of
`allInstances` or `lookup`; FR-153) or other declared value and a qualified call as a tuple constructor, union member
constructor or pure function; the parser does not duplicate those token
strings into `enum-value`, `tuple-value`, `union-value` and call alternatives.

`union-decl` declares a closed set of named members, each with an optional
ordered payload shape; a member with no parenthesized payload is nullary. A
union value expression is the same qualified-call primary already used for
tuple constructors: `U::m(e, ...)` for a member with a payload, or the bare
qualified reference `U::m` for a nullary member. `expr` additionally admits
`case`, matching a scrutinee against the declared members of one union through
`case-arm` entries; each arm's optional bound identifiers correspond
positionally to its member's declared payload. The `case` keyword spelling is
the same terminal already reserved for the protocol `choice`/`case` construct
in the composed grammar above; the two productions are distinguished by their
syntactic context, the same way `first`, `value` and `result` are reused across
this grammar. A `case` scrutinee is an `expr` that admits no top-level
`record-value`, so the `{` following a scrutinee always opens the arm list; a
record value in scrutinee position is written parenthesized.

The selected `quire.value.complete/v1` definition reserves the following exact
qualified intrinsic identities. They use the ordinary qualified-call production
above; their meaning is not inferred from an unqualified spelling or installed
library: `quire::value::ieee::sqrt(x)`,
`quire::value::ieee::fma(a,b,c)`,
`quire::value::ieee::numericEqual(a,b)`,
`quire::value::ieee::totalOrder(a,b)` and
`quire::value::ieee::bitIdentical(a,b)`. User declarations cannot replace or
shadow those selected definition identities.

`Population<T>[N]` spells an explicit population binding's type, where `T`
is a qualified name resolving to a model `object` type and `N` is its declared
maximum. The maximum is optional: `Population<T>` declares no maximum, not a
default finite bound. The bracketed form has one `uint`, including `0`, rather
than the two bounds of a collection type; it admits no sign, leading zero,
fraction, exponent or `*`. Recognition of a qualified name does not prove its
object kind; FR-153 owns that semantic check and the query result types.
`Population` is case-sensitive and reserved in complete-V1 native binder
positions; contextual `member-name` positions still admit it. Lowercase
`population` remains the distinct contextual abstraction-binding keyword and
an identifier elsewhere, never an alias for the uppercase type constructor.
Neither spelling creates a population or supplies an ambient binding. A state clause names a domain-package population by the qualified name `M::P` as the operand of `allInstances` or `lookup` (FR-153); that name selects a declaration and takes its binding from the clause's observation.

The selected `quire.model.complete/v1` definition gives the model forms above
their meaning; FR-150 through FR-153 own it. `lookup<T>(p, r) absent m` reads
the closed population binding `p` at key `r` with the per-query absence mode
`m`. `receiver.member-name(args)` is a dispatched operation call: `member-name`
resolves statically to one exposed effective operation of the receiver's static
type and the call dispatches once, on the receiver's runtime most-specific
type. `receiver.member-name` without parentheses is field or relationship-end
navigation. An `operation-body` supplies the pure query body of one effective
operation `qualified-name`, whose first segment is a declared model alias (a
`model` declaration or an import `as` alias) naming the model; its `using`
selects a declared profile alias, as for every native declaration;
its identifiers bind the receiver-less operation parameters positionally, in
signature order, and the body reads the receiver as `self`. A package that does
not select `quire.model.complete/v1` receives `unsupported_construct` for each
of `lookup`, a dispatched call and an `operation-body`. This sentence does not
gate `Population<T>[N]`, `allInstances<T>(p)` or the field and relationship-end
navigation: they are not refused as `unsupported_construct`, and are admitted
or refused by the package's layer selection, not by this gate.

### Complete temporal, choreography and analysis forms

```ebnf
interval      = '[', uint, ',', ( uint | '*' ), ']' ;
temporal-clause = 'temporal', ident, 'using', ident,
                  'over', '(', parameter, ')', 'clock', string,
                  activation, '{', { fairness }, { capture },
                  temporal-expr, '}' ;
fairness      = 'fair', [ 'weak' | 'strong' ], [ 'whole' | 'each' ],
                qualified-name, ';' ;
terminal-decl = 'terminal', ident, 'using', ident, 'on', ident,
                ( 'when', block | 'any' ), ';' ;
temporal-unary = 'not', temporal-unary
               | ('eventually' | 'always' | 'once' | 'historically'),
                 interval, temporal-unary
               | temporal-primary ;
temporal-relation = temporal-unary,
                    [ ('until' | 'release' | 'since' | 'triggered'),
                      interval, temporal-unary ] ;

role          = 'role', ident, 'on', qualified-name, ';'
              | 'role', ident, 'each', qualified-name,
                'from', ident, 'max', uint,
                'lifetime', role-lifetime, ';' ;
role-lifetime = 'workflow' | 'scope' | 'until', '(', expr, ')' ;
channel       = 'channel', ident, 'from', ident, 'to', ident,
                'carries', type-ref, 'ordering', ordering,
                'delivery', delivery-policy,
                'capacity', capacity,
                'overflow', overflow-policy, ';' ;
delivery-policy = 'unknown' | 'at-most-once' | 'at-least-once'
                | 'exactly-once-premise' ;
capacity      = uint | 'symbolic', '(', ident, ')' ;
overflow-policy = 'reject' | 'block' | 'loss', loss-selector ;
loss-selector = 'head' | 'tail' | 'new' ;
parallel      = 'parallel', ident, [ memory-clause ],
                '{', branch, branch, { branch }, '}',
                'join', join-policy, '[', ident, { ',', ident }, ']',
                [ 'outstanding', ('continue' | 'cancel') ], ';' ;
memory-clause = 'memory', ( 'sc' | 'tso' | 'ra' ) ;
join-policy   = 'all' | 'any' | 'quorum', '(', uint, ')'
              | 'predicate', '(', ident, ')' ;
repetition    = 'repeat', ident, 'by', ident, visibility,
                ( 'max', uint, [ loop-proof ],
                  'while', block, control, 'exhausted', control
                | [ loop-proof ], 'while', block, control ) ;
loop-proof    = 'invariant', block, 'variant', block ;

protocol-clause = 'protocol', ident, 'using', ident,
                  'over', '(', parameter, ')', activation, '{',
                  { capture }, role, { role }, { relationship }, { channel },
                  { protocol-requirement | compensation },
                  { protocol-terminal | protocol-scheduling },
                  'run', control, finish, '}' ;
protocol-terminal   = 'terminal', ( 'when', block | 'any' ), ';' ;
protocol-scheduling = 'scheduling', 'adversarial', ';' ;
control       = sequence | choice | parallel | repetition | await-control
              | event-node | check | commit | fence ;
fence         = 'fence', ident, 'ordering', fence-order, ';' ;
event-node    = 'send', ident, 'via', ident,
                'as', '(', parameter, ')', { related }, block, ';'
              | 'receive', ident, 'via', ident, 'of', node-ref,
                'as', '(', parameter, ')', { related }, block, ';'
              | 'attempt', ident, 'by', ident, 'on', operation-name,
                [ 'ordering', memory-order ],
                'contracts', '[', [ ident, { ',', ident } ], ']',
                'as', '(', parameter, ')', { related }, block, ';'
              | 'effect', ident, 'of', node-ref,
                'as', '(', parameter, ')', { related }, block, ';'
              | 'event', ident, 'by', ident, [ 'for', node-ref ],
                'as', '(', parameter, ')', { related }, block, ';' ;
memory-order  = 'relaxed' | 'acquire' | 'release' | 'acq_rel' | 'seq_cst' ;
fence-order   = 'acquire' | 'release' | 'acq_rel' | 'seq_cst' ;
protocol-step-row    = 'step', protocol-step-source, '->',
                       protocol-step-target, ';' ;
protocol-step-source = [ 'fork' | 'join' | 'cend' | 'duplicate' | 'lose'
                         | 'spawn' | 'retire' | 'activate' ], qualified-name
                     | 'memory', ident ;
protocol-step-target = step-target
                     | 'node', qualified-name,
                       [ '(', step-argument, { ',', step-argument }, ')' ] ;

relation-clause = 'relation', ident, 'using', ident,
                  'over', '(', execution-binding, ',', execution-binding,
                  { ',', execution-binding }, ')', block ;
execution-binding = ident, ':', qualified-name ;
hyper-clause  = 'hyper', ident, 'using', ident, 'over',
                ( trace-domain, quantifier, { quantifier }, block
                | behaviours-domain, quantifier, { quantifier },
                  [ align-skip ], [ match-block ],
                  '{', temporal-expr, '}' ) ;
trace-domain  = ident, 'in', qualified-name, 'bounded', uint ;
behaviours-domain = 'behaviours',
                    [ '(', parameter, { ',', parameter }, ')' ] ;
quantifier    = ('forall' | 'exists'), 'trace', ident,
                [ 'of', ident, { fairness } ] ;
align-skip    = 'align', 'skip', '{', qualified-name,
                { ',', qualified-name }, '}' ;
match-block   = 'match', block ;
step-label    = ident, '.', 'step', '.',
                ( 'op' | 'receiver' | qualified-name, '.', member-name ) ;
indexed-read  = postfix, '@', ident ;
state-graph-clause = possible-clause | always-possible-clause
                   | unique-path-clause ;
possible-clause = 'possible', ident, 'using', ident,
                  [ graph-over ], graph-body ;
always-possible-clause = 'always', 'possible', ident, 'using', ident,
                         [ graph-over ],
                         [ 'from', '(', graph-predicate, ')' ],
                         graph-body ;
unique-path-clause = 'unique', 'path', ident, 'using', ident,
                     [ graph-over ], 'from', '(', graph-predicate, ')',
                     'to', '(', graph-predicate, ')', ';' ;
graph-body    = '{', { fairness }, graph-predicate, '}' ;
graph-predicate = 'let', ident, '=', expr, 'in', graph-predicate
                | 'if', expr, 'then', graph-predicate,
                  'else', graph-predicate
                | graph-implication ;
graph-implication = graph-disjunction,
                    [ 'implies', graph-implication ] ;
graph-disjunction = graph-conjunction, { 'or', graph-conjunction } ;
graph-conjunction = graph-relation, { 'and', graph-relation } ;
graph-relation = graph-unary,
                 [ ('until' | 'release' | 'since' | 'triggered'),
                   [ interval ], graph-unary ] ;
graph-unary   = 'not', graph-unary
              | ('eventually' | 'always' | 'once' | 'historically'),
                [ interval ], graph-unary
              | comparison
              | '(', graph-predicate, ')' ;
graph-over    = 'over', '(', parameter, ')' ;
hybrid-decl   = 'hybrid', ident, 'using', ident, '{', hybrid-mode,
                { hybrid-mode }, '}' ;
hybrid-mode   = 'mode', ident, 'invariant', block,
                'flow', '{', equation, { equation }, '}',
                { 'transition', 'to', ident, 'when', block,
                  'reset', block, ';' } ;
equation      = qualified-name, "'", '=', expr, ';' ;
synthesis-decl = 'synthesis', ident, 'using', ident,
                 'grammar', qualified-name, 'domain', qualified-name,
                 'satisfies', block, ';' ;

verification-plan = 'verify', ident, 'using', ident, '{',
                    verification-step, { verification-step }, '}' ;
verification-step = 'check', ident, 'claim', qualified-name,
                    'method', qualified-name,
                    'domain', qualified-name,
                    [ 'depends', '[', ident, { ',', ident }, ']' ],
                    [ 'bound', uint ], ';' ;
```

A replicated role's instance binder is the role's own name: in `role w each
M::Worker from workers max 2 lifetime until(w.done)`, `w` names one instance
inside the `until` expression, typed `Reference<M::Worker>`. The grammar has
no separate binder production, and the `until` expression is written between
the parentheses with no braces. The binder is readable only in that role's own `until` expression; a role name used as a value anywhere else refuses `missing_declaration`/`missing-name` at the name (FR-428).

An interval is a closed `[a,b]` or a lower-bounded `[a,*]`; `[a,*]` is
admitted only on a clause that selects `quire.temporal.infinite-trace/v1`, and
an unbounded operator is written with no interval
([FR-367](../../spec/functional/temporal/FR-367-evaluate-interval-operators-under-infinite-trace.md)).
`fair`, `weak`, `strong`, `whole`, `terminal` and `any` are reserved words of
the temporal facet. A `fairness` constraint is admitted only on a clause that
selects `quire.temporal.infinite-trace/v1`; its unwritten kind is `weak` and
its unwritten granularity is `whole`
([FR-362](../../spec/functional/temporal/FR-362-apply-fairness-constraints.md)).
A package has at most one `terminal-decl` per model alias; it declares the
model's intended terminal states
([FR-366](../../spec/functional/temporal/FR-366-check-deadlock-freedom.md)).
`exactly-once-premise` is an authored
environment premise, never a transport guarantee inferred by the runtime.
`quorum(n)` requires `1 <= n <= branch-count`. A named join predicate is a
total pure function over the explicitly captured branch dispositions. Dynamic
role lifetime and channel capacity remain checked semantic expressions; a
symbolic capacity requires a discharged finite bound before finite execution.
Verification plans link outside clause-expression evaluation even though their
source is recognized by the same edition.

A complete `repetition` carries progress, every cycle of its body passing
through a step node; an iteration maximum and a
`loop-proof` are each optional model meaning, and only a repetition with a
maximum has an `exhausted` control (FR-052). A protocol clause carries at most
one `protocol-terminal` and at most one `protocol-scheduling` member; a second
refuses `ambiguous_declaration`/`ambiguous-name` (FR-429, FR-430). The
`memory-clause`, `memory-order` and `fence` forms have the meaning of FR-434
and FR-435. A `protocol-step-row` is one row of a refinement declaration whose
concrete side is a protocol subject (FR-432); it is admitted wherever that
declaration admits a step row, and its first `qualified-name` names a
concrete protocol node, template, channel, role or protocol. `step-target`
and `step-argument` are the refinement declaration's productions (FR-375).

The refinement header selects its abstract subject with
`abstract-subject = ident, [ '::', member-name ] ;` (FR-375): a lone `ident`
is a model alias and `ident '::' name` is a model alias with one protocol
clause of that model, as in `abstract Spec::Twice`. The parser tells the two
apart by the `::` only, and the token after `::` is always the name (reserved words included); it resolves neither name, and the checker resolves the
alias and the protocol (FR-375, FR-432). The `concrete` side stays one
`ident`.

#### Refinement population and option forms

Inside a refinement declaration, and only there, `primary` additionally admits:

```ebnf
primary   =/ model-name
           | 'count', '(', binder, ')'
           | 'sum', '(', binder, ':', expr, ')'
           | ('exists' | 'all'), '(', binder, ')'
           | 'only', '(', binder, ':', expr, ')'
           | 'seq', '(', ident, 'in', expr, '..', expr, ':', expr, ')'
           | 'Some', '(', expr, ')' ;
binder    = ident, 'in', model-name, [ 'where', expr ] ;
```

A `model-name` in `primary` position names a concrete population. `count`,
`sum`, `exists` and `all` are reserved words already. The population forms
differ from the existing forms by what follows: `count` and `sum` read the
population form when no `<` follows the word (the typed aggregates are
`count<T>(` and `sum<T>(`), and `exists(binder)` reads the population form
when `where` or `)` follows the binder's `model-name`, while `:` reads the
state-clause quantifier over a collection expression. `only`, `seq` and `Some`
are matched as keyword spellings only in these productions and remain
identifiers elsewhere; inside a refinement row `only(`, `seq(` and `Some(`
always read as the forms and so hide a pure function of the same name. `where`
is matched as a keyword in `binder`, as it already is in the temporal
`aggregate-term`, and is an identifier in other positions. `FR-376` owns the typing and value of the six
population forms and the `..` bounds; `FR-375` owns `Some` and the rows it
appears in. A state-clause body outside a refinement declaration that uses
a population name or the `count`, `sum`, `exists` or `all` population form refuses
`unsupported_construct`/`expression-form`; `only` and `seq` spelled as forms there
are syntax errors (`invalid_syntax`/`unexpected-token`) at their `in` token.

### Abstraction relation declaration

An `abstraction-decl` spells the model-to-implementation abstraction relation
of FR-353. FR-450 owns its static meaning and refusals. It is distinct from
`relation-clause`, which relates executions of one model.

```ebnf
abstraction-decl   = 'abstraction', ident, 'using', ident, '{',
                     abstraction-binding, { abstraction-binding }, '}' ;
abstraction-binding = object-binding | population-binding | frame-binding ;
object-binding     = 'type', type-name, 'to', rust-path,
                     [ '{', field-binding, { field-binding }, '}' ], ';' ;
field-binding      = member-name, 'to', rust-field, ';' ;
population-binding = 'population', model-name, 'to', rust-path, ';' ;
frame-binding      = 'operation', operation-name, 'to', rust-path,
                     'receiver', rust-receiver,
                     '(', [ parameter-binding, { ',', parameter-binding } ],
                     ')', ';' ;
parameter-binding  = member-name, 'to', rust-name ;
rust-path          = string ;
rust-field         = string ;
rust-receiver      = string ;
rust-name          = string ;
```

The four Rust positions are quoted strings so that Rust spellings never meet
this edition's lexical contract. The parser recognizes any string in them.
FR-450 checks each string's content against its Rust syntax, and because that
content admits no escape sequence, every Rust segment's byte offsets inside the
string are source spans. A package that does not select
`quire.model.complete/v1` receives `unsupported_construct` for an
`abstraction-decl`.

### Hyperproperty and state-graph forms over a model

[FR-395](../../spec/functional/analysis/FR-395-analyze-hyperproperties-over-behaviours.md)
gives a `hyper` clause over `behaviours` its meaning,
[FR-397](../../spec/functional/analysis/FR-397-check-step-relations-over-model-operations.md)
a `relation` clause whose execution bindings name model operations,
[FR-398](../../spec/functional/analysis/FR-398-align-hyperproperty-traces-on-a-projection.md)
the `align skip` list and
[FR-390](../../spec/functional/analysis/FR-390-analyze-state-graph-claims.md)
the three state-graph clauses. The rules below are recognition and binding
rules only.

- `behaviours`, `align`, `skip`, `possible`, `unique` and `path` are
  reserved words of the complete facets. `step`, `op`, `receiver` and
  `stutter` are matched as keyword spellings only inside a `match-block`, in
  the `step-label` position and as the `stutter` primary; they remain
  identifiers and member names elsewhere.
- A quantifier's fairness set is the `fairness` constraints written after its
  alias, each the temporal facet's `fairness` production with its `;`; a
  quantifier with none has the empty set
  ([FR-362](../../spec/functional/temporal/FR-362-apply-fairness-constraints.md)).
- Inside a `match-block`, `primary` additionally admits `step-label` and the
  `stutter` keyword. `step-label` is admitted nowhere else.
- Inside the body of a `hyper` clause over `behaviours`, the `expr` of a
  `holds(...)` leaf admits `indexed-read` wherever it admits `postfix`. `@`
  is a single punctuation token that binds tighter than every operator and
  looser than member navigation, so `v.l @ a = v.l @ b` is
  `(v.l @ a) = (v.l @ b)`. `indexed-read` is admitted nowhere else.
- An `align-skip` entry and a model-operation execution binding each name an
  operation as `qualified-name`, whose first segment is a model alias the
  clause binds.
- The predicate positions of a state-graph clause (its body and its `from`
  and `to` operands) are `graph-predicate`s: a clause expression in which the
  unary and binary temporal operators of the temporal facet stand at any
  operand position, each with an optional `interval`, and in which `fairness`
  constraints may stand at the start of a `graph-body`. One rule resolves
  every overlap between `graph-predicate` and `expr`: a predicate or
  parenthesised group that derives as an `expr` is read as that `expr`, and
  the `not`, `(` and temporal alternatives of `graph-unary` apply only to a
  phrase that does not. So `not c.n = 2` is the `expr` `(not c.n) = 2`, as in
  any expression, and `not eventually c.n = 2` and `(eventually c.n = 2)` are
  read through `graph-unary`. The production is recognition only: it carries
  no profile selection, and a temporal operator or a fairness constraint that
  a production recognizes is parsed and then refused by
  [FR-390](../../spec/functional/analysis/FR-390-analyze-state-graph-claims.md)
  at admission. Everything else is a syntax error: a fairness constraint
  anywhere but the start of a `graph-body` (so none in a `from` or `to`
  operand, and none on a `unique path` clause, which has no body), and a
  temporal operator in an `if` condition, a `let` binding, or an operand of
  a primary, such as a call argument or a quantifier body.
  A `graph-relation` admits at most one binary temporal operator, as the
  temporal clause's `temporal-relation` does, so `c.a until c.b until c.c` is
  a syntax error and a chain is written `(c.a until c.b) until c.c`. `over` is
  optional in all three clauses.
- The keywords of the state-graph clauses are distinct from the `reaches`
  primary: `reaches` reads one state's object graph and the state-graph
  clauses read the model's state graph.

| Source fragment | Expected syntax / binding |
| --- | --- |
| `hyper NI using inf over behaviours (v: V::Vault) forall trace a of V forall trace b of V match { a.step.op = b.step.op } { always holds(v.l @ a = v.l @ b) }` | One hyper clause over behaviours, two quantifiers over alias `V`, one match block and one temporal body with two indexed reads. |
| `hyper NI using inf over behaviours forall trace a of V fair weak V::Vault::step; forall trace b of V { always holds(true) }` | Quantifier `a` with one weak `whole` constraint; quantifier `b` with the empty fairness set. |
| `hyper H using inf over t in Q bounded 4 forall trace a of V { true }` | Parses; the `of` on a finite trace domain refuses at semantic admission (FR-395). |
| `always holds(v.l @ a)` in a `temporal` clause | Syntax error: `indexed-read` is admitted only in a hyper body over `behaviours`. |
| `possible Win using v over (g: G::Game) { g.phase = G::Phase::Won }` | One `possible` state-graph clause with one `over` parameter. |
| `always possible Rec using v from (not x.lost) { x.ok }` | One `always possible` clause with no `over` parameter and one `from` predicate. |
| `unique path One using v over (j: M::Job) from (not j.a) to (j.done);` | One `unique path` clause with its two predicates. |
| `possible R using v over (c: C::Cfg) { c.n = 1 and eventually c.n = 2 }` | Parses: a conjunction whose second operand is an unbounded `eventually`. Refused at admission, `unsupported_construct`/`expression-form` at `eventually` (FR-390-AC-2). |
| `possible N using v { not eventually[0,5] c.n = 2 }` and `possible P using v { (eventually c.n = 2) }` | Both parse, with an interval operator and a parenthesised temporal operand; each is refused at admission at the operator (FR-390-AC-2). |
| `possible U using v { a until b }` | Parses as a binary temporal operator with no interval; refused at admission at `until` (FR-390-AC-2). |
| `always possible Rec using v from (always c.ok) { c.done }` | Parses: a temporal operator in a `from` operand; refused at admission at `always` (FR-390-AC-2). |
| `always possible Rec using v { fair weak M::Cfg::step; c.done }` | Parses: a fairness constraint at the start of the body; refused at admission at the constraint (FR-390-AC-2). |
| `possible P using v { c.done; fair weak M::Cfg::step }` | Syntax error: a fairness constraint is admitted only at the start of a body. |
| `possible N using v over (c: C::Cfg) { not c.n = 2 }` | Parses as the `expr` `(not c.n) = 2`, as in any expression; a type error at the checker, not a grammar case. |
| `possible W using v over (c: C::Cfg) { eventually[1,*] c.done }` | Parses; the interval is recognized and FR-390 refuses at `eventually` (`expression-form`); no profile refusal applies, since a state-graph claim selects no temporal profile. |
| `possible B using v over (c: C::Cfg) { if eventually c.a then c.b else c.c }` and `unique path One using v from (c.a) to (c.d); fair weak M::Cfg::step` | Syntax errors: a temporal operator in an `if` condition, and a fairness constraint on a clause with no body. |

### Probabilistic, timed and real-time forms

These productions extend `declaration` when the package selects the complete
facets. [FR-405](../../spec/functional/analysis/FR-405-declare-probabilistic-models.md)
to [FR-421](../../spec/functional/temporal/FR-421-check-probabilistic-timed-claims-exactly.md)
own their meaning.
`a =/ b` adds the alternatives `b` to the existing production `a`.

```ebnf
declaration    =/ random-decl | reward-decl | delay-decl | workload-decl
                | probabilistic-clause | time-decl | time-invariant
                | urgent-decl | taskset-decl | release-decl
                | schedulability-clause ;

exact-value    = uint | decimal-literal | exact-number ;
quantity-literal = exact-value, qualified-name ;
primary        =/ quantity-literal ;

random-decl    = 'random', qualified-name, '(', ident, ')', '~',
                 '{', weighted-value, { ',', weighted-value }, [ ',' ], '}', ';' ;
weighted-value = expr, ':', exact-value ;
reward-decl    = 'reward', ident, 'on', qualified-name, '=', expr, ';' ;
delay-decl     = 'delay', qualified-name, '~', delay-dist, ';' ;
delay-dist     = 'uniform', [ '[', quantity-literal, ',', quantity-literal, ']' ]
               | 'exponential', '(', exact-value, ')'
               | 'discrete', '{', weighted-delay, { ',', weighted-delay },
                 [ ',' ], '}' ;
weighted-delay = quantity-literal, ':', exact-value ;
workload-decl  = 'workload', ident, 'on', ident, '{',
                 workload-weight, { workload-weight }, '}' ;
workload-weight = 'weight', qualified-name, '=', exact-value, ';' ;

probabilistic-clause = 'probabilistic', ident, 'using', ident, 'on', ident,
                       [ 'over', '(', parameter, ')' ],
                       scheduler-clause, activation, [ confidence ],
                       '{', probabilistic-form, '}' ;
scheduler-clause = 'under', ident
                 | 'under', 'every', 'scheduler', { fairness-item } ;
fairness-item  = 'fair', ( 'weak' | 'strong' ), [ 'each' ], qualified-name ;
confidence     = 'with', confidence-item, { ',', confidence-item } ;
confidence-item = ( 'alpha' | 'beta' | 'indifference' ), exact-value ;
probabilistic-form
               = 'probability', bound-op, exact-value, '[', event, ']'
               | 'quantile', exact-value, 'of', '(', measure, ')', bound-op, expr
               | 'mean', 'of', '(', fraction, ')', bound-op, exact-value
               | 'long_run_fraction', 'holds', '(', expr, ')',
                 [ weighting ], bound-op, exact-value
               | 'expected', ( 'accumulate', ident | 'elapsed' ),
                 'until', 'holds', '(', expr, ')', bound-op, expr ;
bound-op       = '<=' | '>=' ;
event          = temporal-expr | measure, bound-op, expr ;
measure        = ( 'accumulate', ident | 'steps' | 'elapsed' ),
                 'from', 'holds', '(', expr, ')',
                 'until', 'holds', '(', expr, ')', 'within', horizon
               | fraction ;
fraction       = 'fraction', 'holds', '(', expr, ')',
                 'over', '[', '0', ',', horizon, ']', [ weighting ] ;
weighting      = 'weighted', 'by', ( ident | 'delay' ) ;
horizon        = uint | quantity-literal ;

aggregate-atom = aggregate-term, ( '<' | '<=' | '=' | '>=' | '>' ), expr ;
aggregate-term = ( 'count' | 'sum' | 'min' | 'max'
                 | 'fraction', 'holds', '(', expr, ')'
                 | 'quantile', exact-value ),
                 [ 'of', expr ], [ 'where', 'holds', '(', expr, ')' ],
                 'over', 'past', window, 'min_count', uint ;
window         = '[', horizon, ',', horizon, ']' ;
temporal-primary =/ aggregate-atom ;

time-decl      = 'time', ident, ( 'dense'
                                | 'tick', qualified-name, 'period', quantity-literal ),
                 'unit', qualified-name, ';' ;
time-invariant = 'time', 'invariant', ident, 'when', block,
                 '{', clock-bound, { 'and', clock-bound }, '}' ;
clock-bound    = postfix, ( '<' | '<=' ), quantity-literal ;
urgent-decl    = 'urgent', ident, ( 'when', block | qualified-name ), ';' ;
type-ref       =/ 'Clock' ;
timed-interval = ( '[' | '(' ), quantity-literal, ',', quantity-literal, ( ']' | ')' ) ;

taskset-decl   = 'taskset', ident, '{', task, { task }, '}' ;
task           = 'task', ident, ( 'periodic' | 'sporadic' ), quantity-literal,
                 'deadline', quantity-literal, 'wcet', quantity-literal,
                 [ 'priority', uint ], [ 'jitter', quantity-literal ],
                 [ 'blocking', quantity-literal ],
                 ( 'lo' | 'hi', 'wcet_hi', quantity-literal ), ';' ;
release-decl   = 'release', ident, '.', ident, 'on', qualified-name, ';' ;
schedulability-clause
               = 'schedulable', ident, 'on', ident, 'under', policy, ';'
               | 'response', ident, 'on', ident, '.', ident,
                 '<=', quantity-literal, 'under', 'fixed_priority', ';' ;
policy         = 'fixed_priority' | 'edf' | 'amc' ;
```

`random-decl` names an operation and one of its parameters; `reward-decl`,
`delay-decl` and `workload-weight` name operations of the model whose alias
`workload-decl` and `probabilistic-clause` name after `on`. Under the
`quire.temporal.timed/v1` profile an interval-capable operator takes a
`timed-interval` in place of `interval`. An interval opener `(` is followed by
a `quantity-literal`, which begins with a `uint`, a `decimal-literal` or `rational`,
and a temporal operand never begins with any of these, so a recognizer
distinguishes a timed interval from a parenthesized operand by its second
token.

A `decimal-literal` is one token spelled `[0-9]+ '.' [0-9]+`, with no exponent. In an
`exact-value` position it denotes the exact rational it spells, so `0.95` is
`19/20` and `0.0005` is `1/2000`; it is never a floating-point value, and in
every other position it keeps the unsupported numeric-form diagnostic.

A `probabilistic-clause` with no `over` parameter has one binding, the empty
one; with `over (p: T)` it holds for every binding of `p`
([FR-406](../../spec/functional/analysis/FR-406-interpret-probabilistic-models.md)).
A `confidence` clause states each of `alpha`, `beta` and `indifference`
exactly once; a clause that omits or repeats one refuses with
`invalid_probabilistic_model`/`partial-confidence`
([FR-407](../../spec/functional/analysis/FR-407-state-probabilistic-claims.md)).
A `temporal-clause` with no `clock` member binds the profile's default clock
([FR-252](../../spec/objects/temporal/FR-252-temporal-clock-binding.md)).
A `window` takes `uint` positions under the event-position false-extension
profile and time quantities under the timestamped-event finite-window profile
([FR-414](../../spec/functional/temporal/FR-414-evaluate-window-aggregates.md)).
In `release-decl`, `release S.τ on M::o;` makes each step of operation `o`
release one job of task `τ` of task set `S`
([FR-419](../../spec/functional/analysis/FR-419-analyze-task-set-schedulability.md)).
In `schedulability-clause`, the first `ident` names the claim and the second
the task set. A clock read in `clock-bound` or in an operation's precondition
compares a `Clock` field with a `quantity-literal` of the time dimension. In
`task`, `wcet` is `C`, or `C(LO)` for a `hi` task whose `wcet_hi` is `C(HI)`.
A form whose facet is not selected receives `unsupported_construct`.

`Clock` in a `type-ref` is a keyword of the facet that admits the timed forms,
not a `qualified-name`: an unqualified `Clock` in a type position is always read
as the clock type, whether or not the unit selects that facet (a unit that does
not select it receives `unsupported_construct` for the form). `Clock` is not
admitted as a declared type name: a declaration of a record, type alias, enum,
tuple or union that would introduce an unqualified type named `Clock` is refused
`invalid_syntax` at the name.

### Protocol reference expression

`protocol-reference` is a `primary` alternative of the complete-V1 grammar
under `quire.protocol.complete/v1`. Its target is a `qualified-name` naming one
protocol declared in the same unit; the expression carries no package
selector, alias or key. `protocol` leads a reference only in expression
position, where no declaration can begin, so it never changes how a
`protocol-clause` declaration parses. A target that is not a `qualified-name`
is `invalid_syntax`. [FR-463](../../spec/functional/protocol/FR-463-refer-to-a-declared-protocol-in-an-expression.md)
owns the meaning.

## Precedence and bindings

From lowest to highest: `let`/`if`; `implies` (right associative); `or`; `and`;
one comparison; addition/subtraction; product; unary `not`/negation; field access,
calls and parenthesized primaries. Addition/product chains associate left.
Comparisons do not chain: `a < b < c` is invalid syntax. Parentheses select an
explicit tree, which must still type-check. `not a = b` means `(not a) = b`;
authors write `not (a = b)` for negating a comparison.

Model and profile aliases are unique within their source unit; they do not
pollute another source unit's alias scope. Native declaration names are unique
within the linked package, per namespace: dimensions, units and every other declaration (records, tuples, unions, enums,
aliases, functions, predicates, protocols, claims and declared values, the type
namespace) are three namespaces, so a unit may
share its name with a dimension or with a type. Ambiguous cross-unit
declarations refuse. A bare `type-ref` looks up the type namespace only, and
`Quantity<u>` is the one `type-ref` that looks up the unit namespace. A unit
containing only model-independent Boolean predicates needs no dummy model import.
Any referenced model declaration still requires an exact explicit import. Predicate
parameters and lexical binders are unique in their enclosing declaration and
cannot shadow aliases, native declarations or another visible binding. Separate
predicate declarations may reuse a parameter name. A `let` binding is visible
only in its body, not its initializer. A quantifier binder is visible only in
its predicate, not its domain. Forward predicate references can resolve after
declaration collection; the complete call graph, including the FR-151 dispatch
edges from a dispatched call to every candidate body and effective precondition
clause, must be acyclic. No overload
resolution or dynamic dispatch is introduced, except that a package selecting
`quire.model.complete/v1` admits exactly the single dispatch on the receiver's
runtime most-specific type defined by FR-151. That exception supersedes the
preceding sentence for such a package only; it adds no overload resolution.

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

The temporal operators use [FR-090](../../spec/functional/temporal/FR-090-select-temporal-profile-and-clock.md)
through [FR-095](../../spec/functional/temporal/FR-095-preserve-native-tl-correspondence.md).
The syntax does not select a different clock or closed-boundary convention to
fit a backend. Each interval has inclusive integer bounds `0 <= a <= b`; its
unit is one tick of the explicitly selected clock interpretation. Negative,
unbounded or fractional interval spellings are not interval syntax. Reversed
bounds parse but refuse semantic admission. Large valid bounds still require
resource admission. A rational sample period belongs to the clock binding,
not to an implicit decimal interval conversion.

### Unbounded temporal operator grammar edition (infinite-trace)

The refusal above governs the bracketed `interval` production. Under an
explicitly selected `quire.temporal.infinite-trace/v1` profile
([FR-090](../../spec/functional/temporal/FR-090-select-temporal-profile-and-clock.md),
[FR-250](../../spec/objects/temporal/FR-250-temporal-semantic-profile.md)),
this edition makes the interval optional on every interval-capable operator:

```ebnf
temporal-unary-unbounded = 'not', temporal-unary-unbounded
                          | ('eventually' | 'always' | 'once' | 'historically'),
                            [ interval ], temporal-unary-unbounded
                          | temporal-primary ;
temporal-relation-unbounded = temporal-unary-unbounded,
                    [ ('until' | 'release' | 'since' | 'triggered'),
                      [ interval ], temporal-unary-unbounded ] ;
```

These productions reuse the existing reserved keywords. An operator written
with an interval is an interval operator and one written without is an
unbounded operator; the two nest in any order, so
`always (holds(fail) implies eventually always[0,10] holds(healthy))` is one
admitted formula
([FR-367](../../spec/functional/temporal/FR-367-evaluate-interval-operators-under-infinite-trace.md)).
A package that does not select `quire.temporal.infinite-trace/v1` receives
`unsupported_construct` for an operator written without an interval. Selecting the profile
admits them without reinterpreting the bracketed form or any historical
`0-draft` bytes, per the edition rule in
[AD-003](../../spec/assurance/AD-003-native-quire-language-charter.md).
[AD-012](../../spec/assurance/AD-012-unbounded-temporal-operator-grammar.md)
accepts this edition; [FR-048](../../spec/functional/foundation/FR-048-bind-native-temporal-syntax.md)
admits it at the frontend and [FR-161](../../spec/functional/temporal/FR-161-analyze-infinite-trace-properties.md)
analyzes the resulting unbounded formula.

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
admitted instant. M::Sample is a domain type declared in the imported spec's artifacts, not a native schema.
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
enclosing unit must import an exact domain package and profile definitions before admission.
For this synthetic example the domain package M declares an explicitly bounded Amount 1..20,
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
requires the declared view; the spec's artifacts declare its population and F
supplies its membership, window and completeness contract. These finite snapshot/window aggregate rules introduce no
`allInstances()` or implicit history query; the complete-V1
`allInstances<T>(p)` form reads only an explicit population binding under
FR-153.

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

`/` is a left-associative product-level binary operator, never rational-literal
punctuation. Under the carried-forward admission it requires two values of one
named dimensionless rational type, a proved nonzero divisor and a normalized
result inside that same domain. `rational(1,2) / rational(1,2)` can therefore
denote 1/1 under an appropriate explicit type. Under a declaration selecting
`quire.state.core/v1` without `quire.value.complete/v1`, `/` still requires
operands of that same named rational type: integer operands refuse even when a
rational result type is expected. Under `quire.value.complete/v1`, bare `1/2`
cannot introduce a rational value without an expected rational type; with one,
[FR-149](../../spec/functional/type-model/FR-149-apply-complete-equality-matrix.md)
promotes integer operands exactly to `n/1` for rational `/`. `/` never denotes
integer division. `div`, `rem` and `mod` are
recognized at product precedence. When a declaration selects
`quire.value.complete/v1`, integer `div` and `rem` are admitted only with exactly
one checked integer-division definition, and `mod` has the independent
Euclidean meaning specified by [FR-147](../../spec/functional/expressions/FR-147-evaluate-integer-division-domains.md).
Without that facet, each integer operator receives a located profile refusal,
including when its expression is unreachable. Recognition alone never supplies
a missing definition or changes a selected law. Historical `0-draft` arithmetic
retains its original meaning.

## Protocol expressions and bindings

The protocol surface reuses `expr`, `block`, `parameter`, `capture`, `activation`,
`interval` and exact import/profile references. Its control nodes are not Boolean
expressions. A block is a delimiter around the common expression grammar; its
consumer declares whether the result must be Boolean (guards, checks, recovery)
or a specific admitted value type (FIFO keys). Shared predicate declarations
remain Boolean-only. No callback, evaluation of source text or user-code escape
is introduced by a protocol expression position.

`node-ref` resolves protocol structure, not a model type or value. `model-name`
resolves a declaration in an imported domain package; role, relationship,
payload and operation positions require different declaration kinds. Equal spellings
cannot substitute one kind for another. Group/branch paths and bounded iteration
occurrence identities retain the original node declaration and source span.
The [surface contract](choreography-surface.md) states definite availability of
node values, imported temporal requirements and exact runtime binding roles.

## Outstanding integration

Before accepting L1: reconcile the protocol examples with B's control contract,
the complete keyword/type catalog, the domain package declarations and F's exact observation binding slots
and anchors; supply exact profile definition artifacts; and run the selected
baseline review. This document neither closes those requirements nor qualifies
the current compiler against the new syntax.
