---
id: FR-272
title: "Typed diagnostic cause"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-047
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-271
    type: references
  - target: ix://agent-ix/quire-specification/FR-274
    type: references
---
# FR-272: Typed diagnostic cause

## Description

When a producer classifies a failure, the producer SHALL carry a closed discriminated cause variant with the payload its variant requires rather than a free-form string map or an English summary.

The proposed catalog states the rule directly: "The producer determines code and
cause from typed variants at the failing operation. A cause is a closed
discriminated value with the payload required by its selected variant, not a
free-form string map. Serialization emits its stable tag; formatting/
localization reads it and does not select it. Expected/actual values are typed
selections, kinds, domains or locations, not English summaries"
([native diagnostics catalog](../../../proposals/quire-v1/definitions/native-diagnostics.md),
"Common structured context"). FR-047 Behavior requires the same discrimination:
"Cause details distinguish materially different refusals sharing a broader
category, including missing versus conflicting definitions, wrong anchor versus
missing observation, unknown syntax versus prohibited recognized construct, and
unproved definedness versus runtime logical violation."

## Properties

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| code_family | String | 1..1 | |
| variant_tag | String | 1..1 | |
| required_payload | String | 1..1 | |
| location | String | 0..1 | |
| referenced_identities | String | 0..* | |

## Where the fields come from

| Field | Source and note |
|---|---|
| `code_family` | [FR-271](FR-271-diagnostic-code-vocabulary.md). The code family whose closed variant set the cause is drawn from. |
| `variant_tag` | proposed catalog, "Required distinguishing causes". The selected variant, one of the closed set for that family. "Serialization emits its stable tag." |
| `required_payload` | proposed catalog, per-family row. The payload "required by its selected variant". Values are "typed selections, kinds, domains or locations, not English summaries". |
| `location` | [FR-274](FR-274-diagnostic-location-kind.md), FR-047-AC-4. "Original source/model/clause loci and identities remain attached where available." |
| `referenced_identities` | proposed catalog. "Related declarations and upstream diagnostics retain their own exact source/catalog identities rather than being flattened into a string." |

The variant sets are **closed and mandatory**: "These are mandatory refinements
when the named failure occurs. A producer cannot choose a broader listed
alternative to discard a distinction that it knows. The selected producer
interface may carry additional nonsemantic explanatory context, but that context
cannot replace a required cause field or variant" (proposed catalog).

## Closed cause variants by code family

Extracted verbatim in substance from the proposed catalog's "Required
distinguishing causes" table. It is proposed, not adopted.

