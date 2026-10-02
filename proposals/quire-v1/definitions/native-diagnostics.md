# Native diagnostic interpretation candidate

Interpretation identity: `quire.native.diagnostics/v1`; revision: `1-draft.8`.
**The interpretation role is accepted by FS01; this semantic revision remains
proposed and is not adopted or fully implemented.** This supplies the versioned catalog
required by FR-047. It interprets native producer diagnostics in their existing
result envelopes; it is not a clause profile, payload wire or evidence store.

## Selection

The producer response selects this catalog's exact identity, revision and byte
digest separately from its language/profile, payload wire and backend selections.
Changing diagnostic wording, display paths or this response catalog does not
change a checked clause's static meaning. A different diagnostic interpretation
does change the response's interpretation/provenance. No clause `using` alias
can select this catalog or acquire source permissions from it.

This catalog selects the native producer code spellings that FR-271 lists,
with their broad meanings, and the typed refinements below.

Hyphenated and underscored spellings stay exact; they are not normalized aliases.
A compiler enum or its `non_exhaustive` annotation cannot extend this
selection. An unknown selection, code or
required variant refuses interpretation of that diagnostic while retaining its
original bytes and available provenance. It cannot be recast as a known failure,
an empty diagnostic list or a successful assessment.

A code plus message does not establish a typed cause by substring matching.

## Common structured context

Every diagnostic retains its observing producer stage and its catalogued code.
A stage binds the exact selected producer-interface identity, revision and digest
plus one exact label from that interface's closed stage vocabulary. A wrapper
retains the originating pair and cause instead of replacing either with its own
display stage. This catalog does not rename historical stage labels or create a
competing global pipeline enum.

Required context is the affected request/subject, exact selections that were
available, original source region and typed input location where known. An
authored source span, a JSON field/index path and a runtime object/value path
remain different location kinds. Unavailable context is explicitly unavailable;
byte zero, an empty path or a name-search match cannot masquerade as a located
failure. Related declarations and upstream diagnostics retain their own exact
source/catalog identities rather than being flattened into a string.

The producer determines code and cause from typed variants at the failing
operation. A cause is a closed discriminated value with the payload required by
its selected variant, not a free-form string map. Serialization emits its stable
tag; formatting/localization reads it and does not select it. Expected/actual
values are typed selections, kinds, domains or locations, not English summaries.

## Required distinguishing causes

These are mandatory refinements when the named failure occurs. A producer cannot
choose a broader listed alternative to discard a distinction that it knows.
The selected producer interface may carry additional nonsemantic explanatory
context, but that context cannot replace a required cause field or variant.

