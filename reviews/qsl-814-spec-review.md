---
id: SR-12073
title: "Spec review of quire-specification PR #269: supplied set, bag and ordered-set values"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@827fe9ecfa53974beac985984a7efc3fcf3add0f; spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md, spec/functional/runtime/FR-181-simulate-finite-executions.md, spec/test-cases/TC-189-collection-kind-algebra.md, spec/objects/foundation/FR-271-diagnostic-code-vocabulary.md, spec/objects/foundation/FR-272-typed-diagnostic-cause.md, proposals/quire-v1/definitions/native-diagnostics.md"
review_set: subset
---
# Spec review of quire-specification PR #269

## Summary

Ticket: QSL-814. One pass of spec-review over `git diff origin/main...HEAD`,
with the contradiction, testability, integrity (ids and edges) and
diagnostic-catalog checks. This is a spec-only PR. Only major issues are
reported.

Compared against: FR-144 (collection-kind table, element admission and keys,
construction), FR-149 equality matrix (numeric row), FR-140 retained
coefficient/scale, FR-181 typed canonical form, FR-208 state-machine context
anchors (`duplicate-member`), FR-330 and FR-413 member-cause usage, FR-274 input
path. Downstream consumer grammar: QSL FR-106 value grammar and FR-106-AC-15,
cited by id only.

Examined:
- FR-144 "Supplied collection values" section and FR-144-AC-16..19 (examined)
- FR-144 canonical-representation paragraph, edited (examined)
- FR-144 frontmatter edge to FR-181 (examined)
- FR-181 new paragraph after the typed canonical form list, and the Set/Bag and OrderedSet bullets (examined)
- TC-189 rows C14 to C21 (examined, each worked by hand)
- FR-271 and FR-272 `cardinality_out_of_bound` and `invalid_runtime_input` rows, native-diagnostics.md same rows (examined)
- FR-144 construction and traversal, FR-149, FR-140, FR-208, FR-330, FR-413, QSL FR-106 (context_only)

Hand checks that passed: C14 (`{"type":"integer","value":"10"}` sorts before
`"9"` because `0x31 < 0x39` after the common prefix, so the written order is
10 then 9); C15 rows 1 and 2; C16; C17 (`/elements/2` naming `/elements/0`
for both the set and the ordered set; bag admitted); C18 (counts 3, 3
occurrences, 0, and 2 at the bound); C19 (bound before element and before
duplicate); C20 row 1. The FR-144 -> FR-181 `references` edge resolves, AC ids
16 to 19 are new and unique, and TC-189 `verifies` FR-144 covers them. The
three catalog files agree with each other. `duplicate-member` for a repeated
entity matches FR-208 precedent. No roadmap or delivery-order text was added.
No QSL text was copied.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Element validation points at "a standalone supplied value of T", which QSpec never defines. The only consumer grammar (QSL FR-106) is externally tagged and already admits `{"sequence":[...]}`. So the new FR-181-form wire mixes two grammars, redefines the admitted Sequence wire, and C15 row 3 and C21 do not follow from any QSpec rule. | spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md:171-192 |
| FND-002 | medium | AC-16 makes a missing or extra member of the collection object `wrong-value-kind`. The same catalog's `missing-member` and `unknown-member` are used for exactly that case in FR-330 and FR-413 (and in QSL FR-106-AC-15). C20 rows 2 and 3 inherit this. | spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md:177-179,271 |
| FND-003 | medium | A Bag or Set member is keyed by FR-149 `=`, but FR-181 encodes the retained form (decimal coefficient/scale, raw text). A supplied `Bag<Decimal>` holding 1.0 and 1.00 is one member with multiplicity 2, and nothing says which encoding is written for each occurrence. So AC-19's "a producer writes a set or bag in FR-181 order" is not deterministic. | spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md:202-206,274 |
| FND-004 | medium | The edited canonical-representation paragraph says "Complete V1 defines no collection byte encoding, sort-key bytes or hash". The next sentence names FR-181, which is a collection byte encoding with a byte sort and a state-key digest. | spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md:61-67 |
| FND-005 | medium | The body orders element validation (step 2) before the duplicate check (step 3). No AC states this, and no TC-189 vector would fail if the order were swapped. Only bound-first is covered (AC-18, C19). | spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md:189-199,272-273 |

## Verdict

**FIX: one high finding (FND-001), plus four mediums.**

FND-001: the spelling of the wire is the planner's choice. The text has to be
consistent whichever spelling is chosen. Two minimal fixes:

- (a) Make FR-181's typed canonical form the supplied-value form for every
  type. Define standalone admission of a typed value in QSpec: a missing `type`
  is `missing-member`, a malformed `value` is `invalid-value`, and so on. Then
  record that QSL FR-106 changes its whole grammar, sequences included.
- (b) Spell the collection wire in the snapshot grammar's element form:
  `{"set":[<value>...]}`, `{"bag":[...]}`, `{"ordered-set":[...]}`, next to
  the existing `{"sequence":[...]}`. Drop the "also the wire value form" claim
  from FR-181. Rewrite C14 to C21 in that grammar.