| Code or code family | Closed variants | Required payload |
| --- | --- | --- |
| `invalid_syntax` | `unexpected-token`, `unexpected-end`, `invalid-token`, `invalid-escape` | The actual token/source span when available and the expected syntactic category. |
| `unsupported_construct` | `declaration-form`, `expression-form`, `not-yet-implemented` | For the first two, the recognized form, owning declaration and exact selected profile that prohibits it. For `not-yet-implemented` (a valid complete-V1 source form or declaration input that the producer's support set does not build; the source is not wrong), the unbuilt feature's name, its owning declaration and the exact selected profile. |
| `unknown_language`, `unknown_edition`, `unknown_profile` | `unsupported-selection`, `wrong-selection-role` | The supplied selection and required role. |
| `unknown_wire` | `unsupported-wire` | The actual selected wire and expected artifact role. |
| `unknown_required_feature` | `unknown-feature`, `unsupported-feature` | The feature and selected consumer/profile scope. |
| `missing_import`, `missing_declaration` | `missing-selection`, `missing-name` | The requested typed dependency/name, owning scope and referring locus; during model intake, the ModelSelection, or the referring IR node identity, absent node identity and the referring node's FCD `origin`. |
| `stale_dependency`, `source_digest_mismatch` | `byte-digest-mismatch`, `digest-domain-mismatch`, `content-mismatch` | Expected and actual typed selections/digest domains where available; for `content-mismatch`, the expected content identity and the actual one: the identity a request retains and the one its subject carries, the digest a replay byte-provision domain-package entry is keyed by and the digest recomputed from its document, or a pinned or declared `package_id` and the one recomputed from the compiled source; for `byte-digest-mismatch`, the declared digest and the digest of the supplied raw bytes. |
| `ambiguous_declaration` | `ambiguous-name`, `conflicting-authority` | The visible name/clause and the distinct conflicting declaration/authority selections and loci; a checked-package V2 `ambiguous-name` retains the duplicated `declaration.qualified_name` and both node keys in ascending digest order. |
| `invalid_package` | `malformed-json`, `missing-member`, `duplicate-member`, `unknown-member`, `wrong-value-kind`, `invalid-value`, `definition-cycle`, `conflicting-definition`, `incompatible-definition`, `feature-set-mismatch`, `unknown-operation`, `operation-class-mismatch`, `operation-law-missing`, `operation-law-mismatch`, `operation-law-unselected`, `operation-mode-mismatch`, `operation-member-mismatch`, `operation-mode-type-mismatch`, `stale-node-key`, `declaration-nominal-mismatch` | The actual field/index path or dependency edges/feature sets appropriate to that variant; under `quire.model.complete/v1`, `definition-cycle` also covers a call-graph cycle through an FR-151 dispatch edge and retains its call edges; each of the eight operation variants retains the application node key, the pre-order ordinal of the application in that node, the leaf path (empty for the operation itself) and the supplied operation identity and class, plus the expected class, required law role, supplied law and its role's catalogued definitions, unselected law, required mode kind and supplied mode, required member kind and supplied member, or supplied mode value and the values pinned by the operand and result types, respectively (for a clause application the role is its profile-selection role and the catalogued definitions are the lock profile selections of that role, and a `profile_operator` member mismatch also retains the operators the selected profile defines); `stale-node-key` retains the retained application node key, the recomputed `quire.application-node/v1` digest and the node ordinal; `declaration-nominal-mismatch` retains the node key, its `declaration.qualified_name` and its nominal `qualified_declaration`. |
| `invalid_model_binding` | `wrong-artifact-role`, `wrong-model-selection`, `wrong-export`, `wrong-context`, `conflicting-binding`, `unpreserved-model-meaning`, `specialization-cycle`, `redefinition-target`, `derivation-conflict`, `malformed-declaration`, `port-direction` | The required/actual declaration or contract, referring location and affected meaning; `wrong-export` for a systems-model kind adds required kind, actual kind (`none` for no kind) and both declaration keys; `specialization-cycle` the cycle's type declaration keys in path order; `redefinition-target` every redefining member's declaration key and the resolved targets; `derivation-conflict` both derivation paths; `malformed-declaration` the IR node identity and invalid member path; `port-direction` the connection's declaration key, its declared direction and both port directions; every model intake and model feature variant adds the IR node identity it names and that node's FCD `origin` verbatim (FR-154): a source origin's `sourceIdentity`, `path` and span as given, or a generated origin's `generatorIdentity`, `generatorVersion` and `inputIdentities` as given, with no span; a node whose `origin` is missing or malformed adds none. |
| `ill_typed` | `type-mismatch`, `unit-mismatch`, `operator-ineligible`, `ambiguous-literal`, `non-boolean-root`, `variance-parameter`, `variance-result`, `multiplicity-narrowing`, `effect-escape`, `subsetting-type`, `identity-observing-form` | The operator/input locus and expected/actual named type, unit or kind; `identity-observing-form` adds the breaking node's form and the `symmetric` population's declaration key (FR-381); each conformance variant adds the redefining and redefined effective member identities, the axis (and parameter index), the expected and actual effective type, typed multiplicity or effect entry, and every contributing declaration key in effective-identity order. |
| `undefined_expression` | `unproved-range`, `unproved-nonzero`, `unproved-presence`, `unproved-decrease`, `unproved-refinement`, `unproved-exhaustiveness` | The potentially evaluated operation, exact operand/read identity and applicable guard/anchor context; for `unproved-decrease`, the recursive component cycle, call edge and failed measure obligation; for `unproved-refinement`, the redefining field, the writing operation and the obligation `field-domain`, `field-presence` or `no-proof-form`; for `unproved-exhaustiveness`, the scrutinized union's declaration identity, the `case` expression's arm set and the failed obligation `duplicate-arm`, `unknown-member`, `arm-arity` or `missing-arm`. |
| `ambiguous_dispatch` | `multiple-undominated`, `no-applicable` | The called effective operation, the closed subtype for which selection fails, every candidate effective method identity and, for `multiple-undominated`, every dominance pair among the candidates. |
| `cardinality_out_of_bound` | `below-minimum`, `above-maximum` | The collection type, its inclusive bound, the formed count and the constructing or converting expression locus, or, for a supplied snapshot or input collection value, its actual input path. |
| `wrong_snapshot` | `wrong-observation`, `wrong-invocation`, `wrong-anchor`, `forbidden-pre-read` | Required and supplied observation/invocation/anchor selections, or the exact prohibited read. |
| `invalid_runtime_input` | `malformed-json`, `missing-member`, `duplicate-member`, `unknown-member`, `wrong-value-kind`, `invalid-value`, `wrong-role-mapping`, `conflicting-role-mapping`, `absent-key`, `abstract-instance`, `subsetting-violation`, `conflicting-identity` | The actual input path, owning required role and supplied mapping when relevant; `absent-key` adds the population binding, requested key and query locus; `abstract-instance` the member record and its abstract type's declaration key; `subsetting-violation` the subsetting field's declaration key, object and values outside the subsetted feature; `conflicting-identity` both member records; `duplicate-member` on a supplied `Set` or `OrderedSet` adds the input path of the earlier equal element. |
| `unavailable_observation` | `missing-required-artifact`, `missing-required-valuation`, `incomplete-scope` | The required role/scope, selected input and exact missing premise. |
| `incomplete_population` | `missing-required-artifact`, `missing-required-valuation`, `incomplete-scope`, `unclosed-subtypes` | The required role/scope, selected input and exact missing premise; `unclosed-subtypes` adds the ModelSelection, the population declaration key, the queried type and every uncovered conforming type. |
| `foreign_reference` | `foreign-universe`, `foreign-model-selection`, `foreign-type` | The required and supplied object universe, ModelSelection key or effective type identity, respectively, and the operation locus. |
| `dangling_reference` | `absent-target-in-complete-population` | The reference identity, exact required universe and admitted completeness selection. |
| `population_delta_mismatch`, `frame_violation` | `delta-disagreement`, `unauthorized-change`, respectively | The invocation, pre/post selections, affected object/field or State root, and the declared delta/frame permission. |
| `resource_exhausted` | `insufficient-next-charge`, `allocation-failed`, `reservation-size-overflow`, `intake-limit-exceeded`, `input-bytes-exceeded` | For `insufficient-next-charge`, `stage`: the stage label that the denying limits type assigns to all its counters (`model-normalization` for every `ModelNormalizationLimitsV1` counter, `population-admission` for every `PopulationAdmissionLimitsV1` counter, wherever the charge occurs; not the compiler stage S1-S4 or a checking sub-stage), a charge-record label that is a payload field independent of the diagnostic's FR-273 producer stage, `accounting_contract`: the accounting contract, `limit_kind`: the denying record's own `limit_kind`, unchanged (for example `ancestor_steps`, `family_steps`, `work_units` or `integer_bits`), one exact string, `limit`: that counter's effective limit, `consumed`: prior usage, `next_charge`: the charge that could not proceed and `charge_point`: the charge point that denied it. For `allocation-failed`, `requested`: the exact upstream `<unit>:<amount>` rendering (`bytes:<amount>` or `additional-elements:<amount>`) of a genuine allocator denial of a representable request, the measured amount and native unit preserved without conversion, inference or substitution. For `reservation-size-overflow`, `upstream`: the exact upstream `StorageRequest` amount and unit and its original capacity/size arithmetic or representation overflow reason; this is not a measured allocator-denied request, and no such measurement is fabricated. For `intake-limit-exceeded`, a size ceiling and not a charge, `kind`: the limit kind as its cause-tag spelling (`input-bytes-exceeded`), `bound`: its configured bound, `actual`: the measured size that exceeded it and `setting`: the limit's FR-461 setting name (QSL FR-260 item 5 / FR-260-AC-4, repo agent-ix/quire-spec-language). For `input-bytes-exceeded`, the format output byte budget (`format.output_bytes`; QSL FR-255 Behavior 1 and FR-255-AC-1: format maps to `input-bytes-exceeded`, intake to `intake-limit-exceeded`), `kind`: `input-bytes-exceeded`, `bound`: its configured bound, `actual`: the measured size that exceeded it and `setting`: the limit's FR-461 setting name. |
| `cancelled` | `caller-cancelled` | `stage`: the stage label that the cancelled work's limits type assigns (as for `resource_exhausted`), `accounting_contract`: the accounting contract, `consumed`: prior usage, `charge_point`: the charge point at which work stopped and `related`: the cancellation source. |
| `duplicate_selection` | `duplicate-identity` | The repeated domain-package `identity` and both the already-selected and newly-requested `digest`. |
| `conflicting_declaration` | `duplicate-declaration` | `subject`: the repeated resolved declaration identity, whatever its declaration kind (the operation identity or `DeclarationKey` that two operation bodies resolve to after local aliases are resolved, the name two tasks share, or the dimension name a unit of compilation and a domain package, or two domain packages, both declare) and `related`: both declarations, each with its label span or, for a domain package declaration, its selection row and IR node identity, in source-unit order and then lock order. No partial package or dispatch table is produced. |
| `stage_limit_exceeded` | `input-bytes-exceeded`, `token-count-exceeded`, `node-count-exceeded`, `edge-count-exceeded`, `occurrence-count-exceeded`, `diagnostic-count-exceeded`, `work-budget-exceeded`, `rational-bits-exceeded` | `kind`: the exceeded limit's kind, as one exact cause-tag spelling of this row (for example `input-bytes-exceeded` or `node-count-exceeded`), `bound`: its configured bound, `actual`: the actual counter, `setting`: the limit's FR-461 setting name (for example `s3.nodes`), and the source location where it was reached, when known. No cause names a depth (FR-460). S1's token ceiling carries `token-count-exceeded`. The S3 exact-value ceiling (the bit length of a written rational, decimal or quantity magnitude, and of a distribution total) carries `rational-bits-exceeded`, with `setting` `s3.rational_bits`. The I2 reader's edge, occurrence and source-map, and diagnostic-entry limits carry `edge-count-exceeded`, `occurrence-count-exceeded` and `diagnostic-count-exceeded`, respectively. The FR-330 ContractPackage reader reports a limit stop as `stage_limit_exceeded` at stage `package_read`, with the cause its incomplete `limit_kind` selects one-to-one: `bytes` carries `input-bytes-exceeded`, `nodes` `node-count-exceeded`, `edges` `edge-count-exceeded`, `occurrences` `occurrence-count-exceeded` and `work` `work-budget-exceeded`; its `limit` is the cause's `bound`, its `consumed` the `actual`, its `path` the location (FR-274), and `setting` is `CheckedPackageReadLimits.<limit_kind>`, the member name of the reader's limits argument and not an FR-461 dotted setting (FR-330). It reports no `diagnostic-count-exceeded`: a ContractPackage has no diagnostics collection. |
| `noncanonical_wire` | `inexact-integer`, `inexact-number` | `document_pointer`: the RFC 6901 pointer of the first such number in document order, each member name escaped (`~` as `~0`, `/` as `~1`) and each array element named by its decimal index. `inexact-integer` is a number whose text denotes a whole value of magnitude above 2^53 (9007199254740992), however it is spelled; `inexact-number` is any other number whose text's exact decimal value differs from the exact value of the shortest round-trip text of its nearest IEEE 754 double. The checked-package V2 reader's refusal of non-canonical bytes (FR-322) is about the byte stream and carries neither cause nor `document_pointer`. |
| `unknown_contract_version` | None; the code is the whole classification, as for noncanonical_wire | `locus`: the `CheckedNodeId` of the node at which the reader reached it, and `document_pointer`, each when the reader has one. Also `contract_version`: the received version string, exact and unnormalized. |
| `malformed_wire` | None; the code is the whole classification, as for noncanonical_wire | `locus`: the `CheckedNodeId` of the node at which the reader reached it, and `document_pointer`, each when the reader has one. |
| `duplicate_member` | None; the code is the whole classification, as for noncanonical_wire | `locus`: the `CheckedNodeId` of the node at which the reader reached it, and `document_pointer`, each when the reader has one. |
| `unknown_member` | None; the code is the whole classification, as for noncanonical_wire | `locus`: the `CheckedNodeId` of the node at which the reader reached it, and `document_pointer`, each when the reader has one. |
| `digest_domain_mismatch` | None; the code is the whole classification, as for noncanonical_wire | `locus`: the `CheckedNodeId` of the node at which the reader reached it, and `document_pointer`, each when the reader has one. |
| `unknown_required_capability` | None; the code is the whole classification, as for noncanonical_wire | `locus`: the `CheckedNodeId` of the node at which the reader reached it, and `document_pointer`, each when the reader has one. |
| `invalid_semantic_graph` | None; the code is the whole classification, as for noncanonical_wire | `locus`: the `CheckedNodeId` of the node at which the reader reached it, and `document_pointer`, each when the reader has one. |
| `unsupported_node_tag` | None; the code is the whole classification, as for noncanonical_wire | `locus`: the `CheckedNodeId` of the node at which the reader reached it, and `document_pointer`, each when the reader has one. |
| `unsupported_projection` | `unsupported-requested-capability`, `unbounded-extent`, `tool-unavailable`, `missing-fairness-premise` | The exact admitted subject, requested claim/target and selected backend; for `unsupported-requested-capability`, the item's FR-290 kind and any named backend when the candidate set is empty, or the obligation form the one candidate's negotiation arm does not discharge; for `unbounded-extent`, the kind, the candidate and its advertised modes; for `tool-unavailable`, the kind and the routed backend, with FR-331 result `unsupported` at probe or `failed` after a passing probe; for `missing-fairness-premise`, the temporal item and the first constraint of its fairness set, evaluated over a supplied trace that carries no enabledness (FR-362). |
| `invalid_capability` | `absent-kind`, `unknown-kind`, `unknown-vocabulary`, `unknown-mode`, `duplicate-backend`, `absent-extent`, `unknown-backend`, `inconsistent-candidates`, `ambiguous-backend`, `absent-evidence-kind`, `unknown-evidence-kind`, `invalid-domains`, `invalid-fairness-kinds`, `not-statistical` | The request item index, or the backend identity for a registration cause; the exact supplied kind or mode for `unknown-kind` and `unknown-mode`; the supplied vocabulary identity or its absence, and the refused carrier, for `unknown-vocabulary`; the repeated identity and the refused registration's manifest for `duplicate-backend`; the named unregistered backend, or registered backend with no negotiation arm, for `unknown-backend`; every offending candidate for `inconsistent-candidates`; every candidate in candidate order for `ambiguous-backend`; the request item index and the named method for `not-statistical`; the backend identity and the defect (`absent`, `not-a-list`, `unknown-label`, `duplicate-label`) for `invalid-fairness-kinds`, with the member as supplied for `not-a-list` and the offending element for `unknown-label` and `duplicate-label`. |
| `invalid_probabilistic_model` | `random-read-in-precondition`, `repeated-support-value`, `duplicate-distribution`, `invalid-weight`, `value-outside-type`, `state-read-in-reward`, `reward-type`, `negative-reward`, `incomplete-workload`, `duplicate-weight`, `unknown-operation`, `delay-on-untimed-model`, `foreign-workload`, `qualitative-threshold`, `partial-confidence`, `duplicate-reward`, `invalid-confidence`, `quantile-over-fraction`, `fraction-bound-out-of-range`, `unbounded-event`, `not-state-predicate`, `trace-subject`, `over-not-object-type`, `not-temporal-profile` | The declaration or claim locus, the operation, parameter, reward, workload, profile, bound, operand or confidence parameter named, and the offending value, argument vector or read. |
| `invalid_timed_model` | `duplicate-time-source`, `clock-without-dense-time`, `clock-write`, `clock-read-outside-constraint`, `clock-comparison`, `clock-bound-not-constant`, `negative-clock-constant`, `time-invariant-shape`, `urgent-guard-reads-clock`, `model-time-on-untimed-model`, `invalid-interval`, `invalid-task-set`, `rate-one-hybrid` | The declaration locus, the clock, constraint, interval, task or hybrid variable named, and the offending value. |
| `invalid_result` | `measured-with-truth`, `missing-member`, `unknown-member`, `wrong-value-kind`, `invalid-value`, `certification-mismatch`, `unknown-cause` | The result item index and the offending member as `document_pointer`: its RFC 6901 pointer relative to the entry that carries it, the result entry or the `counterexamples` entry a `refuted` result names, the empty pointer naming the whole entry. `missing-member` names the first absent required member and `unknown-member` the first member the entry's result or spelling does not define, each first in member-name order (RFC 8785). `wrong-value-kind` names a member whose JSON kind is not the kind its domain takes, and `invalid-value` a member of the right kind outside its FR-331 domain. `certification-mismatch` names `/certification` of a `proved` entry whose `certification` is one of FR-331's three values but not the one FR-331 fixes for that proof's basis and backend, with `expected`: that fixed value, absent where FR-331 fixes none, and `actual`: the received value. For `unknown-cause`, a spelling outside FR-331's closed set or, in a plugin's entry, a core-only basis or cause (FR-331), the received spelling of the result cause or proof basis. A reader reports the first defect in FR-331's reading order ("Result entries"). A value inside the subset of FR-331's closed sets that its backend may send that the reader's own types cannot hold is not an invalid result; it refuses `unknown_required_feature`/`unsupported-feature`. |
| `projection_binding`, `invalid_projection_correspondence` | `upstream-refusal`, `correspondence-loss`, respectively | The originating structured IR/mapping cause and exact source/derived identities. |
| `extraction-requires-run`, `extraction-package-conflict`, `extraction-clause-count` | `wrong-command-mode`, `conflicting-compilation-inputs`, `binding-cardinality`, respectively | The requested mode, conflicting selectors or actual count. |
| `invalid_quire_context` | `unsupported-render-form` | `view`: the kind of the view the request renders (`package`, `node`, `outcome`, `counterexample`, `trace` or `state-graph`), and `form`: the requested output form (`json`, `text` or `dot`). A `render` request (FR-300) refuses this way when it asks for a form the view has no rendering in: `dot` for a view other than `trace` and `state-graph`. A form the view has is never refused this way. |
| `unimplemented_capability` | `backend-not-supplied`, `untranslatable-entry`, `unimplemented-meaning` | For `backend-not-supplied`, `backend`: the requested execution backend (`interpreter`, `aot` or `jit`) that is not available to the run, because the caller did not supply it or the producer has not built it. For `untranslatable-entry`, `backend`: the execution backend that cannot run the entry; `construct`: the kind of the entry's first construct, in clause-node order, that the backend cannot translate (`expression-form`, `value-type` or `operation`); `construct_name`: that construct's form name, value type's canonical name or operation node key; and `location`: its source location. For `unimplemented-meaning`, `capability`: the name of the valid complete-V1 meaning the selected producer does not implement. Each is category unsupported. |
| `invalid_source_identity` | `blank-label`, `empty-path` | For `blank-label`, `label`: the first of `authority` and `identity` whose value is empty or only Unicode `White_Space` scalars; for `empty-path`, nothing further. The label check runs first and the first blank label wins; `empty-path` only when every label is non-blank. No source region. |
| `inexact_decimal` | `nonzero-discarded-digit` | `expected`: the target type's declared domain, whose maximum scale is the rounding step's target scale. |
| `decimal_out_of_domain` | `outside-domain` | `expected`: the target `Decimal[..]` domain. |
| `division_out_of_domain` | `quotient-outside-domain`, `remainder-outside-domain` | `expected`: the bounded consumer's integer domain; the cause names the exposed member that fails it, the quotient for `div` or the remainder for `rem`. |
| `modulo_out_of_domain` | `outside-domain` | `expected`: the bounded consumer's integer domain. |
| `text_length_out_of_domain` | `outside-domain` | `expected`: the declared `Text[..]` bounds and profile. |
| `integer_out_of_domain` | `outside-domain` | `expected`: the target integer domain. |
| `rational_out_of_domain` | `outside-domain` | `expected`: the `Rational[..]` result domain. |
| `ieee_not_exact` | `rounding-required` | `expected`: the target IEEE width; `flags`: the would-be flag set, the flags the same operation would return under `nearest-even` (FR-148). |
| `ieee_nan_payload_not_representable` | `payload-exceeds-target` | `expected`: the target IEEE width; `actual`: the source IEEE width. |
| `ieee_rational_out_of_domain` | `outside-domain` | `expected`: the target `Rational[..]` domain. |
| `runtime_invariant` | `established-invariant-broken` | The named violated evaluator/input invariant and originating stage/context. For the invariant `identity-byte-bound-reached` (originating context `population.identity`), also `kind`: the encoder limit kind reached, exactly one of `canonical-bytes`, `input-bytes` or `object-bytes`, `bound`: the identity byte bound derived from the population's members (not a configured value) and `required`: the amount the encoding needed. |

The catalog attaches non-collapsing rules to individual rows, retained here
verbatim: "An ungrammatical index/slice is not a recognized profile extension"
(`invalid_syntax`); "Presence in unreachable syntax does not erase the refusal"
(`unsupported_construct`); "A known package/diagnostic interpretation used as a
clause profile is the latter" (`unknown_*` selections); "A recognized name is not
implementation support" (`unknown_required_feature`); "A missing artifact cannot
supply an invented actual digest" (`stale_dependency`,
`source_digest_mismatch`); "Missing versus explicit null may not be collapsed
into a generic absent value" (`invalid_model_binding`); "Failure to prove
definedness is not logical violation" (`undefined_expression`); "A missing
observation under a valid mapping is a different code" (`invalid_runtime_input`);
"Unknown membership is not an empty or complete population"
(`unavailable_observation`, `incomplete_population`); "Without that complete
population, use the incomplete-population outcome instead" (`dangling_reference`);
"A maximum fixed by the language is not a caller work budget" (`resource_exhausted`,
`cancelled`); "It cannot reclassify a valid source as malformed"
(`unsupported_projection`); "These three refusals cannot share only an English
distinction" (the extraction refusals); "It is a failed evaluation, not a false
predicate" (`runtime_invariant`).