| Code or code family | Closed cause distinction and required payload |
| --- | --- |
| `invalid_syntax` | `unexpected-token`, `unexpected-end`, `invalid-token` or `invalid-escape`; retain the actual token/source span when available and the expected syntactic category. An ungrammatical index/slice is not a recognized profile extension. |
| `unsupported_construct` | `declaration-form` or `expression-form`; retain the recognized form, owning declaration and exact selected profile that prohibits it. Presence in unreachable syntax does not erase the refusal. |
| `unknown_language`, `unknown_edition`, `unknown_profile` | `unsupported-selection` or `wrong-selection-role`; retain the supplied selection and required role. A known package/diagnostic interpretation used as a clause profile is the latter. |
| `unknown_wire` | `unsupported-wire`; retain the actual selected wire and expected artifact role, even if its outer reference envelope is well formed. |
| `unknown_required_feature` | `unknown-feature` or `unsupported-feature`; retain the feature and selected consumer/profile scope. A recognized name is not implementation support. |
| `missing_import`, `missing_declaration` | `missing-selection` or `missing-name`; retain the requested typed dependency/name, owning scope and referring locus. During model intake, `missing-selection` retains the ModelSelection, and `missing-name` retains the referring IR node identity, the absent node identity, the source artifact id and the span. |
| `stale_dependency`, `source_digest_mismatch` | `revision-mismatch`, `byte-digest-mismatch`, `digest-domain-mismatch` or `content-mismatch`; retain expected and actual typed selections/digest domains where available. `content-mismatch` is a content identity a request retains that differs from the identity its subject carries, such as a node identity taken from another package; it retains both identities. A missing artifact cannot supply an invented actual digest. |
| `ambiguous_declaration` | `ambiguous-name` or `conflicting-authority`; retain the visible name/clause and the distinct conflicting declaration/authority selections and loci. A `quire.checked-package/v2` reader reports two nodes carrying equal `declaration.qualified_name` values as `ambiguous-name`, retaining that name and both node keys in ascending digest order. |
| `invalid_package` | `malformed-json`, `missing-member`, `duplicate-member`, `unknown-member`, `wrong-value-kind`, `invalid-value`, `definition-cycle`, `conflicting-definition`, `incompatible-definition`, `feature-set-mismatch`, `unknown-operation`, `operation-class-mismatch`, `operation-law-missing`, `operation-law-mismatch`, `operation-law-unselected`, `operation-mode-mismatch`, `operation-member-mismatch`, `operation-mode-type-mismatch`, `stale-node-key` or `declaration-nominal-mismatch`; retain the actual field/index path or dependency edges/feature sets appropriate to that variant. Under `quire.model.complete/v1`, `definition-cycle` also covers a call-graph cycle through a dispatch edge (FR-151) and retains the cycle's call edges in the FR-151 order. The eight operation variants (revision `1-draft.4`) retain the application node key, the pre-order ordinal of the application inside that node, the leaf path (empty for the operation itself) and the supplied operation identity and operator class, plus: `unknown-operation` nothing further; `operation-class-mismatch` the catalogued operator class; `operation-law-missing` the required law role; `operation-law-mismatch` the supplied law and the catalogued `DefinitionRef` set of its role; `operation-law-unselected` the unselected law `DefinitionRef`; `operation-mode-mismatch` the required mode kind (or none) and the supplied mode; `operation-member-mismatch` the required member kind (or none) and the supplied member; `operation-mode-type-mismatch` the supplied mode value and the set of mode values pinned by the operand and result types, all as published by `contracts/checked-operation-catalog-v1.json` in `agent-ix/quire-verification-contracts`. For a clause application (`temporal`, `protocol_control`, `state_transition`, `claim`) the law role is the profile-selection role the catalog names (`temporal_profile` or `protocol_profile`), the catalogued `DefinitionRef` set is the package lock's `profile_selections` of that role, and `operation-member-mismatch` for a `profile_operator` member also retains the operator set the selected profile defines. `stale-node-key` retains the application node key as retained, the recomputed `quire.application-node/v1` digest and the node's ordinal in the package graph. `declaration-nominal-mismatch` retains the node key, its `declaration.qualified_name` and its nominal `qualified_declaration`. |
| `invalid_model_binding` | `wrong-artifact-role`, `wrong-model-selection`, `wrong-export`, `wrong-context`, `conflicting-binding`, `unpreserved-model-meaning`, `specialization-cycle`, `redefinition-target`, `derivation-conflict`, `malformed-declaration` or `port-direction`; retain the required/actual declaration or contract, referring location and affected meaning. Missing versus explicit null may not be collapsed into a generic absent value. `wrong-export` for a systems-model kind retains the required kind, the actual kind (`none` when the declaration maps to no kind) and both declaration keys. `specialization-cycle` retains the cycle's type declaration keys in path order. `redefinition-target` retains every redefining member's declaration key and the zero or several resolved targets. `derivation-conflict` retains both derivation paths and exposes no chosen effective member. `malformed-declaration` retains the IR node identity, the member path that is invalid, the source artifact id and the source span. `port-direction` retains the connection's declaration key, its declared `direction` and both ports' directions. Every model intake and model feature refusal also retains the source artifact id and span of the IR node it names. |
| `ill_typed` | `type-mismatch`, `unit-mismatch`, `operator-ineligible`, `ambiguous-literal`, `non-boolean-root`, `variance-parameter`, `variance-result`, `multiplicity-narrowing`, `effect-escape` or `subsetting-type`; retain the operator/input locus and expected/actual named type, unit or kind. The five conformance causes retain the redefining and redefined effective member identities, the axis (parameter index for `variance-parameter`), the expected and actual effective type, typed multiplicity or effect entry, and every contributing declaration key in effective-identity order. |
| `undefined_expression` | `unproved-range`, `unproved-nonzero`, `unproved-presence`, `unproved-decrease`, `unproved-refinement`, `unproved-exhaustiveness` or `undefined-evaluation`; retain the potentially evaluated operation, exact operand/read identity and applicable guard/anchor context, and for `unproved-decrease` the recursive component cycle, call edge and failed measure obligation. `unproved-refinement` retains the redefining field's effective identity, the writing operation's effective identity and the obligation `field-domain`, `field-presence` or `no-proof-form`. `unproved-exhaustiveness` (revision `1-draft.4`) retains the scrutinized `union-decl`'s declaration identity, the `case` expression's arm set and the failed obligation `duplicate-arm`, `unknown-member`, `arm-arity` or `missing-arm`. `undefined-evaluation` is the cause of a proof engine's `refuted` settlement, not a refusal: a claim evaluated undefined at a reached evaluation point (FR-363); it retains `where`: the evaluation point (a model state and position, a tuple of behaviours and positions, a sample and position, or a timed state) and the source locus of the expression that has no value; and `cause`: the undefined value's catalogued reason. Failure to prove definedness is not logical violation. |
| `ambiguous_dispatch` | `multiple-undominated` or `no-applicable`; retain the called effective operation, the closed subtype for which selection fails, every candidate effective method identity and, for `multiple-undominated`, every dominance pair among the candidates. It is a link-time refusal; no source or registration order resolves it. |
| `cardinality_out_of_bound` | `below-minimum` or `above-maximum`; retain the collection type, its inclusive bound, the formed occurrence or member count and the constructing or converting expression locus. It is a runtime refusal of a known out-of-bound collection value, not a type error or resource exhaustion. |
| `wrong_snapshot` | `wrong-observation`, `wrong-invocation`, `wrong-anchor` or `forbidden-pre-read`; retain required and supplied observation/invocation/anchor selections, or the exact prohibited read. |
| `invalid_runtime_input` | `malformed-json`, `missing-member`, `duplicate-member`, `unknown-member`, `wrong-value-kind`, `invalid-value`, `wrong-role-mapping`, `conflicting-role-mapping`, `absent-key`, `abstract-instance`, `subsetting-violation` or `conflicting-identity`; retain the actual input path, owning required role and supplied mapping when relevant. `absent-key` retains the population binding, the requested key and the query locus. `abstract-instance` retains the member record and its abstract type's declaration key. `subsetting-violation` retains the subsetting field's declaration key, the object and the values outside the subsetted feature. `conflicting-identity` retains both member records sharing one universe and object identity. A missing observation under a valid mapping is a different code. |
| `unavailable_observation` | `missing-required-artifact`, `missing-required-valuation` or `incomplete-scope`; retain the required role/scope, selected input and exact missing premise. |
| `incomplete_population` | `missing-required-artifact`, `missing-required-valuation`, `incomplete-scope` or `unclosed-subtypes`; retain the required role/scope, selected input and exact missing premise. `unclosed-subtypes` retains the ModelSelection, the population declaration key, the queried type and every conforming effective type that the population's member types do not cover. Unknown membership is not an empty or complete population. |
| `foreign_reference` | `foreign-universe`, `foreign-model-selection` or `foreign-type`; retain the required and supplied object universe, ModelSelection key or effective type identity, respectively, and the operation locus. No cause is decided from an authority, because a reference key carries none. |
| `dangling_reference` | `absent-target-in-complete-population`; retain the reference identity, exact required universe and admitted completeness selection. Without that complete population, use the incomplete-population outcome instead. |
| `population_delta_mismatch`, `frame_violation` | `delta-disagreement` or `unauthorized-change`, respectively; retain the invocation, pre/post selections, affected object/field or State root, and the declared delta/frame permission. |
| `resource_exhausted`, `cancelled` | `insufficient-next-charge` or `caller-cancelled`, respectively; retain the actual stage, accounting contract, effective limit or cancellation source, prior usage and the work item that could not proceed. A semantic maximum is not a caller work budget. |
| `duplicate_selection` | `duplicate-identity`; retain the repeated domain-package `identity` and both the already-selected and newly-requested `version`. A package selects at most one version of a domain-package identity (QSL ADR-013 O-01, QC-5). |
| `stage_limit_exceeded` | `input-bytes-exceeded`, `nesting-depth-exceeded`, `token-count-exceeded`, `node-count-exceeded`, `edge-count-exceeded`, `occurrence-count-exceeded`, `diagnostic-count-exceeded` or `work-budget-exceeded`; retain the exceeded stage limit kind, its configured bound, the actual counter and the source location where it was reached, when known. S1's token ceiling (a retained CST leaf under complete V1) carries `token-count-exceeded`. The I2 reader's wire-byte, JSON nesting-depth, semantic graph node, graph dependency-edge, combined semantic occurrence and source-map entry, diagnostic-entry and semantic-term validation-work limits carry `input-bytes-exceeded`, `nesting-depth-exceeded`, `node-count-exceeded`, `edge-count-exceeded`, `occurrence-count-exceeded`, `diagnostic-count-exceeded` and `work-budget-exceeded`, respectively. `token-count-exceeded`, `edge-count-exceeded`, `occurrence-count-exceeded` and `diagnostic-count-exceeded` are revision `1-draft.7`. This is a `LimitExceeded` limit kind T-4 defines for compiler stages S1-S4, the I2 reader, a family `check`, the layer-6 `replay` facade and the layer-R `route` module (QSL ADR-013 T-4); it is distinct from `resource_exhausted`'s caller work-budget meter, not from any one of these surfaces. |
| `unsupported_projection` | `unsupported-requested-capability`, `unbounded-extent`, `tool-unavailable` or `missing-fairness-premise`; retain the exact admitted subject, requested claim/target and selected backend. `unsupported-requested-capability` also retains the item's FR-290 capability kind and any named backend when its candidate set is empty, or the obligation form the one candidate's negotiation arm does not discharge. `unbounded-extent` retains the capability kind, the candidate and its advertised modes for an unbounded extent on a bounded-only advertisement with no finite bound available. `tool-unavailable` retains the capability kind, the routed backend and the expected and actual (or absent) tool identity; it accompanies FR-331 result `unsupported` when the probe finds the tool unavailable, and result `failed` when the tool changes after a passing probe. `missing-fairness-premise` retains the temporal item and the first constraint of its fairness set, for a clause with a fairness constraint evaluated over a supplied trace that carries no enabledness (FR-362). It cannot reclassify a valid source as malformed. |
| `invalid_capability` | `absent-kind`, `unknown-kind`, `unsupported-version`, `unknown-mode`, `duplicate-backend`, `absent-extent`, `unknown-backend`, `inconsistent-candidates` or `ambiguous-backend`; retain the request item index, for a registration cause the backend identity, and for `unsupported-version` the refused carrier. `unknown-kind` and `unknown-mode` retain the exact supplied value; `unsupported-version` retains the supplied vocabulary identity, or its absence, and refuses the whole carrier; `duplicate-backend` retains the repeated backend identity and the refused registration's manifest digest; `unknown-backend` retains the named unregistered backend identity, or the registered backend with no negotiation arm; `inconsistent-candidates` retains every offending candidate; `ambiguous-backend` retains every candidate in candidate order. No cause maps a refused label to an FR-290 member. |
| `projection_binding`, `invalid_projection_correspondence` | `upstream-refusal` or `correspondence-loss`, respectively; retain the originating structured IR/mapping cause and exact source/derived identities. |
| `extraction-requires-run`, `extraction-package-conflict`, `extraction-clause-count` | Distinct `wrong-command-mode`, `conflicting-compilation-inputs` and `binding-cardinality` variants, respectively; retain the requested mode, conflicting selectors or actual count. These three refusals cannot share only an English distinction. |
| `invalid_source_identity` | `blank-label` or `empty-path`; `blank-label` retains `label`: the first of the source labels `authority`, `identity`, `revision_namespace` and `revision`, in that order, whose value is empty or consists only of Unicode `White_Space` scalars; `empty-path` retains nothing further. The label check runs first, over the labels in that order, and the first blank label decides `blank-label`; `empty-path` is reported only when every label is non-blank and the source path is empty. Neither cause names a source region. |
| `inexact_decimal` | `nonzero-discarded-digit`; retain `expected`: the target type's declared domain, whose maximum scale is the rounding step's target scale (an integer target has scale zero). Strict `exact` met a rounding step: at least one discarded digit is nonzero. No rounded or exact value is retained. |
| `decimal_out_of_domain` | `outside-domain`; retain `expected`: the target `Decimal[lo, hi; smin, smax]` domain whose membership the result fails. No coefficient or scale of the result is retained. |
| `division_pair_out_of_domain` | `quotient-outside-domain`, `remainder-outside-domain` or `both-outside-domain`; retain `expected`: the bounded consumer's integer domain both members of the `div`/`rem` pair are checked against. The cause names which members fail membership; neither member is retained. |
| `modulo_out_of_domain` | `outside-domain`; retain `expected`: the bounded consumer's integer domain the Euclidean `mod` remainder fails. No remainder is retained. |
| `text_length_out_of_domain` | `outside-domain`; retain `expected`: the declared `Text[min, max; profile]` bounds and profile whose length the result fails. No text is retained. |
| `integer_out_of_domain` | `outside-domain`; retain `expected`: the target `Int[lo, hi]` domain an exact conversion or arithmetic result fails. No integer is retained. |
| `rational_out_of_domain` | `outside-domain`; retain `expected`: the `Rational[lo, hi; dmin, dmax]` result domain an exact rational arithmetic result fails. No numerator or denominator is retained. |
| `ieee_not_exact` | `rounding-required`; retain `expected`: the target IEEE width (`binary32` or `binary64`); and `flags`: the would-be flag set, the flags of `invalid`, `divide_by_zero`, `overflow`, `underflow` and `inexact` the same operation would return under `nearest-even` (FR-148). Strict `exact` met an inexact, overflowing or tiny-and-inexact result; no rounded bits are retained. |
| `ieee_nan_payload_not_representable` | `payload-exceeds-target`; retain `expected`: the explicit width conversion's target IEEE width; and `actual`: its source IEEE width. The NaN's payload is not smaller than the target width's quiet bit; no payload bits are retained. |
| `ieee_rational_out_of_domain` | `outside-domain`; retain `expected`: the grammar-named `Rational[lo, hi; dmin, dmax]` target of an explicit IEEE-to-exact conversion that the reduced rational fails. No numerator or denominator is retained. |
| `runtime_invariant` | `established-invariant-broken`; retain the named violated evaluator/input invariant and originating stage/context. It is a failed evaluation, not a false predicate. |

