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
| `missing_import`, `missing_declaration` | `missing-selection`, `missing-name` | The requested typed dependency/name, owning scope and referring locus. |
| `stale_dependency`, `source_digest_mismatch` | `revision-mismatch`, `byte-digest-mismatch`, `digest-domain-mismatch` | Expected and actual typed selections/digest domains where available. |
| `ambiguous_declaration` | `ambiguous-name`, `conflicting-authority` | The visible name/clause and the distinct conflicting declaration/authority selections and loci. |
| `invalid_package` | `malformed-json`, `missing-member`, `duplicate-member`, `unknown-member`, `wrong-value-kind`, `invalid-value`, `definition-cycle`, `conflicting-definition`, `incompatible-definition`, `feature-set-mismatch` | The actual field/index path or dependency edges/feature sets appropriate to that variant. |
| `invalid_model_binding` | `wrong-artifact-role`, `wrong-model-selection`, `wrong-export`, `wrong-context`, `conflicting-binding`, `unpreserved-model-meaning` | The required/actual declaration or contract, referring location and affected meaning. |
| `ill_typed` | `type-mismatch`, `unit-mismatch`, `operator-ineligible`, `ambiguous-literal`, `non-boolean-root` | The operator/input locus and expected/actual named type, unit or kind. |
| `undefined_expression` | `unproved-range`, `unproved-nonzero`, `unproved-presence` | The potentially evaluated operation, exact operand/read identity and applicable guard/anchor context. |
| `wrong_snapshot` | `wrong-observation`, `wrong-invocation`, `wrong-anchor`, `forbidden-pre-read` | Required and supplied observation/invocation/anchor selections, or the exact prohibited read. |
| `invalid_runtime_input` | `malformed-json`, `missing-member`, `duplicate-member`, `unknown-member`, `wrong-value-kind`, `invalid-value`, `wrong-role-mapping`, `conflicting-role-mapping` | The actual input path, owning required role and supplied mapping when relevant. |
| `unavailable_observation`, `incomplete_population` | `missing-required-artifact`, `missing-required-valuation`, `incomplete-scope` | The required role/scope, selected input and exact missing premise. |
| `dangling_reference` | `absent-target-in-complete-population` | The reference identity, exact required universe and admitted completeness selection. |
| `population_delta_mismatch`, `frame_violation` | `delta-disagreement`, `unauthorized-change`, respectively | The invocation, pre/post selections, affected object/field or State root, and the declared delta/frame permission. |
| `resource_exhausted`, `cancelled` | `insufficient-next-charge`, `caller-cancelled`, respectively | The actual stage, accounting contract, effective limit or cancellation source, prior usage and the work item that could not proceed. |
| `unsupported_projection` | `unsupported-requested-capability` | The exact admitted subject, requested claim/target and selected backend. |
| `projection_binding`, `invalid_projection_correspondence` | `upstream-refusal`, `correspondence-loss`, respectively | The originating structured IR/mapping cause and exact source/derived identities. |
| `extraction-requires-run`, `extraction-package-conflict`, `extraction-clause-count` | `wrong-command-mode`, `conflicting-compilation-inputs`, `binding-cardinality`, respectively | The requested mode, conflicting selectors or actual count. |
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

## Open gap

The catalog requires each variant's "required payload" but does not give those
payload components stable field names, a total order, or a spelling convention
of their own. The key order above is therefore stated over "payload components
in their stated order", which is the catalog's prose order and not a declared
order. No artifact establishes a canonical payload field vocabulary; this
artifact supplies none.

## Dependencies

- [FR-047](../../functional/FR-047-emit-typed-located-causes.md) establishes the
  typed-cause obligation and the rendering-exclusion rule.
- [Native diagnostics catalog](../../../proposals/quire-v1/definitions/native-diagnostics.md)
  is the proposed, non-adopted source of the closed variant sets.
- [FR-271](FR-271-diagnostic-code-vocabulary.md) supplies the code families.
- [FR-274](FR-274-diagnostic-location-kind.md) supplies the location kinds.
- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
