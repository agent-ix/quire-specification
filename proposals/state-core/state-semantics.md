# FS03: finite-state typing and evaluation rules

Status: proposed elaboration, `agent-a-fs03/1-draft`, for review against the
approved implementation target in `profile.md`. This is not an implemented
evaluator or an accepted addition to that profile. The profile file and all
earlier fixture bytes retain their current digests. Accepting these refinements
requires a versioned profile-definition artifact and explicit reference
adoption under FS02/FS05; a digest of the earlier profile file cannot silently
cover a later semantic amendment.

The accompanying `fixtures/typing-cases.json` contains independently authored
judgment examples. Its type environments are hypotheses, not another domain
model wire, a compiled model package, or substitute for the existing #54 adapter.
The actual ConfigVersion producer fixture remains a separate unqualified input.

## 1. Judgments and phase boundaries

Let D be an immutable, resolved declaration environment supplied by the shared
model authority. A identifies the clause, context type, invocation and selected
observation. G maps lexical names to immutable typed values or stable access
paths. F contains observation-qualified presence and numeric range facts.

`D; A; G; F |- e : T ! defined` means e has type T and every potentially
evaluated subexpression is defined under those hypotheses. This judgment is
required for a Boolean clause root. Parsing alone establishes none of D, A, G
or F. Unknown declarations, conflicting exports, unavailable model bindings or
unadmitted features cannot be discharged using runtime samples.

`D; A; G; S; budget |- e => v / events` describes evaluation against validated
immutable snapshots S. It either yields a value, refuses a known invalid input,
or remains incomplete. These are internal judgment alternatives, not a second
portable result schema. B owns their result-envelope representation. A Boolean
logical outcome exists only after successful validation and completed evaluation.

Linking checks all syntax for names, types and admitted features, including
unreachable branches. Definedness obligations are conditional on paths that
can execute. Thus an unreachable unsafe unwrap may be discharged, but an
unreachable unknown name or unsupported operation is still refused.

## 2. Types and values

Use the shared declaration identity for named scalar, enum, record and object
types. Do not invent identities from display names or observed values. The
following notation describes checked views of those declarations:

- Bool; bounded integer `Int(D, lo, hi, unit)`; bounded scalar text; closed enum.
- Acyclic structural record, with declared fields and their types.
- `Option(T)` with distinct absent and present(T) values. A present nullable
  value is not absence; nullable model fields are unsupported in this profile.
- `Seq(T, maxLength)`, retaining order and every repeated occurrence.
- `Ref(T)` and identity-bearing object access to T. Their runtime value retains
  universe, stable type/object identity and observation. Object access is not a
  recursive structural record encoding.

Every integer declaration supplies lo/hi inside signed 64-bit representation.
Every text and sequence declaration supplies a finite authored maximum. The
adapter preserves unit, optionality, multiplicity, ordering and uniqueness or
refuses the declaration; neither machine width nor a sample supplies a bound.

Equality is admitted for Boolean, integer, text, enum, eligible acyclic records,
and identity-bearing objects/references. Record equality compares all fields
only when each field type admits equality. Sequence/option equality is not
implicitly added by their host representations. Ordering initially admits
integers and text; no Boolean or enum ordering is inferred from host ordinals.
Record identity and object identity remain different relations.

## 3. Contextual typing and scalar operations

Field access resolves a declared field identity on its exact receiver type;
operation names and enum variants resolve similarly. There is no implicit
subtyping, cast, record/object coercion, unit conversion or numeric widening.
All nonliteral comparison operands must have equal types and units. Integer
and text literals check against the unique expected declared scalar type and
must fit its bounds. An unconstrained or ambiguous literal type is refused.

Arithmetic uses the same declared integer type for its operands and result.
For example, `self.versionNumber + 1` obtains the literal's type from the field,
then must prove the sum fits that type. Let inference cannot choose the first
compatible imported scalar when several declarations would fit.

Proposed `size` refinement: `size(xs)` checks against an expected declared,
dimensionless integer type whose bounds contain every possible length of xs.
A comparison such as `size(self.items) = self.count` provides that type. Without
a unique context it is underconstrained; do not infer an unbounded or machine
integer. This is an explicit review choice, not a previously qualified size ABI.