`ambiguous_dispatch` and `foreign_reference` are catalog-owned spellings of
revision `1-draft.3`; `invalid_capability` is a catalog-owned spelling of
revision `1-draft.5`; `duplicate_selection` and `stage_limit_exceeded` are
catalog-owned spellings of revision `1-draft.6`; and `inexact_decimal`,
`decimal_out_of_domain`, `division_pair_out_of_domain`, `modulo_out_of_domain`,
`text_length_out_of_domain`, `integer_out_of_domain`, `rational_out_of_domain`,
`ieee_not_exact`, `ieee_nan_payload_not_representable` and
`ieee_rational_out_of_domain` are catalog-owned spellings of revision
`1-draft.8`. Their spelling and causes are selected by this table; a producer claiming
revision `1-draft.3` or later emits `ambiguous_dispatch` and
`foreign_reference`, one claiming `1-draft.5` or later emits
`invalid_capability`, and one claiming `1-draft.6` or later emits
`duplicate_selection` and `stage_limit_exceeded`, and one claiming `1-draft.8`
or later emits the ten revision-`1-draft.8` codes, each with these closed causes.

Revision `1-draft.4` adds the eight `invalid_package` operation variants,
`stale-node-key` and `declaration-nominal-mismatch` above, the V2
`ambiguous-name` use, and `unproved-exhaustiveness` of `undefined_expression`
for an unproved FR-146 `case` exhaustiveness obligation over a `union-decl`
scrutinee. A `quire.checked-package/v2` reader emits exactly one
diagnostic for the first refusal it finds, and never a broader code:

1. schema refusals (`invalid_package`, their structural causes);
2. the model selection check of FR-322 "Model-owned members" step 1, over
   the `model_selections` rows in lock order, in FR-154's admission order:
   `digest-domain-mismatch` (`stale_dependency`), `missing-selection`
   (`missing_import`), `byte-digest-mismatch` (`stale_dependency`) and
   `wrong-model-selection` (`invalid_model_binding`); then the first FR-154 intake or
   `quire.model.complete/v1` normalization refusal in FR-154's report order;
3. graph-shape refusals of FR-322 (`invalid_semantic_graph`): an application
   node whose `semantic_form` contradicts its body or whose dependency list is
   not exactly the derived one;
4. `stale-node-key` (`invalid_package`), at the first application node in
   ascending retained node-id digest order whose retained key differs from the
   JCS SHA-256 of its `quire.application-node/v1` preimage;
5. `declaration-nominal-mismatch` (`invalid_package`), nodes in ascending
   node-id digest order;
6. `ambiguous-name` (`ambiguous_declaration`), at the first node in ascending
   node-id digest order whose name an earlier node already declares;
7. operation refusals, nodes in ascending node-id digest order and, inside a
   node, applications in pre-order (an application before its arguments,
   arguments left to right). One application is checked in this order:
   `unknown-operation`, `operation-class-mismatch`, `operation-law-missing`,
   `operation-law-mismatch`, `operation-law-unselected`,
   `operation-mode-mismatch`, `operation-member-mismatch` (all
   `invalid_package`); then every argument that is itself an application,
   directly or as a binder value, left to right under this same order, whose
   first refusal is reported; then `operation-mode-type-mismatch`
   (`invalid_package`) and `operator-ineligible` (`ill_typed`), where a
   model-owned `field` or `operation` member reports FR-322 "Model-owned
   members" steps 2 to 4 in place of `operator-ineligible`:
   `missing-selection` (`missing_declaration`), `stale-node-key`
   (`invalid_package`), `ambiguous-name` (`ambiguous_declaration`) and
   `operator-ineligible` (`ill_typed`), in that order; then its
   `leaves`, each cause over all entries before the next cause:
   `operation-law-missing` (fewer entries than compared leaves),
   `operation-law-mismatch` (extra entries, a path out of order or a law
   outside the role), `operation-law-unselected`, `operation-mode-mismatch`
   and `operation-mode-type-mismatch`. A `dependency_reference` (FR-322) is
   checked at its place in its node's pre-order term walk, wherever it
   stands (an application argument, a binding value, an aggregate member
   or the body root), by `missing-selection` (`missing_declaration`),
   `missing-name` (`missing_declaration`) and `operator-ineligible`
   (`ill_typed`), in that order.

