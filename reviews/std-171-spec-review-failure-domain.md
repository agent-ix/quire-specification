---
id: SR-12823
title: "Failure-domain review of PR #279: state-graph graph-predicate production"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-specification@5193b7d306a98ac4eac8950974af9c188db1c899; PR #279 diff against origin/main 1da546f: TC-330 SG-R09..SG-R13, FR-390 Admission and FR-390-AC-2, shared-grammar.md state-graph fragment rows; context: quire-spec-language FR-165-AC-5"
review_set: subset
---
# Failure-domain review of PR #279 (STD-171)

## Summary

Ticket: STD-171. This review looked at how each new refusal vector can fail
for a reason other than the one it names: name resolution, refusal order and
positions with no grammar slot. The positive structure is sound. Each
operator position has a vector, and the locus "at the operator" is
well-defined because every temporal operator is a single keyword token. The
issues are unbound names in the vectors and one claim form that cannot carry
fairness at all.

## Verdict

FND-001 is a medium oracle defect, to fix in this PR. FND-002 is low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | SG-R10, SG-R11 and SG-R12 read names that nothing binds, so their single expected refusal depends on a refusal order the spec does not state. `possible N using v { not eventually[0,5] c.n = 2 }`, `possible P using v { (eventually c.n = 2) }`, `possible U using v { a until b }`, `always possible Rec using v from (always c.ok) { c.done }`, `unique path One using v from (c.a) to (eventually c.done);` and SG-R12's `c.done` have no `over` binding and no declared `c`, `a` or `b`. QSL FR-165-AC-5 refuses an unbound name `missing-name` at the name. A checker that resolves names before it scans for operators reports `missing-name` at `c` (or `a`) and fails the "`expression-form` at `eventually`/`until`/`always`" oracle. A checker that scans first passes. The same holds for the fairness target `M::Cfg::step` when no model alias `M` is declared. Fix: bind every name, as SG-R09 does (`over (c: C::Cfg)`, with `a`/`b` as `c.a`/`c.b` and a declared `C::Cfg::step`), or state in FR-390 that the temporal-operator and fairness refusals take precedence over name resolution. | spec/test-cases/TC-330-state-graph-claim-syntax-and-routing.md:36-38; spec/functional/analysis/FR-390-analyze-state-graph-claims.md:116-122 |
| FND-002 | low | A `unique path` clause has no `graph-body`. It ends in `';'`, so it has no slot for fairness. A fairness constraint on it is a syntax error, not the FR-390 admission refusal "When a state-graph claim carries a fairness constraint" (ADR-022 GM-7), which is therefore unreachable for one of the three forms. This is a defensible choice (author point 1), but it is unstated, and FR-390-AC-2 says "a fairness constraint at the start of a claim body" without noting that `unique path` has none. Fix: one sentence in the recognition rules. | proposals/quire-v1/shared-grammar.md:582-584; spec/functional/analysis/FR-390-analyze-state-graph-claims.md:120-122 |

## Dispositions

Round 1, reviewed at fad2cd29132c763e218b918230ed71c48a210a0c. SG-R10 to
SG-R13 now bind `c` through `over (c: C::Cfg)`, read `c.a`/`c.b` instead of
bare `a`/`b`, and name fairness on `C::Cfg::step`. The vectors' only
remaining over-less, unbound source is the third SG-R15 vector, and that one
is a syntax error before name resolution, so its oracle does not depend on
refusal order.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
