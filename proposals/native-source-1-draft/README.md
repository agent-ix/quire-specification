# Native source definition amendment

Status: **superseded as an adoption candidate; retained as the reviewed source of
carried-forward constraints and historical rationale**. FS01 selected the composed
v1 profile hierarchy instead of enabling `state-finite/1-draft`. This packet is
not an implemented profile, an alias, qualification evidence or a second ledger.

## Selected authority and definition identity

Native Quire is the sole editable formal-clause language. OCL 2.4, SysML v2 and
FRETish are output-only mapping profiles under FS06. Generated text cannot become
a second authority for the same clause. Shared model declarations supply the
types a clause names; they are not another clause language.

The superseded replacement identity was language `ix:native`, edition `0-draft`,
semantic profile `state-finite/1-draft`. It is not registered, implemented,
qualified or available for selection. Its constraints are refined under the
composed `ix:native` / `1-draft` state hierarchy; no frontend may translate this
old tuple into that hierarchy. Exact rational spelling changes the composed
edition contract and therefore does not reinterpret the historical `0-draft`
grammar.

The `proposals/state-core` profile and rule files define `state-finite/0-draft`.
They cannot be used to identify this amendment. Final adoption must select this
definition by identity, plus every inherited rule definition and
its precedence; the resolver derives and records content identity in the lock.
No complete inheritance closure is fixed by this review draft. An unavailable selection refuses; it cannot silently fall back to
0-draft. Historical reports keep their original meaning and identity. Recompiling
under a replacement definition requires a new explicitly bound derivation.

## Grammar admission approach

Retain the native lexer/parser architecture and explicit model imports, named
invariant/pre/post clauses, expression spans and source bytes. Recognition,
profile admission, typing/definedness, runtime validity and backend support are
separate stages. Unsupported constructs remain source-addressable and cannot be
removed by dead-code elimination or hidden by a more capable IR/backend.

Native `present`/guarded `value`, short-circuit Boolean control, `if` and `let`
retain their native definedness/evaluation requirements. Clause grammar and
truth tables are Quire's own.
The carried-forward table below is binding; an external language's punctuation
does not silently redefine a native construct. Exact rational spelling and the
inherited-rule closure are now specified by the composed
[shared grammar](../quire-v1/shared-grammar.md) and
[state contract](../quire-v1/state-contract.md). Their implementation and
qualification remain FS03/backend work; this superseded candidate is not adopted.

## Carried-forward semantic admission

These are frontend rules, independent of general IR and backend capacity.
Every bound originates in an explicit reviewed domain declaration or named
profile parameter. Storage width and observed sample values are not bounds.

| Concern | Replacement candidate rule |
| --- | --- |
| Integers | Inclusive finite endpoints within signed-64 representability; reject overflow. No float, NaN, infinity, saturation, widening or inferred machine-width domain. Contextual literals require one exact expected declared type and must fit it. |
| Exact rationals | Normalize sign and common factors before testing numerator/denominator bounds. Zero has its normalized representation; denominator is nonzero and positive after normalization. Admit only exact bounded values and proved intermediate ranges. No implicit integer-to-rational conversion. Missing implementation remains a gap; refusal is not rational support. |
| Arithmetic | Admit `+`, `-`, `*`, unary numeric negation only with statically proved intermediate ranges. Rational division also requires a proved nonzero divisor. Integer `/`, `div`, `mod` and equivalent `rem` remain deferred, including guarded or unreachable uses. |
| Presence/null/invalid | One option-none channel represents one explicitly declared absence channel. Preserve independently authored absence and explicit null through a qualified encoding or refuse the field. Invalid is a rejected evaluation possibility, never false or option-none. Required member presence and multiplicity remain independent model facts; neither is inferred from an option, sequence bound or IR v1.1 field shape. |
| Containment/equality | Acyclic fully typed records and exact owner/field paths. Scalar, enum and text equality are admitted; record/object identity equality is deferred, without structural-equality substitution. |
| References | Opaque UUIDs may be compared as their declared scalar representation; no scalar-to-object cast, foreign-key navigation, recursive inline parent, ambient object store or allInstances. Neither a supplied reference environment nor a new IR reference type is admitted by this first source profile. The reference design gate below remains open. |
| Collections | Explicitly ordered, duplicate-preserving finite sequences with a declared maximum at most 10000. Empty universal is true and empty existential false; missing input is not an empty sequence. Set, Bag, OrderedSet, unbounded ranges, map/filter/flatten/closure and implicit conversions are deferred. Check nested and unused type declarations as well as values actually reached by a clause. |
| Calls/result | No user-defined calls, recursion, reflection, I/O or ambient extension library. A signature alone supplies no executable semantics. Require exactly a statically defined Boolean root, without nullable/partial results, exception recovery or truthiness coercion. |
| Resources | Charge the selected parser/checker/runtime ceilings at their owning stages. Exhaustion and missing required observations remain incomplete, with no manufactured Boolean. Total evaluation, finite search, decidable validity and practical completion remain different claims. |