A producer claiming revision `1-draft.3` emits none of these causes.

Revision `1-draft.5` adds `invalid_capability` with its nine causes, and the
`unbounded-extent` and `tool-unavailable` causes of `unsupported_projection`,
for FR-290 capability-kind admission, candidate-set negotiation and tool
absence. A producer claiming revision `1-draft.4` or earlier emits none of
them.

Revision `1-draft.6` adds `duplicate_selection` for a second domain-package
model selection of an already-selected identity (QSL ADR-013 QC-5), and
`stage_limit_exceeded` for T-4's `LimitExceeded` limit kind, raised from
compiler stages S1-S4, the I2 reader, a family `check`, `replay` and `route`,
with one cause per closed limit kind (QSL ADR-013 QC-11). A producer
claiming revision `1-draft.5` or earlier emits neither. `runtime_invariant`
also carries a T-4 `InternalFault` raised from compiler stages S1-S4, the I2
reader, `replay` and `route`, and not only from `CheckedPackage::call`'s
failed evaluation.

Revision `1-draft.7` adds four causes of `stage_limit_exceeded`:
`token-count-exceeded` for S1's token ceiling, and `edge-count-exceeded`,
`occurrence-count-exceeded` and `diagnostic-count-exceeded` for the I2
reader's graph dependency-edge, occurrence and source-map, and
diagnostic-entry limits (QSL ADR-013 QC-28). A producer claiming revision
`1-draft.6` or earlier emits none of them.

