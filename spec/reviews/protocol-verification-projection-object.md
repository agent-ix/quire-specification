---
id: SR-208
title: "Object review of the protocol-result verification projection"
type: SpecReview
analysis: base
scope: "quire-specification FR-061/TC-061 and quire-verification FR-007/e02-draft-2 TechniqueResult"
review_set: subset
evaluated_revision: "d104cf9b4dcf29515f9d4a830108dc86e9b11ea0"
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-061, type: reviews }
  - { target: ix://agent-ix/quire-verification/FR-007, type: reviews }
---
## Summary

The protocol result and the quire-verification `TechniqueResult` are correctly
separate objects. FR-061 owns the complete protocol result, including subject
progress and closure, settlement basis, decision support, protocol adequacy and
immutable supersession. quire-verification FR-007 owns a narrower compatibility
assessment over execution, activation, observation adequacy, participation,
assumptions and logical findings. The E02 assessor deliberately returns that
narrow view and does not retain arbitrary `TechniqueResult` extensions.

The boundary is nevertheless not rebuildable from the specifications at the
reviewed revisions. FR-061 requires a mapping that preserves every FR-007
finding, but no artifact defines the total mapping between the two closed state
vocabularies, the identity of the derived projection, or the distinction between
protocol adequacy and E02 observation adequacy.

## Verdict

**FAIL for implementation promotion.** The composed baseline may retain the two
objects, but a Rust projection adapter cannot be planned or accepted until the
mapping is normative and discriminating tests cover every source state and
semantic collision.

## Object inventory

| Object | Owner and authority | Required retained meaning | Observed consumer behavior |
| --- | --- | --- | --- |
| Protocol assessment result | quire-specification FR-061 and the protocol contract at `d104cf9` | Assessment execution, subject progress/closure, truth, settlement basis, activation, participation, completeness, protocol adequacy, claim, provenance, decision support and immutable history | Canonical result consumed directly by A/E/F and future Engineering Assurance readers |
| Verification technique result | quire-verification FR-007 and `contracts/e02-draft-2.schema.json` at `4a5e196` | Exact E02 scope plus execution, logical, activation, participation, assumptions, observation adequacy, provenance, native artifacts and oracle contributions | `assess_evidence` returns only the FR-007 assessment view and intentionally omits arbitrary top-level extensions |
| Protocol-to-verification projection | Named by FR-061-AC-7 but not defined as an object or process | A deterministic derived identity, source-result linkage, total state mapping, every simultaneous FR-007 finding and no sufficiency decision | Missing |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | FR-061 names a mapping but defines no total conversion for protocol execution states (`refused`, `resource-incomplete`), truth states (`pending`, `unavailable`), activation, participation or assumptions into E02's different closed vocabularies. Two conforming adapters can therefore produce different FR-007 findings from the same protocol result. | quire-specification FR-061; quire-verification FR-007/E02 | missing-requirement |
| FND-002 | high | Protocol adequacy and E02 observation adequacy share the word “adequacy” but have different authorities. No rule prevents protocol case coverage from being emitted as E02 observation adequacy or vice versa. | FR-061; E02 `TechniqueResult.adequacy` | missing-requirement |
| FND-003 | high | The mapping has no required exact reference to the canonical protocol-result bytes, no distinct derived result identity rule and no loss/refusal outcome for unrepresentable source meaning. The projected `TechniqueResult` can alias its source or become an untraceable replacement after the E02 assessor drops protocol-only dimensions. | FR-061-AC-3/AC-7; `result_view` | missing-requirement |
| FND-004 | medium | TC-061 asks only for one simultaneous partial/unmet/violated example. It does not walk every source state, adequacy collision, identity substitution or source-result retention rule. | TC-061 | missing-requirement |

## Cross-reference and completeness checks

- The upstream reference exists: FR-061 depends on quire-verification FR-007.
- Both payloads have dedicated normative ownership; no new authored language or
  duplicate evidence store is needed.
- `TechniqueResult.additionalProperties` does not solve the boundary. The Rust
  assessor projects selected fields in `result_view`, so arbitrary protocol
  fields are neither a typed mapping contract nor a retained canonical result.
- The projection should remain an adapter from the canonical protocol result to
  a derived E02 result. Expanding E02 into the complete protocol-result model
  would collapse the intended ownership boundary.

## Required remediation

1. Define one normative protocol-to-E02 projection process with a total mapping
   table and an explicit refusal for a source value the selected E02 revision
   cannot represent without losing an FR-007 finding.
2. Require a distinct derived result identity and an exact retained reference to
   the canonical protocol result, including its revision/digest and adapter
   version.
3. Map E02 `adequacy` only from observation adequacy. Preserve protocol adequacy
   in the canonical result and prohibit either adequacy from proving the other.
4. Extend TC-061 with every source-state row, simultaneous findings, identity
   substitutions, adequacy cross-wiring and confirmation that E02 assessment
   projection does not replace the canonical protocol result.

## Evidence inspected

- quire-specification `d104cf9`: FR-061, TC-061, protocol contract, IT-021 and
  the F-owned observation output mapping.
- quire-verification `4a5e196`: FR-007, E02 `TechniqueResult`,
  `src/evidence.rs::findings_for`, `classify`, `result_view`, and TC-014/TC-015.
- QUOIN `spec-object-review` rebuild, cross-reference and semantic-payload
  checks. No implementation or executed-test claim is made.