The runtime wire's explicit-null optional result selector is not a nullable
clause value. An omitted required wire field is a decoding refusal, not evidence
that independently authored model member presence has been specified. A does
not introduce presence syntax by inference.

## Source authority and evaluation anchors

Each clause has one stable identity and exactly one editable source: a selected
native inline region or a selected local-file region, never both. Bind the
path and the half-open byte span; any content identity is derived by the
resolver into the lock. A floating URL is insufficient. Multi-clause files bind each clause separately. Preserve
source/profile, clause/requirement revision, declaration closure, tool versions and
anchor inputs independently of the source-free semantic identity. Duplicate
authorities, ambiguous names and conflicting declarations refuse
before lowering. Extraction treats the native body as opaque.

| Clause | Exact binding | Ordinary self | Historical-state access |
| --- | --- | --- | --- |
| Invariant | Named initialization or handler observation | Selected current snapshot | Refused |
| Precondition | Named pre anchor of the selected operation | That invocation's pre snapshot | Refused |
| Postcondition | Named post anchor of the same operation | That invocation's post snapshot | That invocation's pre snapshot |

Native `pre(expr)` is the source spelling requiring correspondence with this
table; no `@pre` token is introduced by quoting the external source rule.
Snapshots are supplied immutable values; the frontend reads no application state.
Operation parameters are immutable declared inputs, not state-selected object
fields; result is an explicitly declared post-only input. Missing signatures or
anchors, bare context types, foreign operations, historical-state access on a
parameter and cross-observation guard reuse refuse. The existing 0-draft
`pre(parameter)` rule is therefore not inherited. The exact treatment of aliases
and composite pre expressions is now specified in the composed
[state contract](../quire-v1/state-contract.md). A passing parameter fixture from
`0-draft` is not implementation evidence for that contract.

## Reference and ConfigVersion design gate

The original `parent: ConfigVersion[0..1]` with an Integer having only `min 1`
has neither a finite declared maximum nor an admissible recursive containment
encoding. Do not remove parent, invent a maximum or replace it with an opaque
scalar lookup to make a fixture pass.

The compiler's FR-032 example is the retained worked example, not a second
frontend. Its deliberately authored 0..1000 model, explicit reference roles and
finite supplied population run under the historical native contract. Those
results do not discharge the original reference/bounds gate or demonstrate
replacement-profile conformance. Reference identity/equality/reachability remain
deferred under the carried-forward rule. The relation between those deferrals
and the desired full parent workflow needs explicit owner disposition; this
amendment does not redefine the whole workstream as a scalar-only success.

The recommended finite identity-preserving reference environment and alternative
new IR reference construct remain later design choices, neither assumed here.

## Retained delivery boundaries

[FR-019](../../spec/functional/foundation/FR-019-admit-native-source-profile.md) specifies
observable admission outcomes. Its planned tests
remain obligations for the composed compiler reconciliation, not executed
conformance. Existing tests may be reused only after checking that they exercise
the selected composed rule.

1. Retain the completed [ADR-0053 handoff](adr-0053-handoff.md), the original full
   workflow objective and the reference gate; do not silently narrow either.
2. Implement the composed FS03 rational and anchor contracts, including
   normalization-before-bounds and eligible pre-state controls. Refusal is not
   rational support.
3. Adopt exact composed definition identities at their owning semantic gates,
   then implement each selected source admission at model/checking boundaries.
4. Exercise positive, adverse and incomplete cases with actual Rust consumers;
   retain failed/unavailable stages. No schema, authored example or old green
   suite establishes full native correspondence. Independent review and later
   assurance remain distinct from implementation delivery.

OCL/SysML/FRETish exports remain wanted Rust implementations, with source-bound
preserved/conditional/unrepresented/refused loss reports. Parsing emitted text
does not prove equivalence. No Java/JVM/Maven/Eclipse/Electron production or
qualification dependency is authorized; any such dependency requires its own
owner decision. Temporal remains behind a separately specified tl-* bridge.
The SysML pilot license correction remains LGPL-3.0-or-later. Standard
artifact licensing and public release are still separate decisions.