Revision `1-draft.8` adds ten codes for the evaluator's value refusals, one
per refusal: `inexact_decimal` for strict `exact` at a decimal rounding step
(FR-140, FR-142), `decimal_out_of_domain` (FR-140), `division_pair_out_of_domain`
and `modulo_out_of_domain` (FR-147), `text_length_out_of_domain` (FR-141),
`integer_out_of_domain` for an integer-target unit conversion (FR-142) and a
bounded integer arithmetic result (FR-044), `rational_out_of_domain` for a
`Rational[..]` arithmetic result (FR-044), `ieee_not_exact` for strict IEEE `exact`,
`ieee_nan_payload_not_representable` for an explicit NaN width conversion, and
`ieee_rational_out_of_domain` for an explicit IEEE-to-`Rational` conversion
(FR-148). Each is category refusal: the operation is defined, and the
selected rounding policy, target width or declared target domain does not
admit its result. None is undefined, incomplete or a type error, and none
retains the refused value. A `sum` seed or running total outside `N`'s domain
is none of these refusals: FR-145 makes it a located undefined outcome, since
a running total is not the expression's result. Revision `1-draft.8` also closes the causes of
`invalid_source_identity` as `blank-label` and `empty-path`. In these eleven
rows, each backticked payload name after the cause list is the exact key of
that payload item in a refusal record's fields, written `key`: before the
item it names, and every one of them
(`expected`, `actual`, `flags` and `label`) is one of FR-272's canonical
payload names: a target domain or width is the expected selection.

