---
id: FR-271
title: "Diagnostic code vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-047
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-003
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-008
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-009
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-062
    type: references
---
# FR-271: Diagnostic code vocabulary

## Description

When a producer reports a failure, the reader SHALL match its machine-readable code as an exact catalogued spelling rather than as a normalized alias or an English message match.

The members below are the code families named by the **proposed** native
diagnostic interpretation
([native diagnostics catalog](../../../proposals/quire-v1/definitions/native-diagnostics.md)),
which is "**Proposed, not adopted or implemented.**" They are extracted, not
adopted. Each member's closed cause-variant set is typed by
[FR-272](FR-272-typed-diagnostic-cause.md). Spellings are exact: "Hyphenated and
underscored spellings stay exact; they are not normalized aliases" (proposed
catalog).

## Values

| Value | Description |
| --- | --- |
| `invalid_syntax` | Ungrammatical source. "An ungrammatical index/slice is not a recognized profile extension." |
| `unsupported_construct` | A recognized form prohibited by the exact selected profile. "Presence in unreachable syntax does not erase the refusal." |
| `unknown_language` | An unrecognized or wrongly-roled language selection. |
| `unknown_edition` | An unrecognized or wrongly-roled edition selection. |
| `unknown_profile` | An unrecognized or wrongly-roled profile selection. "A known package/diagnostic interpretation used as a clause profile" is a wrong selection role. |
| `unknown_wire` | An unsupported selected payload wire, "even if its outer reference envelope is well formed". |
| `unknown_required_feature` | A required feature that is unknown or unsupported in the selected consumer/profile scope. "A recognized name is not implementation support." |
| `missing_import` | A requested typed dependency that is not present in the owning scope. |
| `missing_declaration` | A requested name that is not present in the owning scope. |
| `stale_dependency` | A byte-digest, digest-domain or content-identity mismatch against a dependency. |
| `source_digest_mismatch` | A byte-digest or digest-domain mismatch against source. "A missing artifact cannot supply an invented actual digest." |
| `ambiguous_declaration` | A visible name or clause resolving to distinct conflicting declarations or authorities, including two checked-package nodes declaring the same qualified name. |
| `invalid_package` | A malformed, incomplete, conflicting or cyclic package definition, including feature-set mismatch, or a checked-package application whose operation identity, operator class, required law, lock-selected law, mode, member or type-pinned mode is unknown or inconsistent, a checked-package application node that retains a stale node key, or a checked-package declaration that disagrees with its nominal qualified declaration. |
| `invalid_model_binding` | A model binding with a wrong role, selection, export or context, a conflicting binding, unpreserved model meaning, or a specialization cycle, redefinition-target mismatch, derivation conflict, malformed domain package declaration or port-direction violation. "Missing versus explicit null may not be collapsed into a generic absent value." |
| `ill_typed` | A type, unit, operator-eligibility, literal-ambiguity or non-Boolean-root typing failure, a parameter or result variance, multiplicity-narrowing, effect-escape or subsetting-type conformance failure, or an identity-observing form over a `symmetric` population (FR-381). |
| `undefined_expression` | Definedness not proved for a potentially evaluated operation, or a redefinition refinement not constructed or proved. "Failure to prove definedness is not logical violation." |
| `wrong_snapshot` | A wrong or forbidden observation, invocation, anchor or pre-read. |
| `invalid_runtime_input` | Malformed, incomplete, conflicting or wrongly-mapped runtime input. "A missing observation under a valid mapping is a different code." |
| `unavailable_observation` | A required artifact, valuation or scope premise that is not available. Named by FR-008-AC-4: "Missing sequence data receives unavailable_observation." |
| `incomplete_population` | A population whose membership is not established. "Unknown membership is not an empty or complete population." |
| `dangling_reference` | A reference absent from a population admitted as complete. "Without that complete population, use the incomplete-population outcome instead." |
| `population_delta_mismatch` | A declared delta disagreeing with the observed pre/post change. |
| `frame_violation` | A change to an object, field or State root not permitted by the declared frame. |
| `resource_exhausted` | A work item that could not proceed under the effective limit and accounting contract. "A semantic maximum is not a caller work budget." |
| `cancelled` | A work item stopped by a caller cancellation source. |
| `unsupported_projection` | A requested capability kind with an empty candidate set (no registrant advertises it, or the named backend does not), an obligation form the one candidate's negotiation arm does not discharge, or an unbounded extent on a bounded-only advertisement with no finite bound or a routed backend whose tool is unavailable. "It cannot reclassify a valid source as malformed." |
| `projection_binding` | An upstream refusal carried with its originating structured IR/mapping cause. |
| `invalid_projection_correspondence` | A loss of correspondence between exact source and derived identities. |
| `extraction-requires-run` | An extraction refused because of the requested command mode. |
| `extraction-package-conflict` | An extraction refused because of conflicting compilation inputs. |
| `extraction-clause-count` | An extraction refused on binding cardinality. These three extraction refusals "cannot share only an English distinction". |
| `runtime_invariant` | A named established evaluator or input invariant broken. "It is a failed evaluation, not a false predicate." |
| `io_error` | An explicitly authorized tool or adapter I/O operation failed; evaluators never emit it for a semantic value. |
| `malformed_command` | A CLI command or option set failed structural validation. |
| `invalid_identifier` | A digest, typed identity or canonical domain failed validation. |
| `output_serialization` | A validated result could not be encoded under the selected output wire contract. |
| `invalid_quire_context` | A lifecycle request supplied an incompatible package, stage or semantic context. |
| `invalid_source_identity` | A required source label (authority or identity) is empty or only whitespace, or the source path is empty. Its causes are `blank-label` and `empty-path`. |
| `invalid_source_map` | A source map was non-monotone, out of bounds or split a UTF-8 scalar. |
| `cardinality_out_of_bound` | A constructed or converted collection whose occurrence or member count lies outside its type's inclusive cardinality bound (FR-144). |
| `foreign_reference` | A supplied identity belongs to a different authority, universe, model or subject domain than the position requires. Its causes are `foreign-universe`, `foreign-model-selection` and `foreign-type`. |
| `ambiguous_dispatch` | A dispatched operation call for which some closed subtype of the receiver's static type has no applicable method or several undominated methods. No source or registration order resolves it. |
| `unimplemented_capability` | Valid complete-V1 meaning is not implemented by the selected producer; distinct from prohibited source and unsupported backend projection. |
| `parity_failure` | Two implementations required to agree produced different typed verdicts or values for the same subject/input. |
| `invalid_capability` | A requested or advertised capability selection that names no FR-290 kind, an unknown kind or mode, an unsupported vocabulary identity, a duplicate registered backend, no extent classification, an unregistered or arm-less backend, candidates inconsistent with the manifest or named backend, or several candidate backends with none named. |
| `duplicate_selection` | A second domain-package model selection of an already-selected identity. A package selects at most one version of a domain-package identity. |
| `conflicting_declaration` | Two declarations at the S3 compile or link that must be unique resolving to one identity: two `operation-body` declarations resolving to one operation identity (FR-151), or two tasks with one name. |
| `stage_limit_exceeded` | A compiler stage S1-S4, the I2 reader, a family `check`, `replay` or `route` reached one of its own stage limits. "A semantic maximum is not a caller work budget": this is distinct from `resource_exhausted`. |
| `noncanonical_wire` | A JSON wire document that its RFC 8785 encoding does not carry as written: a number whose exact value that encoding loses, with cause `inexact-integer` or `inexact-number` at its `document_pointer`, or, for a reader that requires canonical bytes as the checked-package V2 reader does (FR-322), any other bytes, as a refusal about the byte stream. |
| `inexact_decimal` | Strict `exact` met a decimal rounding step: a nonzero digit would be discarded. |
| `decimal_out_of_domain` | A decimal result outside its target `Decimal[..]` domain. |
| `division_out_of_domain` | A `div` quotient or a `rem` remainder outside the bounded consumer's domain. Only the member the operator exposes is checked; the refused member is not exposed. |
| `modulo_out_of_domain` | A Euclidean `mod` remainder outside the bounded consumer's domain. |
| `text_length_out_of_domain` | A text value whose profile length is outside its declared `Text[..]` bounds. |
| `integer_out_of_domain` | An exact conversion or arithmetic result outside its target integer domain. |
| `rational_out_of_domain` | An exact rational arithmetic result outside its `Rational[..]` result domain. |
| `ieee_not_exact` | Strict IEEE `exact` met an inexact, overflowing or tiny-and-inexact result. Only the would-be flags are reported. |
| `ieee_nan_payload_not_representable` | An explicit IEEE width conversion of a NaN whose payload does not fit below the target width's quiet bit. |
| `ieee_rational_out_of_domain` | An explicit IEEE-to-`Rational` conversion whose reduced rational is outside the target `Rational[..]` domain. |

