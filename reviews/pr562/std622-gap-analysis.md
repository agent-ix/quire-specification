---
id: SR-202610310145
title: "STD-622 source and qualification gap review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification; STD-622; FR-322-AC-291, qualification assertions and strict IR PL-vector completion"
review_set: all
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: reviews
---

## Summary

Gap completion stage of the same independent STD-622 review after its mandatory spec-first PASS. The owner qualification assertion and actual consumer run close the positive/negative witness gap without narrowing the missing-term refusal.

## Verdict

PASS for the scoped obligation chain, with no remaining gaps. Plan completion: not assessed. No whole-repository green assertion is made.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined scope

- FR-322-AC-291 (examined), spec/objects/interfaces/FR-322-checked-package-artifact.md: Each protocol location row joins to one local expression/application node and one of its exact source occurrences; two terms sharing that application and occurrence have distinct `term_ordinal` values and preserve their different semantic locations. Each captured application/occurrence group has consecutive ordinals starting at zero. PL-01-root's lone ordinal-zero row admits; PL-R2's lone ordinal-one row witnesses the missing ordinal-zero mapping and refuses `invalid_package`/`invalid-value` at `/checked_term_data/protocol_locations`. A foreign or absent node, non-application, missing occurren
- FR-322 (examined), spec/objects/interfaces/FR-322-checked-package-artifact.md: Within each captured application/occurrence group the ordinals
  are consecutive from zero: a row with ordinal `k` witnesses that every
  ordinal below `k` requires its own row.
- FR-322-AC-292 (context_only), spec/objects/interfaces/FR-322-checked-package-artifact.md: `checked-term-data-vectors.json` publishes positive top-level text, enum, declared-quantity, compound-quantity and plan equality records, plus two equalities beneath one `and` parent, an equality beneath `or`, aggregate-member and binding-value forms. The parent forms reference each child equality application by its exact `NodeRef`; each equality record names that child node and has an empty `term_path`. The four TC-1070 Fixture E expressions at rows 8, 12, 15 and 17 remain four separate equality rows. Positive metadata demonstrates these parent-to-child references without adding a wire member

The computed repository baseline is 2329 untagged, 309 tagged, 309 method-without-symbol. FR-322-AC-291 has one tagged owner Rust qualifier. That schema/data control alone does not discharge strict-reader admission: the separate actual IR run consumed every PL id and asserted the recorded code, cause where specified, and exact locus, plus typed borrowed positive locations. Interior missing predecessor, no unwitnessed successor, distinct occurrence group reset and canonical large path controls complete the new predicate branches. The branch has no stub, inferred row census or unconditional success path. Broader historical matrix gaps are not claimed repaired.

## Final source and qualification completion

PASS for this scoped specification, Rust qualification assertion and consumer-contract review. The QSpec Rust carrier test passes. The actual strict IR run passes 16 selected tests (12 Location and four hybrid/canonical controls) and explicitly prints all six PL outcomes: PL-01, PL-02 and PL-03 admit with their exact captured locations; PL-S1 refuses MalformedWire at origin/kind; PL-R1 refuses InvalidPackage/ConflictingDefinition at row one; PL-R2 refuses InvalidPackage/InvalidValue at the protocol_locations array. The consumed-id set must equal all six owner ids. Interior-gap and occurrence-group reset controls pass. The lone-child large-path control was repaired to include its published ordinal-zero predecessor while preserving the exact canonical path assertion. The predecessor guard uses the existing charged row pass, exact canonical application/occurrence equality and one boolean; validated safe ordinals make predecessor-plus-one arithmetic safe, with no range expansion or extra wire carrier. Existing row shape/join, duplicate and order failures still precede the final array-gap refusal.

The Rust QSpec assertion now proves that PL-R2 retains the existing positive child (metadata stripped), differs from PL-01, and has ordinal one while PL-01 has zero. The separate supplied-unknown-kind pointer assertion follows the explicitly reviewed owner precision rule. No acceptance condition was removed, no malformed input was admitted, and no second sidecar parser, mirror semantic type or compatibility path was introduced. This audit covers the STD-622 prefix delta and qualification evidence; the existing IR-1218 reviewer retains the original IR PR review/disposition.

Validation limits are explicit: IR workspace all-target Clippy passes on the tested revision. QSpec all-target Clippy fails in two untouched baseline files, and touched checked_package_v2-target Clippy reports 16 pre-existing lint locations before the changed assertion region; comparison to origin/main confirms those lines are unchanged. QSpec global formatting has extensive existing drift, and FR-322 document validation retains its reproduced baseline value_object id/fields errors. No blanket formatting, unrelated lint rewrite, CI trigger or proof was introduced. These broader checks are not claimed green. Artifact validation and scoped semantic qualification are separate claims. Reviewer ran no Cargo or proof.

## Final frozen-source receipts

The final frozen QSpec carrier test passes one test. The final rebased IR consuming run passes all 16 selected Location/hybrid tests in 0.66 seconds and prints all six PL ids with their exact positive or refusal outcomes. Final IR workspace all-target Clippy completes with warnings denied, and final IR formatting has empty output with parent-confirmed exit zero.

The isolated QSpec origin/main control reproduces exactly the same 16 touched-target Clippy diagnostics at the same source positions as the changed branch; all lie before the changed assertion region. Baseline formatting also fails, and the touched file's final formatting differences stop before the new assertions. Broader QSpec all-target Clippy includes two additional unchanged-file diagnostics. FR-322 value_object validation errors are likewise reproduced on origin/main. These checks are disclosed baseline limitations, not represented as passing qualification. The new code adds no lint or formatting diagnostic.

Final scoped review: PASS, with the implementation-stage precedence precision finding fixed and no unresolved findings. No new independent review, Cargo command, proof or source edit was performed by this reviewer.
