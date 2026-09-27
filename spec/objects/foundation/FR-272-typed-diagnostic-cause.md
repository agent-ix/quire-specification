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

| Property | Source | Note |
| --- | --- | --- |
| Code family | [FR-271](FR-271-diagnostic-code-vocabulary.md) | The code family whose closed variant set the cause is drawn from. |
| Variant tag | Proposed catalog, "Required distinguishing causes" | The selected variant, one of the closed set for that family. "Serialization emits its stable tag." |
| Required payload | Proposed catalog, per-family row | The payload "required by its selected variant". Values are "typed selections, kinds, domains or locations, not English summaries". |
| Location, where known | [FR-274](FR-274-diagnostic-location-kind.md), FR-047-AC-4 | "Original source/model/clause loci and identities remain attached where available." |
| Referenced identities | Proposed catalog | "Related declarations and upstream diagnostics retain their own exact source/catalog identities rather than being flattened into a string." |

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
| `unsupported_construct` | `declaration-form`, `expression-form` | The recognized form, owning declaration and exact selected profile that prohibits it. |
| `unknown_language`, `unknown_edition`, `unknown_profile` | `unsupported-selection`, `wrong-selection-role` | The supplied selection and required role. |
| `unknown_wire` | `unsupported-wire` | The actual selected wire and expected artifact role. |
| `unknown_required_feature` | `unknown-feature`, `unsupported-feature` | The feature and selected consumer/profile scope. |
| `missing_import`, `missing_declaration` | `missing-selection`, `missing-name` | The requested typed dependency/name, owning scope and referring locus; during model intake, the ModelSelection, or the referring IR node identity, absent node identity, artifact id and span. |
| `stale_dependency`, `source_digest_mismatch` | `revision-mismatch`, `byte-digest-mismatch`, `digest-domain-mismatch` | Expected and actual typed selections/digest domains where available. |
| `ambiguous_declaration` | `ambiguous-name`, `conflicting-authority` | The visible name/clause and the distinct conflicting declaration/authority selections and loci; a checked-package V2 `ambiguous-name` (revision `1-draft.4`) retains the duplicated `declaration.qualified_name` and both node keys in ascending digest order. |
| `invalid_package` | `malformed-json`, `missing-member`, `duplicate-member`, `unknown-member`, `wrong-value-kind`, `invalid-value`, `definition-cycle`, `conflicting-definition`, `incompatible-definition`, `feature-set-mismatch`, `unknown-operation`, `operation-class-mismatch`, `operation-law-missing`, `operation-law-mismatch`, `operation-law-unselected`, `operation-mode-mismatch`, `operation-member-mismatch`, `operation-mode-type-mismatch`, `stale-node-key`, `declaration-nominal-mismatch` | The actual field/index path or dependency edges/feature sets appropriate to that variant; under `quire.model.complete/v1`, `definition-cycle` also covers a call-graph cycle through an FR-151 dispatch edge and retains its call edges; each of the eight operation variants (revision `1-draft.4`) retains the application node key, the pre-order ordinal of the application in that node, the leaf path (empty for the operation itself) and the supplied operation identity and class, plus the expected class, required law role, supplied law and its role's catalogued definitions, unselected law, required mode kind and supplied mode, required member kind and supplied member, or supplied mode value and the values pinned by the operand and result types, respectively (for a clause application the role is its profile-selection role and the catalogued definitions are the lock profile selections of that role, and a `profile_operator` member mismatch also retains the operators the selected profile defines); `stale-node-key` (revision `1-draft.4`) retains the retained application node key, the recomputed `quire.application-node/v1` digest and the node ordinal; `declaration-nominal-mismatch` (revision `1-draft.4`) retains the node key, its `declaration.qualified_name` and its nominal `qualified_declaration`. |
| `invalid_model_binding` | `wrong-artifact-role`, `wrong-model-selection`, `wrong-export`, `wrong-context`, `conflicting-binding`, `unpreserved-model-meaning`, `specialization-cycle`, `redefinition-target`, `derivation-conflict`, `malformed-declaration`, `port-direction` | The required/actual declaration or contract, referring location and affected meaning; `wrong-export` for a systems-model kind adds required kind, actual kind (`none` for no kind) and both declaration keys; `specialization-cycle` the cycle's type declaration keys in path order; `redefinition-target` every redefining member's declaration key and the resolved targets; `derivation-conflict` both derivation paths; `malformed-declaration` the IR node identity and invalid member path; `port-direction` the connection's declaration key, its declared direction and both port directions; every model intake and model feature variant adds the source artifact id and span of the IR node it names. |
| `ill_typed` | `type-mismatch`, `unit-mismatch`, `operator-ineligible`, `ambiguous-literal`, `non-boolean-root`, `variance-parameter`, `variance-result`, `multiplicity-narrowing`, `effect-escape`, `subsetting-type` | The operator/input locus and expected/actual named type, unit or kind; each conformance variant adds the redefining and redefined effective member identities, the axis (and parameter index), the expected and actual effective type, typed multiplicity or effect entry, and every contributing declaration key in effective-identity order. |
| `undefined_expression` | `unproved-range`, `unproved-nonzero`, `unproved-presence`, `unproved-decrease`, `unproved-refinement`, `unproved-exhaustiveness` | The potentially evaluated operation, exact operand/read identity and applicable guard/anchor context; for `unproved-decrease`, the recursive component cycle, call edge and failed measure obligation; for `unproved-refinement`, the redefining field, the writing operation and the obligation `field-domain`, `field-presence` or `no-proof-form`; for `unproved-exhaustiveness` (revision `1-draft.4`), the scrutinized union's declaration identity, the `case` expression's arm set and the failed obligation `duplicate-arm`, `unknown-member`, `arm-arity` or `missing-arm`. |
| `ambiguous_dispatch` | `multiple-undominated`, `no-applicable` | The called effective operation, the closed subtype for which selection fails, every candidate effective method identity and, for `multiple-undominated`, every dominance pair among the candidates. |
| `cardinality_out_of_bound` | `below-minimum`, `above-maximum` | The collection type, its inclusive bound, the formed count and the constructing or converting expression locus. |
| `wrong_snapshot` | `wrong-observation`, `wrong-invocation`, `wrong-anchor`, `forbidden-pre-read` | Required and supplied observation/invocation/anchor selections, or the exact prohibited read. |
| `invalid_runtime_input` | `malformed-json`, `missing-member`, `duplicate-member`, `unknown-member`, `wrong-value-kind`, `invalid-value`, `wrong-role-mapping`, `conflicting-role-mapping`, `absent-key`, `abstract-instance`, `subsetting-violation`, `conflicting-identity` | The actual input path, owning required role and supplied mapping when relevant; `absent-key` adds the population binding, requested key and query locus; `abstract-instance` the member record and its abstract type's declaration key; `subsetting-violation` the subsetting field's declaration key, object and values outside the subsetted feature; `conflicting-identity` both member records. |
| `unavailable_observation` | `missing-required-artifact`, `missing-required-valuation`, `incomplete-scope` | The required role/scope, selected input and exact missing premise. |
| `incomplete_population` | `missing-required-artifact`, `missing-required-valuation`, `incomplete-scope`, `unclosed-subtypes` | The required role/scope, selected input and exact missing premise; `unclosed-subtypes` adds the ModelSelection, the population declaration key, the queried type and every uncovered conforming type. |
| `foreign_reference` | `foreign-universe`, `foreign-model-selection`, `foreign-type` | The required and supplied object universe, ModelSelection key or effective type identity, respectively, and the operation locus. |
| `dangling_reference` | `absent-target-in-complete-population` | The reference identity, exact required universe and admitted completeness selection. |
| `population_delta_mismatch`, `frame_violation` | `delta-disagreement`, `unauthorized-change`, respectively | The invocation, pre/post selections, affected object/field or State root, and the declared delta/frame permission. |
| `resource_exhausted`, `cancelled` | `insufficient-next-charge`, `caller-cancelled`, respectively | The actual stage, accounting contract, effective limit or cancellation source, prior usage and the work item that could not proceed. |
| `duplicate_selection` | `duplicate-identity` | The repeated domain-package `identity` and both the already-selected and newly-requested `version` (revision `1-draft.6`). |
| `stage_limit_exceeded` | `input-bytes-exceeded`, `nesting-depth-exceeded`, `token-count-exceeded`, `node-count-exceeded`, `edge-count-exceeded`, `occurrence-count-exceeded`, `diagnostic-count-exceeded`, `work-budget-exceeded` | The exceeded stage limit kind, its configured bound, the actual counter and the source location where it was reached, when known (revision `1-draft.6`; `token-count-exceeded`, `edge-count-exceeded`, `occurrence-count-exceeded` and `diagnostic-count-exceeded` revision `1-draft.7`). S1's token ceiling carries `token-count-exceeded`; the I2 reader's edge, occurrence and source-map, and diagnostic-entry limits carry `edge-count-exceeded`, `occurrence-count-exceeded` and `diagnostic-count-exceeded`, respectively. |
| `unsupported_projection` | `unsupported-requested-capability`, `unbounded-extent`, `tool-unavailable` | The exact admitted subject, requested claim/target and selected backend; for `unsupported-requested-capability`, the item's FR-290 kind and any named backend when the candidate set is empty, or the obligation form the one candidate's negotiation arm does not discharge; for `unbounded-extent`, the kind, the candidate and its advertised modes; for `tool-unavailable`, the kind, the routed backend and the expected and actual (or absent) tool identity, with FR-331 result `unsupported` at probe or `failed` after a passing probe. |
| `invalid_capability` | `absent-kind`, `unknown-kind`, `unsupported-version`, `unknown-mode`, `duplicate-backend`, `absent-extent`, `unknown-backend`, `inconsistent-candidates`, `ambiguous-backend` | The request item index, or the backend identity for a registration cause; the exact supplied kind or mode for `unknown-kind` and `unknown-mode`; the supplied vocabulary identity or its absence, and the refused carrier, for `unsupported-version`; the repeated identity and the refused registration's manifest digest for `duplicate-backend`; the named unregistered backend, or registered backend with no negotiation arm, for `unknown-backend`; every offending candidate for `inconsistent-candidates`; every candidate in candidate order for `ambiguous-backend`. |
| `projection_binding`, `invalid_projection_correspondence` | `upstream-refusal`, `correspondence-loss`, respectively | The originating structured IR/mapping cause and exact source/derived identities. |
| `extraction-requires-run`, `extraction-package-conflict`, `extraction-clause-count` | `wrong-command-mode`, `conflicting-compilation-inputs`, `binding-cardinality`, respectively | The requested mode, conflicting selectors or actual count. |
| `invalid_source_identity` | `blank-label`, `empty-path` | For `blank-label`, `label`: the first of `authority`, `identity`, `revision_namespace` and `revision` whose value is empty or only Unicode `White_Space` scalars; for `empty-path`, nothing further. The label check runs first and the first blank label wins; `empty-path` only when every label is non-blank. No source region (revision `1-draft.8`). |
| `inexact_decimal` | `nonzero-discarded-digit` | `expected`: the target type's declared domain, whose maximum scale is the rounding step's target scale (revision `1-draft.8`). |
| `decimal_out_of_domain` | `outside-domain` | `expected`: the target `Decimal[..]` domain (revision `1-draft.8`). |
| `division_pair_out_of_domain` | `quotient-outside-domain`, `remainder-outside-domain`, `both-outside-domain` | `expected`: the bounded consumer's integer domain; the cause names which pair members fail it (revision `1-draft.8`). |
| `modulo_out_of_domain` | `outside-domain` | `expected`: the bounded consumer's integer domain (revision `1-draft.8`). |
| `text_length_out_of_domain` | `outside-domain` | `expected`: the declared `Text[..]` bounds and profile (revision `1-draft.8`). |
| `integer_out_of_domain` | `outside-domain` | `expected`: the target integer domain (revision `1-draft.8`). |
| `rational_out_of_domain` | `outside-domain` | `expected`: the `Rational[..]` result domain (revision `1-draft.8`). |
| `ieee_not_exact` | `rounding-required` | `expected`: the target IEEE width; `flags`: the would-be flag set, the flags the same operation would return under `nearest-even` (FR-148) (revision `1-draft.8`). |
| `ieee_nan_payload_not_representable` | `payload-exceeds-target` | `expected`: the target IEEE width; `actual`: the source IEEE width (revision `1-draft.8`). |
| `ieee_rational_out_of_domain` | `outside-domain` | `expected`: the target `Rational[..]` domain (revision `1-draft.8`). |
| `runtime_invariant` | `established-invariant-broken` | The named violated evaluator/input invariant and originating stage/context. |

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
"A semantic maximum is not a caller work budget" (`resource_exhausted`,
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
  resolves the cause is itself selected by identity, revision and byte digest
  ([FR-270](FR-270-versioned-diagnostic-catalog.md)).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-272-AC-1 | Each listed code family carries exactly the closed variant set recorded above, with no added or removed variant. | Inspection |
| FR-272-AC-2 | Two causes with the same code family and variant tag but different required payloads compare unequal. | Analysis |
| FR-272-AC-3 | Changing or localizing the human-readable message leaves the cause key unchanged. | Analysis |
| FR-272-AC-4 | A cause with an unavailable payload component compares only to absence and is not represented by byte zero, an empty path or a name-search match. | Analysis |
| FR-272-AC-5 | An upstream refusal retains the originating diagnostic's own identity rather than a flattened string. | Inspection |
| FR-272-AC-6 | Under catalog revision `1-draft.3`, each model-domain refusal of FR-150 through FR-154 carries its named variant of `missing_import`, `stale_dependency`, `missing_declaration`, `ambiguous_declaration`, `invalid_model_binding`, `ill_typed`, `undefined_expression`, `ambiguous_dispatch`, `incomplete_population`, `invalid_runtime_input` or `foreign_reference`, and `unavailable_observation` gains no model variant. | Test (TC-196) |
| FR-272-AC-7 | Under catalog revision `1-draft.4`, every checked-package V2 operation and declaration refusal cause is listed in exactly the native-diagnostics row of its code (`invalid_package` for the eight operation causes, `stale-node-key` and `declaration-nominal-mismatch`, `ill_typed` for `operator-ineligible`, `ambiguous_declaration` for `ambiguous-name`), the V2 Diagnostic schema admits each cause only with that code, no two refusals share a cause, every published operation mutation records its cause and that cause's code (a stale retained key records `stale-node-key`), and a package reports the first refusal of its lowest-digest node, an application's own identity, law, mode and member causes before its argument applications' refusals, and those before its own `operator-ineligible`. | Test (TC-233) |
| FR-272-AC-8 | Under catalog revision `1-draft.4`, an unproved `case` exhaustiveness obligation of FR-146 carries `undefined_expression` with cause `unproved-exhaustiveness`, naming the scrutinized union, the arm set and the failed obligation, and no other code family gains a case-exhaustiveness variant. | Test (TC-264) |
| FR-272-AC-9 | Under catalog revision `1-draft.5`, each FR-290 capability refusal carries `invalid_capability` with exactly its named cause, a no-capable-backend or unsupported obligation form carries `unsupported_projection`/`unsupported-requested-capability`, an unbounded extent without a finite bound carries `unsupported_projection`/`unbounded-extent`, a routed tool absence carries `unsupported_projection`/`tool-unavailable`, and no cause maps a refused label to an FR-290 member. | Test (TC-271, TC-282) |
| FR-272-AC-10 | Under catalog revision `1-draft.7`, the native-diagnostics `stage_limit_exceeded` row and this requirement's `stage_limit_exceeded` row each list exactly `input-bytes-exceeded`, `nesting-depth-exceeded`, `token-count-exceeded`, `node-count-exceeded`, `edge-count-exceeded`, `occurrence-count-exceeded`, `diagnostic-count-exceeded` and `work-budget-exceeded`, and the two `duplicate_selection` rows each list exactly `duplicate-identity`. | Test (TC-233) |
| FR-272-AC-11 | Under catalog revision `1-draft.8`, the native-diagnostics row and this requirement's row of each code list exactly these causes: `invalid_source_identity` `blank-label` and `empty-path`; `division_pair_out_of_domain` `quotient-outside-domain`, `remainder-outside-domain` and `both-outside-domain`; `inexact_decimal` `nonzero-discarded-digit`; `ieee_not_exact` `rounding-required`; `ieee_nan_payload_not_representable` `payload-exceeds-target`; and `decimal_out_of_domain`, `modulo_out_of_domain`, `text_length_out_of_domain`, `integer_out_of_domain`, `rational_out_of_domain` and `ieee_rational_out_of_domain` each `outside-domain`. Each code's two rows name exactly these payload keys: `label` for `invalid_source_identity`; `expected` and `flags` for `ieee_not_exact`; `expected` and `actual` for `ieee_nan_payload_not_representable`; and `expected` for each other code. `label` and `flags` stand in this requirement's canonical payload-name list. | Test (TC-233) |

## Canonical cause payload contract

Cause payload fields use these exact ordered names when applicable:
`subject`, `stage`, `definition`, `capability`, `label`, `expected`, `actual`,
`flags`, `path`, `related`, `limit`, `consumed`, `upstream`. `label` names which
source label a refusal concerns and `flags` a would-be IEEE flag set (catalog
revision `1-draft.8`). Absent inapplicable fields are
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

Accepted complete-V1 typed-cause contract. Implementation and qualification
remain tracked separately.