## Identity and comparison

- **Identity kind** — *value*. A cause has no identity of its own and is
  compared componentwise.
- **Key** — (`code family`, `variant tag`, `required payload`). Equality is
  componentwise over exactly these and nothing else. Rendering is excluded on
  the strongest available grounding: FR-047-AC-1 requires that
  "changing/localizing human-readable messages leaves the same typed failure's
  stage, code and cause details unchanged"; FR-047 states "Diagnostic
  formatting/localization cannot change classification"; the catalog states
  "Serialization emits its stable tag; formatting/localization reads it and does
  not select it" and "A cause is a closed discriminated value with the payload
  required by its selected variant, not a free-form string map". Message text,
  locale, display stage labels and display paths therefore never participate in
  the key. Additional nonsemantic explanatory context carried by the producer
  interface does not participate either: it "cannot replace a required cause
  field or variant".
- **Key order** — lexicographic over (`code family`, `variant tag`, payload
  components in their stated order), for collection comparison only. It carries
  no causal meaning and expresses **no severity or precedence ordering** between
  codes or between variants.
- **Causal order** — none in general. An upstream diagnostic named by a
  `projection_binding` `upstream-refusal` cause is a **referenced identity, not
  an ordering**: "Related declarations and upstream diagnostics retain their own
  exact source/catalog identities rather than being flattened into a string"
  (proposed catalog).