## Identity and comparison

- **Identity kind** — *value*. A code is an exact string with no identity of its
  own; it is compared componentwise.
- **Key** — the code spelling itself, taken together with the catalog selection
  that resolves it ([FR-270](FR-270-diagnostic-catalog.md)), because
  FR-047-AC-2 requires that "every emitted machine code resolves to the selected
  catalog". Matching is exact and case-sensitive: hyphenated and
  underscored spellings "stay exact; they are not normalized aliases" (proposed
  catalog). Message text, locale and rendering never participate — FR-047-AC-1:
  "Changing/localizing human-readable messages leaves the same typed failure's
  stage, code and cause details unchanged"; FR-047: "Diagnostic
  formatting/localization cannot change classification".
- **Key order** — lexicographic over the spelling, for collection comparison
  only. It carries no causal meaning and expresses **no severity, precedence or
  preference ordering between codes**. The catalog forbids choosing a broader
  alternative: "A producer cannot choose a broader listed alternative to discard
  a distinction that it knows."
- **Causal order** — none. Where one diagnostic names an upstream diagnostic,
  that upstream diagnostic is a referenced identity, not an ordering: "Related
  declarations and upstream diagnostics retain their own exact source/catalog
  identities rather than being flattened into a string" (proposed catalog).
- **Absence** — a diagnostic with no resolvable code has an absent code and
  compares only to absence. It is never defaulted to a member and never recast:
  an unknown code "refuses interpretation of that diagnostic while retaining its
  original bytes and available provenance. It cannot be recast as a known
  failure, an empty diagnostic list or a successful assessment" (proposed
  catalog).
