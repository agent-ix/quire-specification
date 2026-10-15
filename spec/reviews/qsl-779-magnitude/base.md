---
id: SR-9129
title: "base review of the supplied numeric magnitude bound B in FR-323 and TC-278 (PR 255)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@2a04a4721; spec/objects/interfaces/FR-323-native-runtime-envelope.md; spec/test-cases/TC-278-replay-envelope-members.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: references
  - target: ix://agent-ix/quire-specification/TC-278
    type: references
---

# Base review of the supplied numeric magnitude bound (quire-specification PR 255)

## Summary

Ticket: QSL-779. One base pass over `origin/main...HEAD` at tip 2a04a4721
(two commits: 346de15 and the ruling commit 2a04a47). The diff adds the
optional `limits.supplied.numeric_magnitude_bits` (B) member, the "Numeric
magnitude bound" subsection (rules 1-7), the `operand` member of the
`limit` cause row, FR-323-AC-15 and TC-278 RE-23..RE-28. Integrity,
contradiction and identity checks are folded into this base pass. The
failure-domain sub-analysis is SR-9130.

Oracles were read on each repo's `origin/main` without switching any
checkout:

- QSL FR-277, TC-906 and TC-830.
- QSV FR-109, and QSV `src/declaration/membership.rs` for where the
  `NumericDomain` stop fires.
- quire-exact at f8e0577 (`src/integer.rs`, `src/numeric.rs`).
- QSpec `proposals/quire-v1/definitions/value-accounting.md`.

The author's citation correction holds. QSpec FR-380-AC-6 is the
refinement-subject row. The premise homes are QSL FR-277 (AC-6 and the
"Selected independently bounded helper-input contract") and QSV FR-109
AC-7.

The two rulings in 2a04a47 were checked against the oracles:

- Zero has bit length one. The substance matches
  `value-accounting.md:56-57` ("`bits(x)` ... zero is one") and quire-exact
  `src/integer.rs:178-185` (`magnitude_bits`, `.max(1)`). The cited source,
  "QSL FR-380", does not exist (FND-004).
- Charge first, then the helper. This matches QSV FR-109 Behavior 2 and
  QSL FR-277's poll-and-charge-before-each-step rule. It also matches QSV
  `membership.rs`, which raises the `Event` boundary before the
  `RequiresQualification` boundary, and whose numeric-domain test asserts
  `budget.spent() == 1` at the stop.

Caller rules applied: only wrong/contradictory requirements, wrong refusal
codes, unbound ACs, unproducible or wrong vectors, QSL/QSV conflicts and
an input-unbounded numeric helper behind nominal event charges block.

Examined, clean:

- Identity (design choice 3) against FR-323's caller-API normalization
  paragraph, AC-10 and RE-14. A/H normalize to 16777216 and B has no default;
  RE-14 holds A/H explicit and RE-26 varies only B. No contradiction.
- Malformed B and B in `stage_limits` (rule 3, RE-28). Consistent with the
  closed-object and unknown-stage-setting rules.
- Bit-length math at the tip (zero is one). 2^64-1 has 64 bits and is
  admitted at B=64. 2^64 has 65 bits and refuses at B=64. B=1 with value 0
  is admitted over endpoints of bit length 1. B=0 with value 0 refuses
  `operand: value` with `next_charge`/`counter` 1, because value is checked
  first. RE-25's zero cases agree. QSL integer bounds are i128 since QSL-648,
  so the 2^64 and -2^64 endpoints can be written.
- AC-15 binding. Every clause has a vector: admit at B (RE-23/25), B+1
  cause fields, order and B=0 (RE-24/25), the charged event and `consumed`
  (RE-24/25), identity (RE-26), malformed and stage_limits (RE-28), absent
  B (RE-27).
- Comparison cost (judge item a, value-visit path only). QSV FR-109's
  helper table states that these are "cost premises, not extra logical-work
  charges". One charged `value-visit` unit plus at most 2 or 4 comparisons
  of at most B bits matches FR-109's Integer interval and Rational domain
  rows. No B-proportional unit charge is required. The type-comparison path
  is not covered; see SR-9130.
- Status PROPOSED (item f). Consistent with AC-10..14 and the TC-278
  status. Acceptable.