- **Absence** — an absent key component compares only to absence. The catalog is
  explicit: "Unavailable context is explicitly unavailable; byte zero, an empty
  path or a name-search match cannot masquerade as a located failure."
  FR-047-AC-4 adds that "diagnostics do not reconstruct a location by searching
  for a repeated name". A missing payload component is never defaulted, and
  "Missing versus explicit null may not be collapsed into a generic absent
  value" for `invalid_model_binding`.
- **Digest domain** — where a payload component is a digest, notably the
  `byte-digest-mismatch` and `digest-domain-mismatch` variants, it carries the
  domain it was derived under
  ([FR-201](FR-201-canonical-identity-domain.md)); identities
  crossing domains are unequal regardless of byte equality. The catalog that
  resolves the cause is itself selected by its interpretation identity
  ([FR-270](FR-270-diagnostic-catalog.md)).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-272-AC-1 | Each listed code family carries exactly the closed variant set recorded above, with no added or removed variant. | Inspection |
| FR-272-AC-2 | Two causes with the same code family and variant tag but different required payloads compare unequal. | Analysis |
| FR-272-AC-3 | Changing or localizing the human-readable message leaves the cause key unchanged. | Analysis |
| FR-272-AC-4 | A cause with an unavailable payload component compares only to absence and is not represented by byte zero, an empty path or a name-search match. | Analysis |
| FR-272-AC-5 | An upstream refusal retains the originating diagnostic's own identity rather than a flattened string. | Inspection |
| FR-272-AC-6 | Each model-domain refusal of FR-150 through FR-154 carries its named variant of `missing_import`, `stale_dependency`, `missing_declaration`, `ambiguous_declaration`, `invalid_model_binding`, `ill_typed`, `undefined_expression`, `ambiguous_dispatch`, `incomplete_population`, `invalid_runtime_input` or `foreign_reference`, and `unavailable_observation` gains no model variant. | Test (TC-196) |
| FR-272-AC-7 | Every checked-package V2 operation and declaration refusal cause is listed in exactly the native-diagnostics row of its code (`invalid_package` for the eight operation causes, `stale-node-key` and `declaration-nominal-mismatch`, `ill_typed` for `operator-ineligible`, `ambiguous_declaration` for `ambiguous-name`), the V2 Diagnostic schema admits each cause only with that code, no two refusals share a cause, every published operation mutation records its cause and that cause's code (a stale retained key records `stale-node-key`), and a package reports the first refusal of its lowest-digest node, an application's own identity, law, mode and member causes before its argument applications' refusals, and those before its own `operator-ineligible`. | Test (TC-233) |
| FR-272-AC-8 | An unproved `case` exhaustiveness obligation of FR-146 carries `undefined_expression` with cause `unproved-exhaustiveness`, naming the scrutinized union, the arm set and the failed obligation, and no other code family gains a case-exhaustiveness variant. | Test (TC-264) |
| FR-272-AC-9 | Each FR-290 capability refusal carries `invalid_capability` with exactly its named cause, a no-capable-backend or unsupported obligation form carries `unsupported_projection`/`unsupported-requested-capability`, an unbounded extent without a finite bound carries `unsupported_projection`/`unbounded-extent`, a routed tool absence carries `unsupported_projection`/`tool-unavailable`, and no cause maps a refused label to an FR-290 member. | Test (TC-271, TC-282) |
| FR-272-AC-10 | The native-diagnostics `stage_limit_exceeded` row and this requirement's `stage_limit_exceeded` row each list exactly `input-bytes-exceeded`, `token-count-exceeded`, `node-count-exceeded`, `edge-count-exceeded`, `occurrence-count-exceeded`, `diagnostic-count-exceeded`, `work-budget-exceeded` and `rational-bits-exceeded` and name the `kind`, `bound`, `actual` and `setting` payload keys, and the two `duplicate_selection` rows each list exactly `duplicate-identity`. | Test (TC-233) |
| FR-272-AC-11 | The native-diagnostics row and this requirement's row of each code list exactly these causes: `invalid_source_identity` `blank-label` and `empty-path`; `division_out_of_domain` `quotient-outside-domain` and `remainder-outside-domain`; `inexact_decimal` `nonzero-discarded-digit`; `ieee_not_exact` `rounding-required`; `ieee_nan_payload_not_representable` `payload-exceeds-target`; and `decimal_out_of_domain`, `modulo_out_of_domain`, `text_length_out_of_domain`, `integer_out_of_domain`, `rational_out_of_domain` and `ieee_rational_out_of_domain` each `outside-domain`. Each code's two rows name exactly these payload keys: `label` for `invalid_source_identity`; `expected` and `flags` for `ieee_not_exact`; `expected` and `actual` for `ieee_nan_payload_not_representable`; and `expected` for each other code. `label` and `flags` stand in this requirement's canonical payload-name list. | Test (TC-233) |
| FR-272-AC-12 | The native-diagnostics and FR-272 `invalid_source_identity` rows each check the labels `authority` and `identity` in that order before the path, the first blank label deciding `blank-label`, and each reports `empty-path` only when every label is non-blank. | Test (TC-233) |
| FR-272-AC-13 | The catalog writes each payload value as one exact ASCII string: a domain as one of `Int[lo, hi]`, `Decimal[lo, hi; smin, smax]`, `Rational[lo, hi; dmin, dmax]` and `Text[min, max; profile]` with FR-038 integer bounds, `, ` inside a group and `; ` between groups; an IEEE width as `binary32` or `binary64`; a flag set as its flags in FR-148's order `invalid`, `divide_by_zero`, `overflow`, `underflow`, `inexact`, joined by `,`; and a label as `authority` or `identity`. | Test (TC-233) |
| FR-272-AC-14 | A request that retains a content identity differing from the identity its subject carries, such as a node identity taken from another package, refuses `stale_dependency`/`content-mismatch` naming both identities. | Test (TC-047) |
| FR-272-AC-15 | The native-diagnostics row and this requirement's row of `invalid_probabilistic_model`, `invalid_timed_model` and `invalid_result` each list the same closed cause set, and each probabilistic, timed or result refusal of FR-405, FR-407, FR-409, FR-410, FR-415, FR-416, FR-419, FR-193 and FR-255 carries its named cause from that set; `invalid_capability` adds `absent-evidence-kind` and `unknown-evidence-kind` for FR-290's evidence kinds and `not-statistical` for FR-408's method check. | Test (TC-350, TC-358, TC-367) |
| FR-272-AC-16 | The native-diagnostics row and this requirement's row of `resource_exhausted` each list exactly five causes, `insufficient-next-charge`, `allocation-failed`, `reservation-size-overflow`, `intake-limit-exceeded` and `input-bytes-exceeded`, name `stage`, `accounting_contract`, `limit_kind`, `limit`, `consumed`, `next_charge` and `charge_point` for `insufficient-next-charge`, `requested` for `allocation-failed`, `upstream` for `reservation-size-overflow`, `kind`, `bound`, `actual` and `setting` for each of `intake-limit-exceeded` (the intake budget) and `input-bytes-exceeded` (the format output budget only), and `cancelled` lists exactly `caller-cancelled` and names `stage`, `accounting_contract`, `consumed`, `charge_point` and `related`; each payload key is a canonical name below and `requested` stands in this requirement's canonical payload-name list. | Test (TC-233) |
| FR-272-AC-17 | The native-diagnostics row and this requirement's row of `conflicting_declaration` each list exactly `duplicate-declaration` and name `subject` and `related`, and FR-271 holds one `conflicting_declaration` row. | Test (TC-233) |
| FR-272-AC-18 | The native-diagnostics row and this requirement's row of `runtime_invariant` each list exactly `established-invariant-broken` and, for the invariant `identity-byte-bound-reached` (originating context `population.identity`), name `kind`, `bound` and `required`, each a canonical payload name below. | Test (TC-233) |
| FR-272-AC-19 | The `stage` an `insufficient-next-charge` cause retains is the stage label the denying accounting contract assigns to the denying counter: the native-diagnostics and FR-272 `resource_exhausted` rows and `value-accounting.md` state `model-normalization` for every `ModelNormalizationLimitsV1` counter (including `ancestor_steps` and `family_steps`), whichever checking stage charges it, and `population-admission` for every `PopulationAdmissionLimitsV1` counter; the `cancelled` row names the same rule; this `stage` is a charge-record payload field independent of the diagnostic's FR-273 producer stage. | Test (TC-233) |
| FR-272-AC-20 | The native-diagnostics row and this requirement's row of `unsupported_construct` each list exactly `declaration-form`, `expression-form` and `not-yet-implemented`; a refinement declaration using a feature its producer has not built (FR-375-AC-14) refuses `unsupported_construct`/`not-yet-implemented` retaining the feature name, the owning declaration and the selected profile identity, and the same refusal with another feature name compares unequal (FR-272-AC-2). | Test (TC-047) |
| FR-272-AC-21 | A ContractPackage reader incomplete result is `stage_limit_exceeded` at stage `package_read` with the cause the FR-272 `stage_limit_exceeded` row maps from its `limit_kind` one-to-one (`bytes`, `nodes`, `edges`, `occurrences`, `work` to `input-bytes-exceeded`, `node-count-exceeded`, `edge-count-exceeded`, `occurrence-count-exceeded`, `work-budget-exceeded`), `bound` its `limit`, `actual` its `consumed`, the FR-274 `json_path` location its `path` (absent for `bytes`, whose record has no `path`), and `setting` the name FR-330 gives; the reader reports no `diagnostics` stop. | Test (TC-937) |
| FR-272-AC-22 | The native-diagnostics row and this requirement's row of each of `unknown_contract_version`, `malformed_wire`, `duplicate_member`, `unknown_member`, `digest_domain_mismatch`, `unknown_required_capability`, `invalid_semantic_graph` and `unsupported_node_tag` list no cause variant and name `locus` and `document_pointer`, and `unknown_contract_version` also names `contract_version`; `locus` and `contract_version` are canonical payload names, and `locus` is the `CheckedNodeId` `{domain, digest}`, not an FR-274 location. | Test (TC-233) |
| FR-272-AC-23 | The native-diagnostics row and this requirement's row of `invalid_result`, and FR-331's "Result entries" table, each list exactly `measured-with-truth`, `missing-member`, `unknown-member`, `wrong-value-kind`, `invalid-value`, `certification-mismatch` and `unknown-cause`, and the two rows each name `document_pointer`, `expected` and `actual`. | Test (TC-233) |
| FR-272-AC-24 | The native-diagnostics row and this requirement's row of `invalid_quire_context` each list exactly `unsupported-render-form` and name the `view` and `form` payload keys, and neither row lists another cause. | Test (TC-233) |
| FR-272-AC-25 | The native-diagnostics row and this requirement's row of `unimplemented_capability` each list exactly `backend-not-supplied`, `untranslatable-entry` and `unimplemented-meaning`, and name the `backend`, `construct`, `construct_name`, `location` and `capability` payload keys. | Test (TC-233) |

