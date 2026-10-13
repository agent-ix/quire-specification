---
id: SR-8713
title: "integrity review of PR #244 phase-2 qualify list"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-specification@2989920af9570f8858b8dee479aa8b35729005cc; proposals/quire-v1/definitions/model-complete.md; spec/functional/type-model/FR-150-preserve-model-normalization-provenance.md; spec/test-cases/TC-195-model-normalization-provenance.md"
review_set: subset
---

## Summary

Ticket: QSL-743. Consistency check of the changed phase-2 row and new paragraph
against FR-150, the other model-complete.md sections and the TC-195 steps.
Three inconsistencies were found: FR-150's phase-2 list was not updated, the
model-complete.md member statement and Declaration kinds table do not admit a
variant case as a member, and type-owned invariant clauses get qualify facts in
code while the spec says they get none.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-150 "Normalization derivation" phase (2) list still names only object types, interfaces, fields, relationships, operations, parts, ports, connections and allocations, so the requirement TC-195 verifies disagrees with the updated model-complete.md phase-2 row (no record, event or variant type, no variant case) | FR-150:85-90; model-complete.md:287 |
| FND-002 | medium | model-complete.md:105-106 says "Fields, relationships, operations and clauses are the only members", and the Declaration kinds table has no variant-case row, yet the new phase-2 row makes a variant case an effective member owned by its variant type | model-complete.md:105-106; model-complete.md:129-141; model-complete.md:287 |
| FND-003 | medium | The built normalizer qualifies each type-owned `quire` invariant clause as a member and inherits it (`sorted_direct_clauses` in phase-2 direct members), but the phase-2 row names no clause and model-complete.md:207-211 / FR-150 say a retained clause gets no invented qualify fact; the new paragraph's "their fields are members" repeats the omission | model-complete.md:287; model-complete.md:207-211; model-complete.md:293-298 |

## Verdict

These findings do not block the variant, record and event content, which is
correct, but FND-001 leaves two normative texts disagreeing. FND-001 and
FND-002 are fixes inside this PR. FND-003 is the coder's flagged open item: the
spec and code disagree, and someone must decide which one changes.
Consistent and clean: the new paragraph agrees with model-complete.md:163 (no
object universe outside processes), the Supertypes row (record and event types
generalize their own meaning) and FR-056's "Effective identity of a variant
type" (case keyed by its case node, never the `variant` export key).
