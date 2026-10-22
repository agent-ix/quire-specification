# Native diagnostic interpretation candidate

Interpretation identity: `quire.native.diagnostics/v1`.
**The interpretation role is accepted by FS01; this catalog remains
proposed and is not adopted or fully implemented.** This supplies the catalog
required by FR-047. It interprets native producer diagnostics in their existing
result envelopes; it is not a clause profile, payload wire or evidence store.

## Selection

The producer response selects this catalog's exact identity separately from its language/profile, payload wire and backend selections.
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
A stage binds the exact selected producer-interface identity plus one exact
label from that interface's closed stage vocabulary. A wrapper
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
| `unsupported_construct` | `declaration-form`, `expression-form` or `not-yet-implemented`; retain the recognized form, owning declaration and exact selected profile that prohibits it, or, for `not-yet-implemented` (a valid complete-V1 source form or declaration input that the producer's support set does not build; the source is not wrong), the unbuilt feature's name, owning declaration and exact selected profile. Presence in unreachable syntax does not erase the refusal. |
| `unknown_language`, `unknown_edition`, `unknown_profile` | `unsupported-selection` or `wrong-selection-role`; retain the supplied selection and required role. A known package/diagnostic interpretation used as a clause profile is the latter. |
| `unknown_wire` | `unsupported-wire`; retain the actual selected wire and expected artifact role, even if its outer reference envelope is well formed. |
| `unknown_required_feature` | `unknown-feature` or `unsupported-feature`; retain the feature and selected consumer/profile scope. A recognized name is not implementation support. |
| `missing_import`, `missing_declaration` | `missing-selection` or `missing-name`; retain the requested typed dependency/name, owning scope and referring locus. During model intake, `missing-selection` retains the ModelSelection, and `missing-name` retains the referring IR node identity, the absent node identity and the referring node's FCD `origin`. |
| `stale_dependency`, `source_digest_mismatch` | `byte-digest-mismatch`, `digest-domain-mismatch` or `content-mismatch`; retain expected and actual typed selections/digest domains where available. `content-mismatch` is an expected content identity that differs from the actual one: a content identity a request retains that differs from the identity its subject carries, such as a node identity taken from another package, or, in replay byte provision (FR-323), a `sha256-jcs` domain-package document whose digest differs from the digest it is keyed by, or a pinned or declared `package_id` that differs from the one recomputed from the compiled source, as in dependency selection and replay (FR-322, FR-323); it retains both identities. `byte-digest-mismatch` is a raw-bytes digest that differs from the digest its selection or reference declares. A missing artifact cannot supply an invented actual digest. |
| `ambiguous_declaration` | `ambiguous-name` or `conflicting-authority`; retain the visible name/clause and the distinct conflicting declaration/authority selections and loci. A `quire.checked-package/v2` reader reports two nodes carrying equal `declaration.qualified_name` values as `ambiguous-name`, retaining that name and both node keys in ascending digest order. |
| `invalid_package` | `malformed-json`, `missing-member`, `duplicate-member`, `unknown-member`, `wrong-value-kind`, `invalid-value`, `definition-cycle`, `conflicting-definition`, `incompatible-definition`, `feature-set-mismatch`, `unknown-operation`, `operation-class-mismatch`, `operation-law-missing`, `operation-law-mismatch`, `operation-law-unselected`, `operation-mode-mismatch`, `operation-member-mismatch`, `operation-mode-type-mismatch`, `stale-node-key` or `declaration-nominal-mismatch`; retain the actual field/index path or dependency edges/feature sets appropriate to that variant. Under `quire.model.complete/v1`, `definition-cycle` also covers a call-graph cycle through a dispatch edge (FR-151) and retains the cycle's call edges in the FR-151 order. The eight operation variants retain the application node key, the pre-order ordinal of the application inside that node, the leaf path (empty for the operation itself) and the supplied operation identity and operator class, plus: `unknown-operation` nothing further; `operation-class-mismatch` the catalogued operator class; `operation-law-missing` the required law role; `operation-law-mismatch` the supplied law and the catalogued `DefinitionRef` set of its role; `operation-law-unselected` the unselected law `DefinitionRef`; `operation-mode-mismatch` the required mode kind (or none) and the supplied mode; `operation-member-mismatch` the required member kind (or none) and the supplied member; `operation-mode-type-mismatch` the supplied mode value and the set of mode values pinned by the operand and result types, all as published by `contracts/checked-operation-catalog-v1.json` in `agent-ix/quire-verification-contracts`. For a clause application (`temporal`, `protocol_control`, `state_transition`, `claim`) the law role is the profile-selection role the catalog names (`temporal_profile` or `protocol_profile`), the catalogued `DefinitionRef` set is the package lock's `profile_selections` of that role, and `operation-member-mismatch` for a `profile_operator` member also retains the operator set the selected profile defines. `stale-node-key` retains the application node key as retained, the recomputed `quire.application-node/v1` digest and the node's ordinal in the package graph. `declaration-nominal-mismatch` retains the node key, its `declaration.qualified_name` and its nominal `qualified_declaration`. |
| `invalid_model_binding` | `wrong-artifact-role`, `wrong-model-selection`, `wrong-export`, `wrong-context`, `conflicting-binding`, `unpreserved-model-meaning`, `specialization-cycle`, `redefinition-target`, `derivation-conflict`, `malformed-declaration` or `port-direction`; retain the required/actual declaration or contract, referring location and affected meaning. Missing versus explicit null may not be collapsed into a generic absent value. `wrong-export` for a systems-model kind retains the required kind, the actual kind (`none` when the declaration maps to no kind) and both declaration keys. `specialization-cycle` retains the cycle's type declaration keys in path order. `redefinition-target` retains every redefining member's declaration key and the zero or several resolved targets. `derivation-conflict` retains both derivation paths and exposes no chosen effective member. `malformed-declaration` retains the IR node identity, the member path that is invalid and the node's FCD `origin`. `port-direction` retains the connection's declaration key, its declared `direction` and both ports' directions. Every model intake and model feature refusal also retains the IR node identity it names and that node's FCD `origin` verbatim, with no span for a generated origin. |
| `ill_typed` | `type-mismatch`, `unit-mismatch`, `operator-ineligible`, `ambiguous-literal`, `non-boolean-root`, `variance-parameter`, `variance-result`, `multiplicity-narrowing`, `effect-escape`, `subsetting-type` or `identity-observing-form`; retain the operator/input locus and expected/actual named type, unit or kind. `identity-observing-form` retains the breaking node, its form (order-sensitive traversal, reference conversion or key literal) and the declaration key of the `symmetric` population whose identity it observes (FR-381). The five conformance causes retain the redefining and redefined effective member identities, the axis (parameter index for `variance-parameter`), the expected and actual effective type, typed multiplicity or effect entry, and every contributing declaration key in effective-identity order. |
| `undefined_expression` | `unproved-range`, `unproved-nonzero`, `unproved-presence`, `unproved-decrease`, `unproved-refinement`, `unproved-exhaustiveness` or `undefined-evaluation`; retain the potentially evaluated operation, exact operand/read identity and applicable guard/anchor context, and for `unproved-decrease` the recursive component cycle, call edge and failed measure obligation. `unproved-refinement` retains the redefining field's effective identity, the writing operation's effective identity and the obligation `field-domain`, `field-presence` or `no-proof-form`. `unproved-exhaustiveness` retains the scrutinized `union-decl`'s declaration identity, the `case` expression's arm set and the failed obligation `duplicate-arm`, `unknown-member`, `arm-arity` or `missing-arm`. `undefined-evaluation` is the cause of a proof engine's `refuted` settlement, not a refusal: a claim evaluated undefined at a reached evaluation point (FR-363); it retains `where`: the evaluation point (a model state and position, a tuple of behaviours and positions, a sample and position, or a timed state) and the source locus of the expression that has no value; and `cause`: the undefined value's catalogued reason. Failure to prove definedness is not logical violation. |
| `ambiguous_dispatch` | `multiple-undominated` or `no-applicable`; retain the called effective operation, the closed subtype for which selection fails, every candidate effective method identity and, for `multiple-undominated`, every dominance pair among the candidates. It is a link-time refusal; no source or registration order resolves it. |
| `cardinality_out_of_bound` | `below-minimum` or `above-maximum`; retain the collection type, its inclusive bound, the formed occurrence or member count and the constructing or converting expression locus, or, for a supplied snapshot or input collection value, its actual input path. It is a runtime refusal of a known out-of-bound collection value, not a type error or resource exhaustion. |
| `wrong_snapshot` | `wrong-observation`, `wrong-invocation`, `wrong-anchor` or `forbidden-pre-read`; retain required and supplied observation/invocation/anchor selections, or the exact prohibited read. |
| `invalid_runtime_input` | `malformed-json`, `missing-member`, `duplicate-member`, `unknown-member`, `wrong-value-kind`, `invalid-value`, `wrong-role-mapping`, `conflicting-role-mapping`, `absent-key`, `abstract-instance`, `subsetting-violation` or `conflicting-identity`; retain the actual input path, owning required role and supplied mapping when relevant. `absent-key` retains the population binding, the requested key and the query locus. `abstract-instance` retains the member record and its abstract type's declaration key. `subsetting-violation` retains the subsetting field's declaration key, the object and the values outside the subsetted feature. `conflicting-identity` retains both member records sharing one universe and object identity. `duplicate-member` on a supplied `Set` or `OrderedSet` also retains the input path of the earlier equal element. A missing observation under a valid mapping is a different code. |
| `unavailable_observation` | `missing-required-artifact`, `missing-required-valuation` or `incomplete-scope`; retain the required role/scope, selected input and exact missing premise. |
| `incomplete_population` | `missing-required-artifact`, `missing-required-valuation`, `incomplete-scope` or `unclosed-subtypes`; retain the required role/scope, selected input and exact missing premise. `unclosed-subtypes` retains the ModelSelection, the population declaration key, the queried type and every conforming effective type that the population's member types do not cover. Unknown membership is not an empty or complete population. |
| `foreign_reference` | `foreign-universe`, `foreign-model-selection` or `foreign-type`; retain the required and supplied object universe, ModelSelection key or effective type identity, respectively, and the operation locus. No cause is decided from an authority, because a reference key carries none. |
| `dangling_reference` | `absent-target-in-complete-population`; retain the reference identity, exact required universe and admitted completeness selection. Without that complete population, use the incomplete-population outcome instead. |
| `population_delta_mismatch`, `frame_violation` | `delta-disagreement` or `unauthorized-change`, respectively; retain the invocation, pre/post selections, affected object/field or State root, and the declared delta/frame permission. |
| `resource_exhausted` | `insufficient-next-charge`, `allocation-failed`, `reservation-size-overflow`, `intake-limit-exceeded` or `input-bytes-exceeded`; for `insufficient-next-charge`, retain `stage`: the stage label that the denying limits type assigns to all its counters (`model-normalization` for every `ModelNormalizationLimitsV1` counter, `population-admission` for every `PopulationAdmissionLimitsV1` counter, wherever the charge occurs; not the compiler stage S1-S4 or a checking sub-stage), a charge-record label that is a payload field independent of the diagnostic's FR-273 producer stage, `accounting_contract`: the accounting contract, `limit_kind`: the denying record's own `limit_kind`, unchanged (for example `ancestor_steps`, `family_steps`, `work_units` or `integer_bits`), one exact string, `limit`: that counter's effective limit, `consumed`: prior usage, `next_charge`: the charge that could not proceed and `charge_point`: the charge point that denied it. For `allocation-failed`, retain `requested`: the exact upstream `<unit>:<amount>` rendering (`bytes:<amount>` or `additional-elements:<amount>`) for a genuine allocator denial of a representable request; preserve the measured amount and native unit without conversion, inference or substitution. For `reservation-size-overflow`, retain `upstream`: the exact upstream `StorageRequest` amount and unit and its original capacity/size arithmetic or representation overflow reason; this is not a measured allocator-denied request, and no such measurement is fabricated. For `intake-limit-exceeded`, a size ceiling rather than a charge, retain `kind`: the limit kind as its cause-tag spelling (`input-bytes-exceeded`), `bound`: its configured bound, `actual`: the measured size that exceeded it and `setting`: the limit's FR-461 setting name (QSL FR-260 item 5 / FR-260-AC-4, repo agent-ix/quire-spec-language). For `input-bytes-exceeded`, the format output byte budget (`format.output_bytes`; QSL FR-255 Behavior 1 and FR-255-AC-1: format maps to `input-bytes-exceeded`, intake to `intake-limit-exceeded`), retain `kind`: `input-bytes-exceeded`, `bound`: its configured bound, `actual`: the measured size that exceeded it and `setting`: the limit's FR-461 setting name. These causes remain distinct; a maximum fixed by the language is not a caller work budget (QSL FR-082-AC-10..11; TC-914 steps 2 and 4–5). |
| `cancelled` | `caller-cancelled`; retain `stage`: the stage label that the cancelled work's limits type assigns (as for `resource_exhausted`), `accounting_contract`: the accounting contract, `consumed`: prior usage, `charge_point`: the charge point at which work stopped and `related`: the cancellation source. |
| `duplicate_selection` | `duplicate-identity`; retain the repeated domain-package `identity` and both the already-selected and newly-requested `digest`. A package selects at most one domain package per identity (QSL ADR-013 O-01, QC-5). |
| `conflicting_declaration` | `duplicate-declaration`; retain `subject`: the repeated resolved declaration identity, whatever its declaration kind (the operation identity or `DeclarationKey` that two operation bodies resolve to after local aliases are resolved, or the name two tasks share) and `related`: both declarations, each with its label span, in source-unit order. Raised at S3 when two declarations that must be unique resolve to one identity (QSpec FR-151, QSL FR-302); no partial package or dispatch table is produced. |
| `stage_limit_exceeded` | `input-bytes-exceeded`, `token-count-exceeded`, `node-count-exceeded`, `edge-count-exceeded`, `occurrence-count-exceeded`, `diagnostic-count-exceeded`, `work-budget-exceeded` or `rational-bits-exceeded`; retain `kind`: the exceeded limit's kind, as one exact cause-tag spelling of this row (for example `input-bytes-exceeded` or `node-count-exceeded`), `bound`: its configured bound, `actual`: the actual counter, `setting`: the one setting name that raises the limit at every entry point (QSpec FR-461, for example `s3.nodes`), and the source location where it was reached, when known. No cause names a depth: depth is never a limit kind (QSpec FR-460). S1's token ceiling (a retained CST leaf under complete V1) carries `token-count-exceeded`. The S3 exact-value ceiling (the bit length of a written rational, decimal or quantity magnitude, and of a distribution total) carries `rational-bits-exceeded`, with `setting` `s3.rational_bits`. The I2 reader's wire-byte, semantic graph node, graph dependency-edge, combined semantic occurrence and source-map entry, diagnostic-entry and semantic-term validation-work limits carry `input-bytes-exceeded`, `node-count-exceeded`, `edge-count-exceeded`, `occurrence-count-exceeded`, `diagnostic-count-exceeded` and `work-budget-exceeded`, respectively. The FR-330 ContractPackage reader reports a limit stop as `stage_limit_exceeded` at stage `package_read`, with the cause its incomplete `limit_kind` selects one-to-one: `bytes` carries `input-bytes-exceeded`, `nodes` `node-count-exceeded`, `edges` `edge-count-exceeded`, `occurrences` `occurrence-count-exceeded` and `work` `work-budget-exceeded`; its `limit` is the cause's `bound`, its `consumed` the `actual`, its `path` the location (FR-274), and `setting` is `CheckedPackageReadLimits.<limit_kind>`, the member name of the reader's limits argument and not an FR-461 dotted setting (FR-330). It reports no `diagnostic-count-exceeded`: a ContractPackage has no diagnostics collection. This reader is a separate surface, not a T-4 `LimitExceeded` producer. The observation document reader's objects-per-document and values-per-document limits (`observation.objects`, `observation.values`) carry `node-count-exceeded`, with the same retained `kind`, bound, actual counter and `setting`. This is a `LimitExceeded` limit kind T-4 defines for compiler stages S1-S4, the I2 reader, the observation document reader, a family `check`, the layer-6 `replay` facade and the layer-R `route` module (QSL ADR-013 T-4); it is distinct from `resource_exhausted`'s caller work-budget meter, not from any one of these surfaces. |
| `noncanonical_wire` | `inexact-integer` or `inexact-number`; retain `document_pointer`: the RFC 6901 pointer of the first such number in document order, each member name escaped (`~` as `~0`, `/` as `~1`) and each array element named by its decimal index. `inexact-integer` is a number whose text denotes a whole value of magnitude above 2^53 (9007199254740992), however it is spelled; `inexact-number` is any other number whose text's exact decimal value differs from the exact value of the shortest round-trip text of its nearest IEEE 754 double. RFC 8785 writes a number as that text, so a reader that carries a document's numbers through RFC 8785 cannot carry either number exactly. A reader that requires canonical bytes, as the checked-package V2 reader does (FR-322), refuses any other bytes with this code as a refusal about the byte stream, carrying neither cause nor `document_pointer`. |
| `unknown_contract_version` | No cause variant; the code is the whole classification, as for `noncanonical_wire`; retain `locus`: the `CheckedNodeId` of the node at which the reader reached it, and `document_pointer`, each when the reader has one. `unknown_contract_version` retains `contract_version`: the received version string, exact and unnormalized. |
| `malformed_wire` | No cause variant; the code is the whole classification, as for `noncanonical_wire`; retain `locus`: the `CheckedNodeId` of the node at which the reader reached it, and `document_pointer`, each when the reader has one. |
| `duplicate_member` | No cause variant; the code is the whole classification, as for `noncanonical_wire`; retain `locus`: the `CheckedNodeId` of the node at which the reader reached it, and `document_pointer`, each when the reader has one. |
| `unknown_member` | No cause variant; the code is the whole classification, as for `noncanonical_wire`; retain `locus`: the `CheckedNodeId` of the node at which the reader reached it, and `document_pointer`, each when the reader has one. |
| `digest_domain_mismatch` | No cause variant; the code is the whole classification, as for `noncanonical_wire`; retain `locus`: the `CheckedNodeId` of the node at which the reader reached it, and `document_pointer`, each when the reader has one. |
| `unknown_required_capability` | No cause variant; the code is the whole classification, as for `noncanonical_wire`; retain `locus`: the `CheckedNodeId` of the node at which the reader reached it, and `document_pointer`, each when the reader has one. |
| `invalid_semantic_graph` | No cause variant; the code is the whole classification, as for `noncanonical_wire`; retain `locus`: the `CheckedNodeId` of the node at which the reader reached it, and `document_pointer`, each when the reader has one. |
| `unsupported_node_tag` | No cause variant; the code is the whole classification, as for `noncanonical_wire`; retain `locus`: the `CheckedNodeId` of the node at which the reader reached it, and `document_pointer`, each when the reader has one. |
| `unsupported_projection` | `unsupported-requested-capability`, `unbounded-extent`, `tool-unavailable` or `missing-fairness-premise`; retain the exact admitted subject, requested claim/target and selected backend. `unsupported-requested-capability` also retains the item's FR-290 capability kind and any named backend when its candidate set is empty, or the obligation form the one candidate's negotiation arm does not discharge. `unbounded-extent` retains the capability kind, the candidate and its advertised modes for an unbounded extent on a bounded-only advertisement with no finite bound available. `tool-unavailable` retains the capability kind and the routed backend; it accompanies FR-331 result `unsupported` when the probe finds the tool unavailable, and result `failed` when the tool becomes unavailable after a passing probe. `missing-fairness-premise` retains the temporal item and the first constraint of its fairness set, for a clause with a fairness constraint evaluated over a supplied trace that carries no enabledness (FR-362). It cannot reclassify a valid source as malformed. |
| `invalid_capability` | `absent-kind`, `unknown-kind`, `unknown-vocabulary`, `unknown-mode`, `duplicate-backend`, `absent-extent`, `unknown-backend`, `inconsistent-candidates`, `ambiguous-backend`, `absent-evidence-kind`, `unknown-evidence-kind`, `invalid-domains`, `invalid-fairness-kinds` or `not-statistical`; retain the request item index, for a registration cause the backend identity, and for `unknown-vocabulary` the refused carrier. `unknown-kind` and `unknown-mode` retain the exact supplied value; `unknown-vocabulary` retains the supplied vocabulary identity, or its absence, and refuses the whole carrier; `duplicate-backend` retains the repeated backend identity and the refused registration's manifest; `unknown-backend` retains the named unregistered backend identity, or the registered backend with no negotiation arm; `inconsistent-candidates` retains every offending candidate; `ambiguous-backend` retains every candidate in candidate order; `invalid-fairness-kinds` retains the backend identity and the defect (`absent`, `not-a-list`, `unknown-label` or `duplicate-label`), with the member as supplied for `not-a-list` and the offending element for `unknown-label` and `duplicate-label`; `not-statistical` retains the request item index and the named method, and settles the item `invalid-request` at negotiation (FR-408). No cause maps a refused label to an FR-290 member. |
| `invalid_probabilistic_model` | `random-read-in-precondition`, `repeated-support-value`, `duplicate-distribution`, `invalid-weight`, `value-outside-type`, `state-read-in-reward`, `reward-type`, `negative-reward`, `incomplete-workload`, `duplicate-weight`, `unknown-operation`, `delay-on-untimed-model`, `foreign-workload`, `fairness-under-workload`, `qualitative-threshold`, `partial-confidence`, `duplicate-reward`, `invalid-confidence`, `quantile-over-fraction`, `fraction-bound-out-of-range`, `unbounded-event`, `not-state-predicate`, `trace-subject`, `over-not-object-type` or `not-temporal-profile`; retain the declaration or claim locus, the operation, parameter, reward, workload, profile, bound, operand or confidence parameter named, and the offending value, argument vector or read (FR-405, FR-407). |
| `invalid_timed_model` | `duplicate-time-source`, `clock-without-dense-time`, `clock-write`, `clock-read-outside-constraint`, `clock-comparison`, `negative-clock-constant`, `time-invariant-shape`, `urgent-guard-reads-clock`, `model-time-on-untimed-model`, `invalid-interval`, `invalid-task-set` or `rate-one-hybrid`; retain the declaration locus, the clock, constraint, interval, task or hybrid variable named, and the offending value (FR-415, FR-416, FR-419, FR-193, FR-255). |
| `invalid_result` | `measured-with-truth`, `missing-member` or `unknown-cause`; retain the result item index and the offending member, and for `unknown-cause` the received spelling of the result cause or proof basis (FR-331, FR-409, FR-410). |
| `projection_binding`, `invalid_projection_correspondence` | `upstream-refusal` or `correspondence-loss`, respectively; retain the originating structured IR/mapping cause and exact source/derived identities. |
| `extraction-requires-run`, `extraction-package-conflict`, `extraction-clause-count` | Distinct `wrong-command-mode`, `conflicting-compilation-inputs` and `binding-cardinality` variants, respectively; retain the requested mode, conflicting selectors or actual count. These three refusals cannot share only an English distinction. |
| `invalid_source_identity` | `blank-label` or `empty-path`; `blank-label` retains `label`: the first of the source labels `authority` and `identity`, in that order, whose value is empty or consists only of Unicode `White_Space` scalars; `empty-path` retains nothing further. The label check runs first, over the labels in that order, and the first blank label decides `blank-label`; `empty-path` is reported only when every label is non-blank and the source path is empty. Neither cause names a source region. |
| `inexact_decimal` | `nonzero-discarded-digit`; retain `expected`: the target type's declared domain, whose maximum scale is the rounding step's target scale (an integer target has scale zero). Strict `exact` met a rounding step: at least one discarded digit is nonzero. No rounded or exact value is retained. |
| `decimal_out_of_domain` | `outside-domain`; retain `expected`: the target `Decimal[lo, hi; smin, smax]` domain whose membership the result fails. No coefficient or scale of the result is retained. |
| `division_out_of_domain` | `quotient-outside-domain` or `remainder-outside-domain`; retain `expected`: the bounded consumer's integer domain. Under the selected integer-division profile, `quotient-outside-domain` is a `div` whose quotient fails membership and `remainder-outside-domain` a `rem` whose remainder fails it. Only the member the operator exposes is checked; the other member is never checked and cannot refuse. No member is retained. |
| `modulo_out_of_domain` | `outside-domain`; retain `expected`: the bounded consumer's integer domain the Euclidean `mod` remainder fails. No remainder is retained. |
| `text_length_out_of_domain` | `outside-domain`; retain `expected`: the declared `Text[min, max; profile]` bounds and profile whose length the result fails. No text is retained. |
| `integer_out_of_domain` | `outside-domain`; retain `expected`: the target `Int[lo, hi]` domain an exact conversion or arithmetic result fails. No integer is retained. |
| `rational_out_of_domain` | `outside-domain`; retain `expected`: the `Rational[lo, hi; dmin, dmax]` result domain an exact rational arithmetic result fails. No numerator or denominator is retained. |
| `ieee_not_exact` | `rounding-required`; retain `expected`: the target IEEE width (`binary32` or `binary64`); and `flags`: the would-be flag set, the flags of `invalid`, `divide_by_zero`, `overflow`, `underflow` and `inexact` the same operation would return under `nearest-even` (FR-148). Strict `exact` met an inexact, overflowing or tiny-and-inexact result; no rounded bits are retained. |
| `ieee_nan_payload_not_representable` | `payload-exceeds-target`; retain `expected`: the explicit width conversion's target IEEE width; and `actual`: its source IEEE width. The NaN's payload is not smaller than the target width's quiet bit; no payload bits are retained. |
| `ieee_rational_out_of_domain` | `outside-domain`; retain `expected`: the grammar-named `Rational[lo, hi; dmin, dmax]` target of an explicit IEEE-to-exact conversion that the reduced rational fails. No numerator or denominator is retained. |
| `runtime_invariant` | `established-invariant-broken`; retain the named violated evaluator/input invariant and originating stage/context; for the invariant `identity-byte-bound-reached` (originating context `population.identity`) also retain `kind`: the encoder limit kind reached, exactly one of `canonical-bytes`, `input-bytes` or `object-bytes`, `bound`: the identity byte bound derived from the population's members (not a configured value) and `required`: the amount the encoding needed. It is a failed evaluation, not a false predicate. |

`ambiguous_dispatch`, `foreign_reference`, `invalid_capability`,
`duplicate_selection`, `conflicting_declaration`, `stage_limit_exceeded`, `noncanonical_wire`,
`inexact_decimal`,
`decimal_out_of_domain`, `division_out_of_domain`, `modulo_out_of_domain`,
`text_length_out_of_domain`, `integer_out_of_domain`, `rational_out_of_domain`,
`ieee_not_exact`, `ieee_nan_payload_not_representable` and
`ieee_rational_out_of_domain` are catalog-owned spellings. Their spelling and
closed causes are selected by this table.

The eight `invalid_package` operation variants, `stale-node-key` and
`declaration-nominal-mismatch` above, the V2 `ambiguous-name` use, and
`unproved-exhaustiveness` of `undefined_expression` cover the checked-package
reader and an unproved FR-146 `case` exhaustiveness obligation over a
`union-decl` scrutinee. A `quire.checked-package/v2` reader emits exactly one
diagnostic for the first refusal it finds, and never a broader code:

1. schema refusals (`invalid_package`, their structural causes);
2. the model selection check of FR-322 "Model-owned members" step 1, over
   the `model_selections` rows in lock order, in FR-154's admission order:
   `digest-domain-mismatch` (`stale_dependency`), `missing-selection`
   (`missing_import`) and `wrong-model-selection` (`invalid_model_binding`); then the first FR-154 intake or
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

`invalid_capability` with its causes, and the `unbounded-extent` and
`tool-unavailable` causes of `unsupported_projection`, cover FR-290
capability-kind admission, candidate-set negotiation and tool absence.

`duplicate_selection` covers a second domain-package model selection of an
already-selected identity (QSL ADR-013 QC-5), and `stage_limit_exceeded`
covers T-4's `LimitExceeded` limit kind, raised from compiler stages S1-S4,
the I2 reader, a family `check`, `replay` and `route`, with one cause per
closed limit kind (QSL ADR-013 QC-11). `runtime_invariant`
also carries a T-4 `InternalFault` raised from compiler stages S1-S4, the I2
reader, `replay` and `route`, and not only from `CheckedPackage::call`'s
failed evaluation.

`stage_limit_exceeded` carries `token-count-exceeded` for S1's token ceiling,
and `edge-count-exceeded`, `occurrence-count-exceeded` and
`diagnostic-count-exceeded` for the I2 reader's graph dependency-edge,
occurrence and source-map, and diagnostic-entry limits (QSL ADR-013 QC-28).

Ten codes cover the evaluator's value refusals, one per refusal: `inexact_decimal` for strict `exact` at a decimal rounding step
(FR-140, FR-142), `decimal_out_of_domain` (FR-140), `division_out_of_domain`
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
a running total is not the expression's result. The causes of
`invalid_source_identity` are closed as `blank-label` and `empty-path`. In these eleven
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
- a label is exactly `authority` or `identity`.

Other retained host/source codes keep their selected broad meaning and original
structured producer cause: I/O error, malformed command, invalid digest/identifier,
output serialization failure, invalid Quire context and invalid source map. This catalog does not interpret their OS or upstream codes as
native subvariants. Readers retain that upstream selection explicitly; unknown
upstream meaning cannot establish a native semantic result.

## Undefined reasons

An `undefined` evaluation outcome of a package selecting `quire.model.complete/v1`,
or of a refinement row admitted under it (FR-375, Row layers),
that names a reason carries exactly one of these closed reasons. A reason is not
a refusal code or cause and does not make the outcome refused.

| Reason | Required payload |
| --- | --- |
| `absent-key` | the population binding, the requested reference key and the `lookup` locus, for `lookup<T>(p, r) absent undefined` with no member of that key (FR-153) |
| `only-not-one` | the population binding, the `only` form's locus and the enclosing refinement row, for an `only(x in P where c: e)` whose matching set is empty or holds two or more objects (FR-376); the reason is the same for both, and the payload carries no match count |
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
