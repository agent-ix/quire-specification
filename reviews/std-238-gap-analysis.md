---
id: SR-14812
title: "Gap analysis of the certificate-rejected catalog ACs against TC rows and tests (QSpec PR 200, STD-238)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification@9c60fe7e8347315e5726475ef78d63cc0faae4a5 against merge base 97bfd65e7193d71253f7b84d0ded6218ef8569a5 (PR 200, ticket STD-238); spec/objects/interfaces/FR-331-backend-provider-envelope.md; spec/objects/interfaces/FR-412-probability-certificate.md; spec/objects/interfaces/FR-413-probabilistic-counterexample.md; spec/objects/interfaces/FR-418-zone-certificate.md; spec/test-cases/TC-363-probability-certificate-checker.md; spec/test-cases/TC-364-probabilistic-counterexample-replay.md; spec/test-cases/TC-371-zone-certificate-checker.md; spec/test-cases/TC-374-stochastic-and-probabilistic-timed.md; spec/test-cases/TC-431-backend-provider-wire-vectors.md; proposals/backend-provider-v1/schema.json; proposals/backend-provider-v1/vectors.json; tests/backend_provider_v1.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: reviews
  - target: ix://agent-ix/quire-specification/TC-431
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-413
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-418
    type: reviews
---
# Gap analysis of the certificate-rejected catalog ACs against TC rows and tests

## Summary

Ticket: STD-238. Plan completion: not assessed. This gap analysis maps each
new or changed AC in PR 200 to a TC row and to a traced test.

Gates run on 9c60fe7:

- `cargo test --test checked_package_v2 --test backend_provider_v1` passes,
  117 and 23 tests.
- `quire validate --scope . spec/**/*.md` reports 134 failing documents. The
  touched files' messages are identical to the merge base.

Independent checks, with no finding:

- A scratch Python `jsonschema` 4.26.0 script read the PR's schema and
  vectors and applied each RFC 6902 patch itself. All four
  `certificate-rejected-*` positives are admitted.
  `certificate-rejected-qsl-spellings` has 17 results. Every one of the 77
  adverse vectors on those bases is refused, with an error at or above the
  patched path. 22 of them are BP-24's: 19 on `qsl-spellings` and 3 on
  `loci`.
- Mutation check of `tc_431_certificate_loci_are_the_spellings_qsl_writes`.
  Six schema mutations each made the test fail: `malformed` admitted at a
  proof step, `step/` with any digits, a model state of any domain, a
  `post-differs` without `verdict`, an `Undefined` step verdict (also caught
  by `tc_431_adverse_vectors_are_refused`), and `bad-state` at an entry.
- The 24-rule inventory in the test (`CERTIFICATE_RULES`) equals the schema
  union.

Covered: FR-331-AC-24 (BP-11, BP-24; three traced tests), FR-331-AC-25 (BP-12,
BP-24), FR-412-AC-3 (TC-363 EC-02/03), FR-412-AC-8 (TC-363 EC-07) and
FR-418-AC-2 (TC-371 ZC-02..04).

## Verdict

**Changes needed.** One high finding: an AC with no TC row and no test. Two
medium and one low finding on TC rows.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-331-AC-26 has no TC row and no test. It is the reader's context check: an `inequality-fails` cause on an EN-1 claim, a model-only `successor-missing` for FR-338, and absent or ambiguous request context, each refused. Its verification says "Test (TC-431)", but TC-431's table has no row citing it. The only binding is prose under Expected Results ("BP-13 exercises FR-331-AC-26"), BP-13 is not a table row, and the prose says it is UNRUN. No test carries `FR-331-AC-26` in a trace tag. Fix: add a TC-431 row, or move the AC to the QSL reader's TC where it can run, and say which suite discharges it. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:604; spec/test-cases/TC-431-backend-provider-wire-vectors.md:50 |
| FND-002 | medium | FR-331-AC-27 has no TC-431 row. Tests carry its trace, but TC-431's table never cites it. Expected Results describe BP-14, BP-15, BP-16 and BP-17 as if they were rows, yet the table jumps from BP-12 to BP-18. A matrix reader of TC-431 cannot find AC-27's vectors or the reader-level part marked UNRUN. Fix: add BP-13 to BP-17 as table rows with their inputs, expected results and ACs. | spec/objects/interfaces/FR-331-backend-provider-envelope.md:605; spec/test-cases/TC-431-backend-provider-wire-vectors.md:56 |
| FND-003 | medium | FR-413-AC-6 and FR-413-AC-8 gained new clauses with no new TC row in the TC they cite. AC-6 adds that scheduler and path keys keep the complete product key and refuse digest, index, default or alias substitutes. AC-8 adds digital TS-5 mode and continuation, delay-only recurrence failing admissibility, and IdleObserve aliasing. TC-364 (AC-6) and TC-374 (AC-8) are unchanged and mention none of these. The only binding is the structural test `tc_431_probability_witness_bindings_are_complete` under TC-431. Fix: add the rows to TC-364/TC-374, or cite TC-431 in the verification column for the structural part. | spec/objects/interfaces/FR-413-probabilistic-counterexample.md:193; spec/objects/interfaces/FR-413-probabilistic-counterexample.md:195 |
| FND-004 | low | FR-418's new text adds `initial-missing` at the State locus of the initial discrete state, and `witness-fails` at `{node}` for a time-lock target. FR-418-AC-2 and TC-371 ZC-02..04 cover neither pair. Fix: add both to FR-418-AC-2 and give TC-371 a vector for each. | spec/objects/interfaces/FR-418-zone-certificate.md:89; spec/objects/interfaces/FR-418-zone-certificate.md:113 |