- **Digest domain** — a code carries no digest. The catalog that resolves it is
  selected by its interpretation identity
  ([FR-270](FR-270-diagnostic-catalog.md)).

## Scope: producer-minted codes

This vocabulary is the **producer's**. Its members are the code families a
producer emits when it reports a failure about the source it is processing, and
[FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)'s
catalogued-classification obligation is what they answer to.

A code a **consumer** mints for a refusal it raises about its own operations is
not a member of this table and is not governed by it. Such a code lives in that
consumer's own namespace, and this specification fixes no spelling convention for
it. A consumer code whose spelling equals a member spelling here is a different
code in a different namespace: it never resolves to a member of this table, and
matching is by selected catalog and exact spelling together, never by spelling
alone.

That is a scope boundary, not permission to collide. Because resolution is by
catalog and spelling together, a consumer reusing a published family's spelling
for a different meaning produces two codes that read alike and resolve
differently, which is exactly the hazard the catalog's "not normalized aliases"
rule guards against.

FR-283 now selects the separate projection-refusal catalog and its exact
kebab-case spellings. That resolves the cross-namespace question without
settling unselected native literals or treating spelling as a global convention.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-271-AC-1 | Each member's value equals the exact spelling named by the proposed catalog, and an underscored spelling does not match a hyphenated one. | Inspection |
| FR-271-AC-2 | A code that does not resolve in the selected catalog compares to absence and is not mapped onto a member by substring or message match. | Analysis |
| FR-271-AC-3 | The key order over code spellings is not cited as a severity or precedence ordering. | Analysis |
| FR-271-AC-4 | Localizing the message accompanying a diagnostic leaves its code unchanged. | Analysis |
| FR-271-AC-5 | A code minted by a consumer for its own refusal does not resolve to a member of this table, even when its spelling equals a member spelling. | Analysis |
| FR-271-AC-6 | The values table has exactly one row for each of `invalid_source_identity` and the ten evaluator value-refusal codes: `inexact_decimal`, `decimal_out_of_domain`, `division_out_of_domain`, `modulo_out_of_domain`, `text_length_out_of_domain`, `integer_out_of_domain`, `rational_out_of_domain`, `ieee_not_exact`, `ieee_nan_payload_not_representable` and `ieee_rational_out_of_domain`. | Test (TC-233) |

