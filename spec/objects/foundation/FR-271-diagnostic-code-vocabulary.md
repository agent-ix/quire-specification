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
    type: specifies
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
| `stale_dependency` | A revision, byte-digest or digest-domain mismatch against a dependency. |
| `source_digest_mismatch` | A revision, byte-digest or digest-domain mismatch against source. "A missing artifact cannot supply an invented actual digest." |
| `ambiguous_declaration` | A visible name or clause resolving to distinct conflicting declarations or authorities. |
| `invalid_package` | A malformed, incomplete, conflicting or cyclic package definition, including feature-set mismatch. |
| `invalid_model_binding` | A model binding with a wrong role, selection, export or context, a conflicting binding, or unpreserved model meaning. "Missing versus explicit null may not be collapsed into a generic absent value." |
| `ill_typed` | A type, unit, operator-eligibility, literal-ambiguity or non-Boolean-root typing failure. |
| `undefined_expression` | Definedness not proved for a potentially evaluated operation. "Failure to prove definedness is not logical violation." |
| `wrong_snapshot` | A wrong or forbidden observation, invocation, anchor or pre-read. |
| `invalid_runtime_input` | Malformed, incomplete, conflicting or wrongly-mapped runtime input. "A missing observation under a valid mapping is a different code." |
| `unavailable_observation` | A required artifact, valuation or scope premise that is not available. Named by FR-008-AC-4: "Missing sequence data receives unavailable_observation." |
| `incomplete_population` | A population whose membership is not established. "Unknown membership is not an empty or complete population." |
| `dangling_reference` | A reference absent from a population admitted as complete. "Without that complete population, use the incomplete-population outcome instead." |
| `population_delta_mismatch` | A declared delta disagreeing with the observed pre/post change. |
| `frame_violation` | A change to an object, field or State root not permitted by the declared frame. |
| `resource_exhausted` | A work item that could not proceed under the effective limit and accounting contract. "A semantic maximum is not a caller work budget." |
| `cancelled` | A work item stopped by a caller cancellation source. |
| `unsupported_projection` | A requested capability the selected backend does not support. "It cannot reclassify a valid source as malformed." |
| `projection_binding` | An upstream refusal carried with its originating structured IR/mapping cause. |
| `invalid_projection_correspondence` | A loss of correspondence between exact source and derived identities. |
| `extraction-requires-run` | An extraction refused because of the requested command mode. |
| `extraction-package-conflict` | An extraction refused because of conflicting compilation inputs. |
| `extraction-clause-count` | An extraction refused on binding cardinality. These three extraction refusals "cannot share only an English distinction". |
| `runtime_invariant` | A named established evaluator or input invariant broken. "It is a failed evaluation, not a false predicate." |

## Identity and comparison

- **Identity kind** — *value*. A code is an exact string with no identity of its
  own; it is compared componentwise.
- **Key** — the code spelling itself, taken together with the catalog selection
  that resolves it ([FR-270](FR-270-versioned-diagnostic-catalog.md)), because
  FR-047-AC-2 requires that "every emitted machine code resolves to the selected
  versioned catalog". Matching is exact and case-sensitive: hyphenated and
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
  selected by identity, revision and byte digest, whose domain rule is
  [FR-201](FR-201-canonical-identity-domain.md).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-271-AC-1 | Each member's value equals the exact spelling named by the proposed catalog, and an underscored spelling does not match a hyphenated one. | Inspection |
| FR-271-AC-2 | A code that does not resolve in the selected catalog compares to absence and is not mapped onto a member by substring or message match. | Analysis |
| FR-271-AC-3 | The key order over code spellings is not cited as a severity or precedence ordering. | Analysis |
| FR-271-AC-4 | Localizing the message accompanying a diagnostic leaves its code unchanged. | Analysis |

## Open gap: two spelling conventions, no namespace rule

The catalog makes spelling load-bearing — "Hyphenated and underscored spellings
stay exact; they are not normalized aliases" — while requirements use two
conventions and name no catalog selection for either:

- **Bare snake_case literals.** FR-003-AC-2: "A span splitting a UTF-8 scalar
  receives invalid_source_map." FR-008-AC-4: "Missing sequence data receives
  unavailable_observation." FR-009-AC-3: "Coincident object labels from
  different universes receive foreign_reference."
- **Kebab-case literals.** FR-062 Outputs: "one refusal carrying a stable code
  from `invalid-source-result`, `unsupported-source-version`,
  `unsupported-target-version`, `identity-mismatch`, `unrepresented-source-state`
  or `finding-loss`".

Both conventions are therefore load-bearing, but **no requirement states which
convention belongs to which namespace**, and no requirement places
`invalid_source_map`, `foreign_reference` or any FR-062 literal in a selected
catalog. `unavailable_observation` is spelled identically in FR-008-AC-4 and in
the proposed catalog; whether that is the same code or a coincident spelling in
a second namespace is not stated. None of these FR-spelled literals is listed as
a member above, because no prose establishes their membership. This artifact
records the divergence and resolves neither convention nor membership.

## Open gap: retained codes named by meaning only

The proposed catalog retains seven further codes by meaning without giving their
spellings: "I/O error, malformed command, invalid digest/identifier, output
serialization failure, invalid Quire context, invalid source identity and
invalid source map." They are not listed as members, because inventing a
spelling for them would add members the prose does not name.

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

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage. Where the prose it
extracts from does not establish an identity, key or order, that absence is
recorded here rather than supplied.