Boolean operations require Boolean operands. Integer ordering is mathematical;
text equality and order use exact Unicode scalar sequences (lexicographic order,
shorter equal prefix first), with no normalization or locale-dependent collation.
Like-unit addition/subtraction and comparison are permitted. Multiplication,
division and remainder require dimensionless operands.

## 4. Stable guard facts

A presence fact is `present(K)`, where K identifies the exact immutable optional
value: its snapshot/universe, receiver or immutable lexical binding, and resolved
field path. It is not a stringified expression or merely a field name. Immutable
let aliases may preserve K. Different snapshots never acquire equal K merely
because their source spellings or observed values coincide.

Stable keys are available for anchored field paths and immutable lexical
bindings. A compound optional expression without a stable path must first be
bound with let to obtain a reusable key. The initial checker may refuse an
unproved repeated compound expression; it must not equate it by source text.

The core rules are:

```text
e : Option(T)                         => present(e) : Bool
e : Option(T), present(key(e)) in F    => value(e) : T
r : Ref(T), validated target in S     => deref(r) : object access to T
```

`value(e)` without its presence proof is `undefined_expression`. `deref` is
defined only after snapshot closure and type/universe checks; an absent optional
reference is not a reference and must first be guarded/unwrapped. Equal object
fields cannot establish either identity equality or optional presence.

Let `trueFacts(c,F)` and `falseFacts(c,F)` refine facts for each outcome. A
presence test contributes its exact positive/negative fact; `not` exchanges
outcomes. Comparisons against typed constants refine mathematical ranges, and
inequality to zero can exclude zero. For sequential paths, propagate refinements
in evaluation order. At alternative path joins, retain only facts valid on
every alternative. Do not union mutually exclusive presence facts.

## 5. Branches and sharing

```text
a : Bool; b : Bool under trueFacts(a,F)   => a and b : Bool
a : Bool; b : Bool under falseFacts(a,F)  => a or b : Bool
a : Bool; b : Bool under trueFacts(a,F)   => a implies b : Bool
c : Bool; a : T under trueFacts(c,F);
          b : T under falseFacts(c,F)    => if c then a else b : T
a : T; b : U under G extended by x : T   => let x = a in b : U
```

Evaluate the left operand/condition once. `and` skips its right operand when
false, `or` when true, and `implies` when false, yielding false/true/true
respectively. A value conditional evaluates exactly its selected branch. Let
evaluates the initializer once, then uses an immutable lexical binding; textual
substitution must not duplicate work or change captured snapshot identity.
Duplicate local names and nested shadowing are refused in this initial profile.

Each implication retains its authored source correspondence and separate events
for antecedent reached, antecedent value if completed, and consequent reached.
An implication with a false antecedent has value true and an inactive consequent;
it is not evidence of consequent participation. Exhaustion after a true
antecedent retains that event but yields no completed implication value. A
backend without a corresponding probe reports unavailable coverage rather than
inventing source activation from an optimized Boolean expression.

## 6. Arithmetic definedness

Range obligations use mathematical integers, including during the proof itself.
Given refined operand domains X and Y and declared result interval R, require
`{x op y | x in X, y in Y} subset R`. Unary negation has the analogous obligation.
For division/remainder, additionally prove the divisor is nonzero and each
specified operation is defined. Failure to prove an obligation refuses linking;
it is not a runtime false value or permission to wrap.

The first checker may soundly discharge these obligations using declared bounds,
constant folding and the guard refinements above. It need not solve arbitrary
integer formulas. It must label failure to establish definedness distinctly
from a demonstrated logical counterexample. A stronger prover may be selected
only under an explicit capability/accounting contract.

`div` truncates toward zero. `a rem b = a - (a div b) * b`, so `-7 div 3 = -2`
and `-7 rem 3 = -1`; the remainder has the dividend's sign when nonzero. The
identity defines mathematical remainder, not a lowering that adds unchecked
host intermediates. Signed minimum divided by -1 and a result outside the
declared bounds refuse unless the path's proved domains exclude the case.

## 7. Collections and reachability

`forall(x in xs : p)` and `exists(x in xs : p)` require `Seq(T,N)` and a Boolean
predicate under x : T. Bind each occurrence in sequence order, retaining repeated
values. Stop at the first false/true respectively. Empty forall is true and
empty exists false. Missing sequence data is unavailable input, never empty data.
`collect`, set/bag coercion, flattening, helpers and recursion remain unsupported.
A requested collect of duplicate outputs must be refused, not silently deduplicated.

