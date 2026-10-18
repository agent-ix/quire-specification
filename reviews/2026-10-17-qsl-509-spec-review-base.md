---
id: SR-12103
title: "Base review of QSpec PR #272: open-interval until and since as the limit of the closed form"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification@f3ddff979fadc2ef54a4a36fe14d18e91801e615; PR #272 diff against origin/main: spec/functional/temporal/FR-367-evaluate-interval-operators-under-infinite-trace.md, spec/functional/temporal/FR-416-evaluate-timed-temporal-claims.md, spec/test-cases/TC-298-interval-operators-under-infinite-trace.md; context FR-090, FR-091, FR-092, FR-254, FR-255, proposals/quire-v1/definitions/temporal-common.md, proposals/quire-v1/definitions/temporal-timestamped-window.md"
review_set: subset
---
# Base review of QSpec PR #272 (QSL-509)

## Summary

Ticket: QSL-509. The PR makes FR-367's `until[a,*]` and `since[a,*]` read the
closed form of FR-091 and FR-092 with no upper bound, adds discriminating
cases to FR-367-AC-7 and TC-298 step 8, and adds one cross-reference sentence
each to FR-367 and FR-416 that says which profiles read intervals as position
offsets and which read them over time stamps. Reviewed at the final head
f3ddff9 (three commits).

Checked and clean:

- Limit derivation. FR-091 `p until[a,b] q` is: some `d` in `[a,b]` with `q` at
  `i+d` and `p` at `i+e` for every `e` in `[a,d)`. As `b` grows without bound
  the witness set unions over every `d >= a`. With `j = i+d` this gives `q` at
  some `j >= i+a` and `p` at every `k` with `i+a <= k < j`, which is the
  amended sentence. FR-092 `since`: `q` at `i-d` and `p` at `i-e` for `e` in
  `[a,d)`, so `k = i-e` ranges over `(i-d, i-a]`. With `j = i-d` this gives
  `j <= i-a` and `j < k <= i-a`, which is the amended sentence. At `a = 0` both
  sentences reduce to the old reading and to FR-161's unbounded operators.
- Duals. FR-091 and FR-092 (quoted again in FR-254) define closed release and
  triggered as the Boolean dual through `not p until/since not q`. Negation
  turns the monotone union over `b` into the intersection over `b`. So
  `not (not p until[a,*] not q)` equals the limit of the closed release as
  `b` grows, and the same holds for triggered. They are consistent.
- AC-7 and TC-298 step 8 examples, computed by hand. `p until[2,*] q` at 0,
  with `q` only at 3 and `p` only at 2: the new reading needs `p` on `{2}`, so
  it is true. The old reading needs `p` on `{0,1,2}`, so it is false. The case
  discriminates. With `p` false at 2, both readings give false. That case does
  not discriminate, but it pins that position `i+a` is still required.
  `p since[2,*] q` at 4, with `q` only at 1 and `p` only at 2: the new reading
  needs `p` on `{2}`, so it is true. The old reading needs `p` on `{2,3,4}`, so
  it is false. The case discriminates. The false variant is false under both
  readings.
- No other statement of the old left-operand range exists in spec/ or
  proposals/. FR-254, FR-255 and proposals temporal-common.md all state the
  lower-bound convention.
- FR-416 timed/v1 uses the real-time MTL reading (`i <= k < j`). The planner
  ruled this deliberate, and it is not a finding. FR-091's own text allows a
  different convention under a different identity.
- `quire validate --scope . 'spec/**/*.md'` gives the same output on
  origin/main and on f3ddff9: 134 documents fail, all pre-existing, and none
  of them is a touched file.

## Verdict

Changes needed: one high finding and one medium finding, both in the
cross-reference sentences added after the first commit. The limit semantics
commit (FR-367, AC-7, TC-298) is correct as written.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-416:61 now says FR-416's pointwise reading applies to `quire.temporal.timestamped-event.finite-window/v1`. That reading requires `p` at every `k` with `i <= k < j`, and its TT forms, model-time clock and timed-subject inputs come with it. FR-091:70-77 gives the timestamped finite-window profile its own meaning, and in it `until` "requires its left operand at every eligible earlier instant from the lower bound to the witness". That is the lower-bound convention. The profile's definition (temporal-timestamped-window.md, built on bounded-facet/v1 temporal-common.md:41: "Until/since require the left operand only between the interval's lower bound and the witness") says the same. FR-416's Description scopes FR-416 to timed/v1, and FR-090-AC-8 makes the timestamped-event profile settle `unsupported` over a model subject. The new sentence contradicts FR-091 for the timestamped-event profile. Under the current text, that profile measures interval membership by time-stamp distance, as FR-416 does, but keeps the lower-bound left-operand range of FR-091, which FR-416 does not use. So it fits neither of the two buckets. Ruling needed: keep FR-091's lower-bound convention for timestamped-event, in which case FR-416:61 names timed/v1 only and the cross-reference points timestamped-event to FR-091/FR-092's timestamped finite-window rule, or deliberately move timestamped-event to the MTL convention, which means changing FR-091, FR-092 and the profile definition under a new profile identity per FR-090. | spec/functional/temporal/FR-416-evaluate-timed-temporal-claims.md:61; spec/functional/temporal/FR-091-evaluate-bounded-future.md:70-77; proposals/quire-v1/definitions/temporal-common.md:41 |
| FND-002 | medium | The two cross-reference sentences do not partition FR-090's profile list in the same way. FR-367:70 names infinite-trace, event-position and fixed-sample as offset profiles and timed/v1 as the time-stamp profile, and never mentions `quire.temporal.timestamped-event.finite-window/v1`. FR-416:61 places that profile with timed/v1. Each FR-090 profile id should appear in exactly one bucket, and the same bucket in both sentences. After the FND-001 ruling, both sentences should name the timestamped-event profile in the same place. | spec/functional/temporal/FR-367-evaluate-interval-operators-under-infinite-trace.md:70; spec/functional/temporal/FR-090-select-temporal-profile-and-clock.md:44-50 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | Fixed | FR-416 names `quire.temporal.timed/v1` only and points `quire.temporal.timestamped-event.finite-window/v1` at the timestamped rule of FR-091 and FR-092. |
| FND-002 | Fixed | All five FR-090 profile ids sit in one bucket and both sentences agree: offset reading for infinite-trace, event-position and fixed-sample; FR-416 for timed; the FR-091 and FR-092 timestamped rule for timestamped-event. |