Validation: `quire validate` (quire 0.36.2, engine 0.50.2) on FR-323
reports a missing value_object `id`, `semantic.record-invalid` (fields
required) and the line-21 legacy-properties warning. The same diagnostics
reproduce on `origin/main`'s FR-323 copied to a scratch path with the same
scope, so they are pre-existing (item g). TC-278 validates clean.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | Absent B is required to produce a runtime `UnsupportedHelper { family: NumericDomain }` stop (rule 4, AC-15, RE-27). Merged text says a missing operand-bound premise is a qualification blocker, "not a runtime invalid/limit/storage/unsupported result": FR-323 Qualification prerequisites, FR-323-AC-13 and TC-278 RE-21, QSL FR-277-AC-6 and its helper-input contract, QSV FR-109 Behavior 5. Rule 4 also gives the stop no envelope disposition (not limit, input or fault; mapping left as a prerequisite). FR-323-AC-1 requires exactly one disposition per item, so RE-27's expected result cannot be observed through the real envelope result producer TC-278 mandates. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:192-200, :369-370, :402, :414, :416; spec/test-cases/TC-278-replay-envelope-members.md:87, :93 | wrong-requirement |
| FND-002 | high | Conflict with the paired QSL text. QSL FR-277 requires every bound a lifecycle operation enforces to be a caller limits field "with a published default" (Description, Inputs, Behavior). It states that the shared request's `limits.supplied` carries "these two fields" (A/H), and QSL's typed supplied-value limits value has only A/H. FR-323 rule 1 makes B a defaultless enforced bound, and QSL has no carrier or setting name for it. The rationale's reading of FR-277 is wrong: FR-277's "select any default numeric/type bound" sentence says the cost premises select no default; it does not permit a defaultless field. Without a paired QSL amendment, a QSL caller can never state B. TC-906's mandatory positive replays over `Int[0, 9]` fields (steps 1, 4) and `r(v: Rational)` (step 4) then stay at NumericDomain forever. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:175-179, :239-250 | wrong-requirement |
| FND-003 | medium | Recorded, not blocking. The over-B `limit` cause sets `limit_kind: numeric_magnitude_bits`, but `consumed` is the A/H prefix, which now includes the stopping event. `counter` is the operand bit length, not consumed + next_charge. Both depart from the meanings of `consumed` ("successful prefix of the owning counter ... never includes a denied step") and `counter` ("checked attempted count") in the cause row and in QSL FR-277's pre-call limit record. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:216-233, :275-276, :282 | wrong-requirement |
| FND-004 | low | Recorded, not blocking. Rule 5 cites the zero-is-one rule to "QSL FR-380", which does not exist. The real sources are value-accounting.md:56-57 and quire-exact `Integer::magnitude_bits`. RE-23's spend oracle compares against a different fixture. The Rational operand names read as rational endpoints, but QSV `RationalDomain` is a numerator interval plus a denominator interval. `spec/complete-v1/tests.md` (TM-009) lists FR-323 only to AC-9. That gap predates this PR (AC-10..14 were missing after QSL-681), and the matrix is hand-maintained traceability, the tracking antipattern. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:205-210; spec/test-cases/TC-278-replay-envelope-members.md:89; spec/complete-v1/tests.md:1062-1070 | correct-requirement-no-evidence |

## Verdict

CHANGES REQUESTED: two blocking findings. FND-001: the runtime outcome for
an absent B contradicts the merged "missing premise is not a runtime
unsupported result" rule and has no envelope disposition. One way to fix it
is to refuse a request whose supplied type needs a numeric comparison and
states no B before consumption, under the existing "lacking an effective
applicable ceiling SHALL NOT be admitted" rule. The other is to state an
explicit carve-out in AC-13, QSL FR-277 and QSV FR-109 together with a
disposition. FND-002: B as a defaultless field conflicts with QSL FR-277
and has no QSL carrier, so a paired QSL FR-277 amendment (field, setting
name, default policy) must land with this one. Failure-domain finding is in
SR-9130. Both rulings match their oracles in substance. Identity, malformed
handling, bit math, AC binding and the value-visit cost model are clean.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-005 | high | Rule 5's heading and closing sentence say the value-in-domain part is "implemented-qualified by quire-exact f8e05771...". FR-323's own Status says AC-10..16 are "UNQUALIFIED. No implementation, typed native mapping, helper proof or acceptance execution is claimed". FR-323-AC-13 and QSL FR-277-AC-6 qualify a supplied path only with the consumer caller and the reviewed typed mapping. Only the kernel helper is landed (quire-exact `src/numeric_membership.rs` `try_contains_bounded`). QSV `origin/main` has no caller of it, and the QSV caller is quire-exact FR-380-AC-6 / IR-792, which is still pending. The sentence also adds a commit SHA pin, which this repo's CLAUDE.md bars. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:199, :214-215, :470-472 | wrong-requirement |
| FND-006 | medium | Recorded, not blocking. The published B default of 4096 sits against quire-exact FR-380 Behavior 6 ("a convenient default ... is not qualification") and FR-380-AC-6 ("no numeric default is added"). Read as written, the default resolved at the caller API into the request's caller-owned B is consistent with FR-380, but FR-380's wording should be aligned in the paired amendments. | spec/objects/interfaces/FR-323-native-runtime-envelope.md:178-183 | wrong-requirement |
| FND-007 | low | Recorded, not blocking. "Both members SHALL be exact unsigned 64-bit ceilings" is stale now that there are three members. `consumed` 0 is listed among the limit cause fields (cause row, AC-15), but it is a context member. RE-29's "at exactly B all admit" needs expected and actual endpoints widened as a pair, or the type comparison mismatches. RE-27's 4096-bit admit needs a fixture whose endpoints are at least 4096 bits (QSL's current integer bounds are i128). | spec/objects/interfaces/FR-323-native-runtime-envelope.md:127, :305, :440; spec/test-cases/TC-278-replay-envelope-members.md:93, :95 | correct-requirement-no-evidence |

## Dispositions

Round 1, reviewed at agent-ix/quire-specification@582b2e3b4 (commits 52f89db and 582b2e3 on top of 2a04a47). Oracles: QSL FR-277 and QSV FR-109 on origin/main, quire-exact FR-380 and `src/numeric_membership.rs` on origin/main.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 582b2e3 |
| FND-002 | deferred | Fixed on the QSpec side in 582b2e3: B now has a published default of 4096, so the FR-277 published-default rule holds. The remaining gaps are FR-277's "these two fields" sentence and the missing QSL carrier and setting for B. They close with the planner's paired QSL FR-277 amendment under QSL-779, which the dispatcher ruled out of scope for this PR. |
| FND-003 | fixed | 582b2e3 |
| FND-004 | deferred | Fixed in 52f89db and 582b2e3: the FR-380 citation now names quire-exact FR-380, RE-23 uses the same fixture, and the Rational operand names match the numerator/denominator intervals. TM-009 `spec/complete-v1/tests.md` still stops at AC-9. That gap predates this PR (AC-10..14 since QSL-681), and the hand-maintained matrix is a tracking antipattern to delete separately. |