`reaches(a,b,parent)` requires identity-bearing objects/references with equal
type, universe and selected observation. The named field must resolve to
`Option(Ref(T))` on that type. It follows one or more edges. Expand each identity
at most once, but test a reached target before suppressing repeated expansion.
Thus a self-loop or a cycle back to a satisfies `reaches(a,a,parent)`, while
an isolated object does not. Optional absence ends traversal. Exact termination
uses the validated finite population, not an implicit path-length cutoff.

Object equality compares universe, stable type identity and object identity.
Observation is retained for field reads but excluded from identity comparison
across pre/post of the same declared universe. References from different
universes cannot be compared by coincident object labels. Reachability cannot
mix observations because its edges must belong to one snapshot graph.

## 8. Observation selection

Invariant self selects current; a precondition selects pre; a postcondition
selects post. `result` is post-only and parameters are immutable invocation
values. The proposed initial anchor rule permits `pre(e)` in postconditions,
selecting that invocation's pre observation for context-dependent reads in e.
Other uses refuse at linking rather than guessing a current/pre equivalence.

An already bound lexical value is not reevaluated or retagged by pre. If
`let p = self.parent` captures a post optional reference, `pre(p)` is still that
captured value; `pre(self.parent)` performs a pre read. Likewise deref follows
the reference's own observation. Guard facts follow these value identities,
not a global current-snapshot variable. Nested pre selects the same pre snapshot.

A pre reference can access an object later deleted. It cannot be retagged as a
post reference. A post clause's required self must itself exist in post; an
allowed deletion does not manufacture that context. This limitation is explicit
in the initial operation fixture and would require a separate profile change.

## 9. Validate runtime inputs before evaluation

Validate known structure, byte bindings, identity/type/universe, operation and
invocation correspondence first. Required observations must be available and
declare the required finite populations complete. A completeness declaration is
an assumption about the supplied population, not proof about a deployment.

For complete observations, validate every required field, value bound and
reference target, even if the predicate will skip that field. Duplicate object
identities, wrong types and dangling targets are invalid inputs. Optional
absence, nullable values, missing fields and unavailable observations retain
their distinct meanings. Missing information does not justify fabricated object
deltas or inferred empty populations.

For operations, created/deleted identity sets must be disjoint and equal the
actual population difference. Check every changed field and created/deleted
type against the selected authored frame binding. Callers cannot enlarge that
frame at runtime. Then validate required clause context and evaluate. A known
defect remains a diagnostic if other data is unavailable; no combination yields
a Boolean. Use deterministic diagnostic phase/locus/code order, with related
declaration loci, without claiming that absent facts were inspected.

## 10. Proposed reference accounting

The candidate evaluator accounting is `native-ref-cost/1-draft`, separate from
the semantic profile and backend capability. Input parsing, model linking and
snapshot validation have separate bounded-work policies. No phase may report
success using a partially validated population.

After successful validation, charge one expression step before entering each
evaluated expression node, including lexical reads and each repeated predicate
evaluation. Grouping is transparent; it retains source lineage but adds no step.
Charge one graph-expansion step before expanding a previously unexpanded
identity in reaches. Revisited targets are still tested as specified above.
Check cancellation before work/loop continuation. Skipped branches consume no
expression steps. A let initializer consumes steps once; each use is a lexical
read, not another initializer evaluation.

Insufficient budget or cancellation yields incomplete with no Boolean and only
the events actually observed. Budgets are work ceilings, not authored number,
collection or graph bounds. The zero-budget vector is an authored expectation;
exact nonzero accounting vectors and independent implementation parity are
required before this accounting version is accepted. The let examples propose
five-step completion and four-step exhaustion, but have not been executed by a
reference evaluator. Backend work/fuel must not
be claimed equivalent merely because logical outputs agree on samples.

## Review decisions and remaining implementation

FS03 review must accept or amend the explicit equality/ordering whitelist,
contextual size typing, pre anchor/captured-value behavior, proof strategy and
reference accounting. These resolve gaps in the initial prose; they are not
silent changes to historical profile identity. Model-owner type mapping, source
activation, operation frames and B/C interchange adoption remain prerequisites.
LC02/LC03 should consume this contract only after those reviews, retaining the
existing shared model authority and executable binder. No evaluator or healthy/
violating executable workflow is supplied by this document.