## Canonical cause payload contract

Cause payload fields use these exact ordered names when applicable:
`subject`, `stage`, `definition`, `capability`, `label`, `expected`, `actual`,
`flags`, `path`, `document_pointer`, `related`, `kind`, `bound`, `limit`, `setting`, `consumed`,
`upstream`, `requested`, `required`, `accounting_contract`, `next_charge`, `charge_point`, `limit_kind`, `contract_version`, `locus`. `setting`
names the FR-461 setting that raises a limit. `kind` names a limit's kind as one exact ASCII string, the cause-tag spelling QSL emits for it (for example `input-bytes-exceeded`, `node-count-exceeded`) or, for `identity-byte-bound-reached`, the encoder limit kind as the `runtime_invariant` row spells it, never prose; `bound` its configured value, or for `identity-byte-bound-reached` the byte bound derived from the population's members; `required` the amount a failed encoding needed; `limit` is the effective limit of an `insufficient-next-charge` charge record. `label` names which
source label a refusal concerns and `flags` a would-be IEEE flag set.
`contract_version` is the contract version string an `unknown_contract_version` refusal of a contract reader received, exact and unnormalized. `locus` is the `CheckedNodeId` `{domain, digest}` of the node at which a reader refusal was reached (FR-330); it is a node identity, not an FR-274 location kind, which a reader's `path` rides in. `document_pointer` is the RFC 6901 pointer of a value in a JSON document. Absent inapplicable fields are
omitted, never encoded as empty placeholders. Collections inside a field use
their owning semantic order or canonical key order. Unknown required fields
refuse; unknown optional fields are permitted only by a later selected wire
revision.

## Dependencies

- [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md) establishes the
  typed-cause obligation and the rendering-exclusion rule.
- [Native diagnostics catalog](../../../proposals/quire-v1/definitions/native-diagnostics.md)
  is the proposed, non-adopted source of the closed variant sets.
- [FR-271](FR-271-diagnostic-code-vocabulary.md) supplies the code families.
- [FR-274](FR-274-diagnostic-location-kind.md) supplies the location kinds.
- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Status

Accepted complete-V1 typed-cause contract.
