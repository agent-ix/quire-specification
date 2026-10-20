---
id: SR-13531
title: "Spec review of quire-specification PR #299: FR-383 locations affect directionally (QSL-564 spec follow-up)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@0e822a6f1dc32a66bc7b094adc0a27bc1a301220; PR #299, git diff refs/remotes/origin/main...0e822a6: spec/functional/analysis/FR-383-explore-under-state-space-reduction.md, spec/test-cases/TC-323-partial-order-reduction-vectors.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-383
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-323
    type: reviews
---
# Spec review of quire-specification PR #299

## Summary

Ticket: QSL-564. This is the QSpec half of the QSL-564 spec follow-up. It
pairs with QSL PR #1096 (SR-13530). The diff is 2 files, +35/-15.

**"Locations"** now defines which cells a read location covers and which cells
a written location affects:

- a written `Membership{P, k}` affects the membership cell and every field
  cell of `k`;
- a read `Membership{P, k}` covers only the membership cell;
- a written field affects no membership cell;
- two written locations meet when either affects the other.

This has the same meaning as QSL ADR-021 POR-4 and FR-155 "Meeting". It is
also the relation that PR #1094's `Location::affects` implements and that the
SR-13470 oracle found sound: over about 22,000 random models, no verdict
differed between the full and the reduced search.

"Independence and visibility", the ample-set closure, FR-383-AC-10 and AC-13
now use "affects" for write/read pairs. New FR-383-AC-17 states the rule, and
TC-323 verifies it.

`AnyField{T, f}` now covers objects of `T` or of a type conforming to `T`. The
old text read "related by conformance". The new wording is narrower, but it
is exact and still sound, so it is not a finding.

**Gates:**

- `quire validate --scope . "spec/**/*.md"`: 134 documents fail. Main fails
  on exactly the same 330 diagnostic lines, so none are new. FR-383 and
  TC-323 have no diagnostics.
- `cargo test`: green (111, 22, 11, 7, 4 and 3 passed).
- No FR was added, so the index is unchanged.

## Verdict

Changes requested: two medium findings. The rule itself is correct.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Part of the new rationale in "Independence and visibility" is false, and part records the review rather than the system. "The symmetric one ... leaves no ample set smaller than the enabled set for the subjects of FR-383-AC-1 to AC-5" fails three ways. AC-1 and AC-2 are symmetry vectors. AC-4's counters reduce from 27 to 7 states under the symmetric relation too: `bump(a)` and `bump(b)` touch different objects, and the QSL tests stayed green with `affects` replaced by `meets`. Only AC-5's `Gate`, with its same-object operations, has no reduction under the symmetric meet. "The directional relation agrees with unreduced exploration on a 22,000-model comparison" is reviewer evidence that nobody can reproduce from this repository. Keep the reason (a field write does not change whether an object exists) and drop the evidence count. | spec/functional/analysis/FR-383-explore-under-state-space-reduction.md:206-212 |
| FND-002 | medium | FR-383-AC-17 cites "the reduced run of the counter subject (FR-383-AC-1)", but FR-383-AC-1 is the sort canonicaliser (symmetry). The partial-order counter subject is FR-383-AC-4. That clause also cannot fail under the old symmetric relation, because the counters reduce under either relation. The clause that tells the two apart is AC-5's `Gate` (`toggle` invisible under `fair weak go`, `A = {toggle}`). Cite AC-4, and add the `Gate` expectation, or a pointer to AC-5, to AC-17 and to TC-323's new paragraph. | spec/functional/analysis/FR-383-explore-under-state-space-reduction.md:326; spec/test-cases/TC-323-partial-order-reduction-vectors.md:174 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | Four TC-323 paragraphs still use "meets" for a write against a read location: case 7's `AnyMembership{jobs}` "which meets the membership of `j`", `unlink(j)`'s `Field{j, peer}` "which meets it", and `set`'s `Field{a, v}` "which meets it" (twice). Each pair is one where the two relations agree, so the meaning holds. But FR-383 now says "affects" for write/read pairs. Case 7 also says "both storing 4 states" without saying which states are counted. | spec/test-cases/TC-323-partial-order-reduction-vectors.md:138-139; spec/test-cases/TC-323-partial-order-reduction-vectors.md:146; spec/test-cases/TC-323-partial-order-reduction-vectors.md:158; spec/test-cases/TC-323-partial-order-reduction-vectors.md:167 |

## Dispositions

Round 1, reviewed at `9e31dc272adcf54b4a1f9211a85467a683f2b39c`.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | 9e31dc2: the rationale names only FR-383-AC-5's same-object Gate operations, and the 22,000-model count is gone. |
| FND-002 | fixed | 9e31dc2: FR-383-AC-17 and TC-323 now assert AC-5's Gate case (`toggle` invisible, `A = {toggle}`, equal verdicts) in place of the AC-1 counter citation. |

Gates at 9e31dc2:

- `quire validate --scope . "spec/**/*.md"`: 134 documents fail, with
  diagnostics identical to main's (330 lines), so none are new. FR-383 and
  TC-323 have no diagnostics.
- `cargo test`: green (111, 22, 11, 7, 4 and 3 passed).
- The committed copy of this file passes `quire validate`.

Round 2, reviewed at `f8ac00022fe3ab32002e79f5b8a0d3eaf70d20bb`.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-003 | fixed | f8ac00022: the four TC-323 write/read pairs now say "affects". Every count in the file now reads "N states (model states stored by the run's search)". No "meets" for a write/read pair remains in FR-383 or TC-323. TC-323:174's heading "the directional meet" is a label for the rule, not a relation claim. One caveat, not reopened: these counts (4, 5, 6) match a complete model-level search. A safety run that stops at the first violation stores fewer states; QSL's `check_model` stores 2 for the deleting-the-receiver vector. QSL #1096 words the same count as "in the model-level reduced search", which is the more exact reading. |

Gates at f8ac00022: `quire validate --scope . "spec/**/*.md"` reports 134
failing documents, the same diagnostics as main, and none on FR-383 or
TC-323.