## Resolved catalog namespaces

The catalog makes spelling load-bearing — "Hyphenated and underscored spellings
stay exact; they are not normalized aliases" — while requirements use two
conventions:

- **Bare snake_case literals.** FR-003-AC-2: "A span splitting a UTF-8 scalar
  receives invalid_source_map." FR-008-AC-4: "Missing sequence data receives
  unavailable_observation." FR-009-AC-3: "Coincident object labels from
  different universes receive foreign_reference."
- **Kebab-case projection literals.** FR-062 Outputs names six values and FR-283
  now places them in `quire.protocol.projection-refusal/v1-draft.1`. They are
  outside this native producer catalog and have no normalized aliases here.

The complete-V1 native producer catalog assigns `invalid_source_map`,
`foreign_reference` and `unavailable_observation` the exact snake-case members
listed above. FR-283 independently resolves its six-member projection namespace;
those kebab-case codes do not alias native members.

## Resolved retained code spellings

The values table fixes the seven previously meaning-only spellings as
`io_error`, `malformed_command`, `invalid_identifier`, `output_serialization`,
`invalid_quire_context`, `invalid_source_identity` and `invalid_source_map`.
It additionally fixes `unimplemented_capability` and `parity_failure` for the
complete-V1 lifecycle, `cardinality_out_of_bound` for FR-144 collection
construction and FR-145 conversion, `ambiguous_dispatch` for FR-151 link-time
dispatch selection, `invalid_capability` for FR-290 capability-kind admission
and candidate-set negotiation, `duplicate_selection` for a second selection of
an already-selected domain-package identity, `stage_limit_exceeded` for a stage
limit, `noncanonical_wire` for a JSON number that RFC 8785 cannot carry
exactly and for the FR-322 V2 reader's non-canonical bytes, and
`inexact_decimal`, `decimal_out_of_domain`,
`division_out_of_domain`, `modulo_out_of_domain`,
`text_length_out_of_domain`, `integer_out_of_domain`, `rational_out_of_domain`,
`ieee_not_exact`, `ieee_nan_payload_not_representable` and
`ieee_rational_out_of_domain`, one refusal code per evaluator value refusal.
FR-272 types every cause, including the causes of `foreign_reference` for
FR-153 and FR-204, the checked-package operation variants, `stale-node-key`,
`declaration-nominal-mismatch` and `ambiguous-name`, `unproved-exhaustiveness`
for FR-146, the `unbounded-extent` and `tool-unavailable` causes of
`unsupported_projection`, the `stage_limit_exceeded` causes, the
`noncanonical_wire` causes, and the `invalid_source_identity` causes; `operator-ineligible` remains `ill_typed`. Matching remains catalog-qualified and exact.

## Dependencies

- [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md) establishes the
  catalogued-classification obligation.
- [FR-003](../../functional/foundation/FR-003-bind-source-correspondence.md),
  [FR-008](../../functional/foundation/FR-008-preserve-sequence-semantics.md),
  [FR-009](../../functional/foundation/FR-009-preserve-object-identity.md) and
  [FR-062](../../functional/protocol/FR-062-project-results-to-portable-verification.md)
  supply the FR-spelled code literals recorded above.
- [Native diagnostics catalog](../../../proposals/quire-v1/definitions/native-diagnostics.md)
  is the proposed, non-adopted source of the members.
- [FR-200](FR-200-quire-assessment-domain.md) supplies the identity
  discipline.
- [FR-283](../protocol/FR-283-projection-refusal-code.md) defines the separate
  projection-refusal namespace.

## Status

Accepted complete-V1 diagnostic-code contract. Implementation and qualification
remain tracked separately.