Each payload value is written as one exact ASCII string:

- a domain is its type name and bounds, `Int[lo, hi]`,
  `Decimal[lo, hi; smin, smax]`, `Rational[lo, hi; dmin, dmax]` or
  `Text[min, max; profile]`: each bound is written as FR-038 writes an
  integer (a leading `-` when negative, no other sign, leading zero or
  exponent), bounds within a group are separated by `, `, groups by `; `, the
  text profile is its grammar spelling, and no rounding mode is written;
- an IEEE width is `binary32` or `binary64`;
- a flag set is its member flag names in the order `invalid`,
  `divide_by_zero`, `overflow`, `underflow`, `inexact`, joined by `,` with no
  space;
- a label is exactly `authority`, `identity`, `revision_namespace` or
  `revision`.

A producer claiming revision `1-draft.7` or earlier emits none of the ten codes,
and emits `invalid_source_identity` with neither `blank-label` nor
`empty-path`.

Other retained host/source codes keep their selected broad meaning and original
structured producer cause: I/O error, malformed command, invalid digest/identifier,
output serialization failure, invalid Quire context and invalid source map. This catalog does not interpret their OS or upstream codes as
native subvariants. Readers retain that upstream selection explicitly; unknown
upstream meaning cannot establish a native semantic result.

## Undefined reasons

An `undefined` evaluation outcome of a package selecting `quire.model.complete/v1`
that names a reason carries exactly one of these closed reasons. A reason is not
a refusal code or cause and does not make the outcome refused.

| Reason | Required payload |
| --- | --- |
| `absent-key` | the population binding, the requested reference key and the `lookup` locus, for `lookup<T>(p, r) absent undefined` with no member of that key (FR-153) |
| `precondition-false` | the called effective operation, the selected method's effective identity, the receiver reference and the call locus, for a dispatched call whose selected effective precondition is false (FR-151) |

## Outcomes and delivery boundary

The catalog classifies diagnostics, not truth or adequacy. A false evaluated
predicate is a logical result under the selected family contract, not
`undefined_expression` or `runtime_invariant`. Failed prerequisites prevent only
their dependent requests from executing. Resource exhaustion, cancellation and
required missing observations retain incompleteness; known invalidity remains
refused. Retain both facts when they coexist, even when the detail list is capped.
An error-list length cannot decide success. B's result contract still owns
assessment execution, truth, activation, decision support and subject closure.

This definition adds no wire fields to historical payloads and no new production
implementation. The existing compiler/package/output interfaces must specify how
they carry the catalog selection and typed causes under a supported response
wire before claiming this interpretation. FR-047 and TC-047 own the shared
obligation; existing compiler L2/diagnostic work supplies the Rust implementation.