FND-002: use `missing-member` for an absent `elements` and `unknown-member`
for an extra member such as `count`. Keep `wrong-value-kind` for a tag of
another kind or a non-array `elements`.

FND-003 (the author's hazard). The non-adjacency is a symptom only. Reading is
unaffected, because step 3 compares each element with all earlier ones. That
FR-149-equal values with different retained forms encode differently is an
existing FR-181 property: it already holds for a scalar decimal or a Text
field in a state key. Record it as existing; do not fix it here. What this PR
must add is one sentence, because AC-19 now relies on it: "A set or bag member
is represented by the value of its first admitted occurrence; a bag writes
that one encoding once per multiplicity." With that sentence, the occurrences
of one member are byte-identical and adjacent, and the written wire is
determined by the admitted value. Add one vector: `Bag<Decimal>` supplied
`1.00, 1.0` is written as two `1.00` occurrences.

FND-004: reword to "The FR-144 canonical order is not a byte encoding; the
FR-181 typed canonical form is the one QSpec-owned collection serialization
...".

FND-005: add the element-before-duplicate order to AC-17 or AC-18, and add a
vector such as `Set<Integer>[0,3]` supplied `[1, 1, "x"-valued integer]`. It
is refused `invalid-value` at `/elements/2`, not `duplicate-member`.

Recorded, not blocking. The vectors give input paths relative to the
collection (`/elements/2`). FR-274 asks for the actual input path. A note
saying the paths are relative to the field would remove the ambiguity.
FR-181's OrderedSet bullet admits "no duplicate encoded element", which is
weaker than FR-144's FR-149 rule. The two agree, because FR-144 refuses first.

## New findings (disposition pass 1)

Reviewed at 3cc54e1c5ddf72dd1aca19a70b68359ed3d33f03 (fix commit 3cc54e1). Scope: the new FR-181 "Canonical element encoding" subsection, the FR-144 cause table, the uuid/timestamp/string sentence, AC-16 and AC-19..21, and TC-189 C15, C20, C22 and C23. No existing FR-181 or state-key vector holds a decimal or normalizing-text Set/Bag element, so normalizing those elements breaks no recorded vector.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | high | FR-144 now says `uuid`, `timestamp` and `string` "have no FR-144 canonical key either and are not admitted collection element types". FR-149 gives `UUID` and `Timestamp` a total canonical key (the canonical payload string) that "extends FR-144's type-owned key invariant", and FR-149-AC-14 asserts equal keys. FR-144's own rule admits every `=`-admitting element type. So `Set<UUID>` is admitted by FR-144 and FR-149 and denied by this sentence, and no refusal is named for a supplied collection of such elements. | spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md:193-195 |
| FND-007 | medium | Row 1 of the cause table maps "the value is not a JSON object" to `missing-member`. A JSON string or number where an object is required is a wrong JSON kind, which row 5 of the same table and FR-330 map to `wrong-value-kind`. | spec/functional/type-model/FR-144-preserve-complete-collection-algebra.md:186 |
| FND-008 | medium | FR-181 "A reader accepts any encoding of a value and normalizes it." has no scope. Read as written, it normalizes every decimal and normalizing-profile text that is read, including a scalar field in a state key. That contradicts the sentence above it ("applies only to the element of a set or bag, so the retained representation elsewhere is unchanged"). FR-144's version is scoped to an element. | spec/functional/runtime/FR-181-simulate-finite-executions.md:135 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 3cc54e1 |
| FND-002 | fixed | 3cc54e1 |
| FND-003 | fixed | 3cc54e1 |
| FND-004 | fixed | 3cc54e1 |
| FND-005 | fixed | 3cc54e1 |
| FND-006 | open | awaiting planner choice between (a) and (b); not yet fixed |
| FND-007 | fixed | cause table row 1 split: not a JSON object is `wrong-value-kind`, an object with no `type` member is `missing-member` |
| FND-008 | fixed | FR-181 sentence scoped to "a set or bag element" |

Round 1 verdict: **FIX** on FND-006. Minimal fixes:

- FND-006, two options. (a) Drop "they have no FR-144 canonical key either". State that FR-181 has no wire form for these kinds, so a supplied collection whose element type is one of them is refused with a named cause, while construction is unchanged. (b) Add FR-181 forms `{"type":"uuid"|"timestamp","value":"<canonical payload>"}`, which are canonical as written.
- FND-007: change row 1 to "the value is not a JSON object → `wrong-value-kind`; it has no `type` member → `missing-member`".
- FND-008: scope the sentence: "A reader accepts any encoding of a set or bag element and normalizes it."

Recorded, not blocking: the C23 written outputs show decimal members in the order `type, coefficient, scale`. FR-181 is JCS, which orders members `coefficient, scale, type`. This does not change the element order here, but a byte-exact comparison of the vector would fail.
